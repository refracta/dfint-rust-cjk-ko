use anyhow::{anyhow, Ok, Result};
use std::io::Read;

use crate::utils;

#[derive(Default, Debug)]
pub struct Offsets {
  // Windows search patterns
  #[cfg(target_os = "windows")]
  pub addst_pattern: String,
  #[cfg(target_os = "windows")]
  pub addst_flag_pattern: String,
  #[cfg(target_os = "windows")]
  pub top_addst_pattern: String,
  #[cfg(target_os = "windows")]
  pub draw_nineslice_pattern: String,
  #[cfg(target_os = "windows")]
  pub draw_horizontal_nineslice_pattern: String,
  #[cfg(target_os = "windows")]
  pub gps_allocate_pattern: String,
  #[cfg(target_os = "windows")]
  pub update_all_pattern: String,
  #[cfg(target_os = "windows")]
  pub update_tile_pattern: String,
  // Linux search symbols
  #[cfg(target_os = "linux")]
  pub addst_symbol: String,
  #[cfg(target_os = "linux")]
  pub addst_flag_symbol: String,
  #[cfg(target_os = "linux")]
  pub top_addst_symbol: String,
  #[cfg(target_os = "linux")]
  pub draw_nineslice_symbol: String,
  #[cfg(target_os = "linux")]
  pub draw_horizontal_nineslice_symbol: String,
  #[cfg(target_os = "linux")]
  pub gps_allocate_symbol: String,
  #[cfg(target_os = "linux")]
  pub update_all_symbol: String,
  #[cfg(target_os = "linux")]
  pub update_tile_symbol: String,
  // Fields offsets
  pub enabler_textures_offset: usize,
  pub renderer_sdl_renderer_offset: usize,
  pub renderer_dispx_z_offset: usize,
  pub gps_screenx_offset: usize,
  pub gps_screenf_offset: usize,
  pub gps_uccolor_offset: usize,
  pub gps_top_in_use_offset: usize,
  pub gps_dimx_offset: usize,
}

impl Offsets {
  pub fn load() -> Result<Self> {
    let mut offsets = Self::default();

    let mut file = std::fs::File::open("./dfint-data/offsets.txt")?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)?;

    for cap in regex::bytes::Regex::new(r"\[([^\]]+):([^\]:]+)\]")?.captures_iter(&contents) {
      let key = String::from_utf8_lossy(&cap[1]).into_owned();
      let value = String::from_utf8_lossy(&cap[2]).into_owned();

      #[cfg(target_os = "windows")]
      {
        match key.as_str() {
          "PATTERN:ADDST" => offsets.addst_pattern = value,
          "PATTERN:ADDST_FLAG" => offsets.addst_flag_pattern = value,
          "PATTERN:TOP_ADDST" => offsets.top_addst_pattern = value,
          "PATTERN:DRAW_NINESLICE" => offsets.draw_nineslice_pattern = value,
          "PATTERN:DRAW_HORIZONTAL_NINESLICE" => offsets.draw_horizontal_nineslice_pattern = value,
          "PATTERN:GPS_ALLOCATE" => offsets.gps_allocate_pattern = value,
          "PATTERN:UPDATE_ALL" => offsets.update_all_pattern = value,
          "PATTERN:UPDATE_TILE" => offsets.update_tile_pattern = value,
          "OFFSET:ENABLER:TEXTURES:WINDOWS" => offsets.enabler_textures_offset = parse_hex(&value)?,
          "OFFSET:RENDERER:SDL_RENDERER:WINDOWS" => offsets.renderer_sdl_renderer_offset = parse_hex(&value)?,
          "OFFSET:RENDERER:DISPX_Z:WINDOWS" => offsets.renderer_dispx_z_offset = parse_hex(&value)?,
          "OFFSET:GPS:SCREENX:WINDOWS" => offsets.gps_screenx_offset = parse_hex(&value)?,
          "OFFSET:GPS:SCREENF:WINDOWS" => offsets.gps_screenf_offset = parse_hex(&value)?,
          "OFFSET:GPS:UCCOLOR:WINDOWS" => offsets.gps_uccolor_offset = parse_hex(&value)?,
          "OFFSET:GPS:TOP_IN_USE:WINDOWS" => offsets.gps_top_in_use_offset = parse_hex(&value)?,
          "OFFSET:GPS:DIMX:WINDOWS" => offsets.gps_dimx_offset = parse_hex(&value)?,
          _ => {}
        }
      }

      #[cfg(target_os = "linux")]
      {
        match key.as_str() {
          "SYMBOL:ADDST" => offsets.addst_symbol = value,
          "SYMBOL:ADDST_FLAG" => offsets.addst_flag_symbol = value,
          "SYMBOL:TOP_ADDST" => offsets.top_addst_symbol = value,
          "SYMBOL:DRAW_NINESLICE" => offsets.draw_nineslice_symbol = value,
          "SYMBOL:DRAW_HORIZONTAL_NINESLICE" => offsets.draw_horizontal_nineslice_symbol = value,
          "SYMBOL:GPS_ALLOCATE" => offsets.gps_allocate_symbol = value,
          "SYMBOL:UPDATE_ALL" => offsets.update_all_symbol = value,
          "SYMBOL:UPDATE_TILE" => offsets.update_tile_symbol = value,
          "OFFSET:ENABLER:TEXTURES:LINUX" => offsets.enabler_textures_offset = parse_hex(&value)?,
          "OFFSET:RENDERER:SDL_RENDERER:LINUX" => offsets.renderer_sdl_renderer_offset = parse_hex(&value)?,
          "OFFSET:RENDERER:DISPX_Z:LINUX" => offsets.renderer_dispx_z_offset = parse_hex(&value)?,
          "OFFSET:GPS:SCREENX:LINUX" => offsets.gps_screenx_offset = parse_hex(&value)?,
          "OFFSET:GPS:SCREENF:LINUX" => offsets.gps_screenf_offset = parse_hex(&value)?,
          "OFFSET:GPS:UCCOLOR:LINUX" => offsets.gps_uccolor_offset = parse_hex(&value)?,
          "OFFSET:GPS:TOP_IN_USE:LINUX" => offsets.gps_top_in_use_offset = parse_hex(&value)?,
          "OFFSET:GPS:DIMX:LINUX" => offsets.gps_dimx_offset = parse_hex(&value)?,
          _ => {}
        }
      }
    }

    Ok(offsets)
  }
}

fn parse_hex(value: &str) -> Result<usize> {
  utils::parse_hex_as_usize(&value).ok_or(anyhow!("无法解析偏移量：{}", value))
}
