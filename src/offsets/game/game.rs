use std::collections::HashMap;
use std::ffi::CStr;
use std::fs;

use anyhow::{anyhow, Result};

use indexmap::IndexMap;

use crate::config::CONFIG;

#[cfg(target_os = "linux")]
use super::elf_file::ElfFile;
use super::globals;
use super::matcher::HexMatcher;
use super::matcher::Matcher;
#[cfg(target_os = "windows")]
use super::pe_file::PeFile;

type Offsets = IndexMap<String, usize>;

const PLATFORM_ITCH: &str = "itch";
const PLATFORM_STEAM: &str = "steam";

const OS_WINDOWS: &str = "windows";
const OS_LINUX: &str = "linux";

pub struct Game {
  #[allow(unused)]
  path: String,
  data: Vec<u8>,
  version: String,
  platform: &'static str,
  globals: Offsets,
  functions: Offsets,
}

impl Game {
  pub fn new<P: ToString>(path: &P) -> Result<Self> {
    let path = path.to_string();
    let data = fs::read(&path)?;

    #[cfg(target_os = "windows")]
    let pe = PeFile::new(&data)?;

    let mut game = Game {
      path,
      data,
      version: String::new(),
      platform: PLATFORM_ITCH,
      globals: Offsets::new(),
      functions: Offsets::new(),
    };
    let mut matcher = game.matcher()?;
    game.run_matcher(&mut matcher);

    for key in matcher.keys() {
      let results = matcher.get_results(&key);

      if key == "df_version" {
        let data_len = matcher.data_len(&key);
        let mut max = 0f32;
        let mut version = "unknown".to_string();
        for result in results {
          let offset = result as usize + 1;
          let length = data_len - 2;
          let verstr = String::from_utf8_lossy(&game.data[offset..offset + length]).into_owned();
          let vernum: f32 = verstr.parse().unwrap_or(0f32);
          if vernum > max {
            max = vernum;
            version = verstr;
          }
        }
        game.version = version.clone();

        continue;
      }

      if key == "steam_api_init" {
        if results.is_empty() {
          game.platform = PLATFORM_ITCH;
        } else {
          game.platform = PLATFORM_STEAM;
        }

        continue;
      }

      if key == "global_table_magic" {
        if results.len() != 1 {
          return Err(anyhow!("Expected one result for global table, found {}", results.len()));
        }

        let global_table_offset = results[0] as usize + matcher.data_len(&key);
        let globals_addresses = globals::parse_globals(&game.data, global_table_offset);

        let mut globals = HashMap::new();
        for (symbol_address, global_address) in globals_addresses {
          #[cfg(target_os = "windows")]
          let (symbol_address, global_address) = {
            let symbol_address = pe
              .address_to_offset((symbol_address - 0x140000000) as u32)
              .ok_or(anyhow!("Failed to locate global symbol"))?;
            (symbol_address, global_address - 0x140000000)
          };

          #[cfg(target_os = "linux")]
          let (symbol_address, global_address) = (symbol_address - 0x00400000, global_address);

          let symbol = CStr::from_bytes_until_nul(&game.data[symbol_address as usize..])?.to_str()?;
          globals.insert(symbol.to_string(), global_address);
        }

        for global_symbol in game.global_symbols() {
          let offset = globals.get(&global_symbol).ok_or(anyhow!("Global symbol not found: {}", global_symbol))?;
          game.globals.insert(global_symbol, *offset);
        }

        continue;
      }

      if results.len() != 1 {
        return Err(anyhow!(
          "Expected one result for key '{}', found {}",
          key,
          results.len()
        ));
      }

      #[cfg(target_os = "windows")]
      {
        let result =
          pe.offset_to_address(results[0]).ok_or(anyhow!("Failed to convert file offset to virtual address"))?;
        game.functions.insert(key.clone(), result as usize);
      }
    }

    #[cfg(target_os = "linux")]
    game.parse_gsrc_lib()?;

    Ok(game)
  }

  pub fn version(&self) -> &str {
    &self.version
  }

  pub fn os(&self) -> &'static str {
    if cfg!(target_os = "windows") {
      OS_WINDOWS
    } else {
      OS_LINUX
    }
  }

  pub fn platform(&self) -> &'static str {
    &self.platform
  }

  pub fn globals(&self) -> &Offsets {
    &self.globals
  }

  pub fn functions(&self) -> &Offsets {
    &self.functions
  }

  fn run_matcher(&self, matcher: &mut dyn Matcher) {
    self.data.windows(matcher.len()).enumerate().for_each(|(offset, window)| matcher.matches(offset, window));
  }

  fn matcher(&self) -> Result<HexMatcher> {
    let mut matcher = HexMatcher::new();

    matcher.add_pattern("df_version", "00 ## ## 2e ## ## 00")?;
    matcher.add_pattern("steam_api_init", "53 74 65 61 6d 41 50 49 5f 49 6e 69 74")?;
    matcher.add_pattern(
      "global_table_magic",
      "78 56 34 12 78 56 34 12 21 43 65 87 21 43 65 87 ef cd ab 89 ef cd ab 89",
    )?;
    #[cfg(target_os = "windows")]
    {
      matcher.add_pattern("addst", CONFIG.offsets.addst_pattern.as_str())?;
      matcher.add_pattern("addst_flag", CONFIG.offsets.addst_flag_pattern.as_str())?;
      matcher.add_pattern("top_addst", CONFIG.offsets.top_addst_pattern.as_str())?;
      matcher.add_pattern("draw_nineslice", CONFIG.offsets.draw_nineslice_pattern.as_str())?;
      matcher.add_pattern(
        "draw_horizontal_nineslice",
        CONFIG.offsets.draw_horizontal_nineslice_pattern.as_str(),
      )?;
      matcher.add_pattern("gps_allocate", CONFIG.offsets.gps_allocate_pattern.as_str())?;
      matcher.add_pattern("update_all", CONFIG.offsets.update_all_pattern.as_str())?;
      matcher.add_pattern("update_tile", CONFIG.offsets.update_tile_pattern.as_str())?;
    }

    return Ok(matcher);
  }

  #[cfg(target_os = "linux")]
  fn parse_gsrc_lib(&mut self) -> Result<()> {
    let mut symbols_map = IndexMap::new();

    symbols_map.insert(CONFIG.offsets.addst_symbol.as_str(), "addst");
    symbols_map.insert(CONFIG.offsets.addst_flag_symbol.as_str(), "addst_flag");
    symbols_map.insert(CONFIG.offsets.top_addst_symbol.as_str(), "top_addst");
    symbols_map.insert(CONFIG.offsets.draw_nineslice_symbol.as_str(), "draw_nineslice");
    symbols_map.insert(
      CONFIG.offsets.draw_horizontal_nineslice_symbol.as_str(),
      "draw_horizontal_nineslice",
    );
    symbols_map.insert(CONFIG.offsets.gps_allocate_symbol.as_str(), "gps_allocate");
    symbols_map.insert(CONFIG.offsets.update_all_symbol.as_str(), "update_all");
    symbols_map.insert(CONFIG.offsets.update_tile_symbol.as_str(), "update_tile");

    let path = std::path::Path::new(&self.path).parent().unwrap().join("libg_src_lib.so");
    let data = fs::read(path)?;
    let elf = ElfFile::new(&data)?;
    self.functions = elf.function_offsets(symbols_map)?;

    Ok(())
  }

  fn global_symbols(&self) -> Vec<String> {
    return vec!["enabler".into(), "gps".into()];
  }
}
