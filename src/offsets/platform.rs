use std::fs;

use crate::utils;

#[cfg(target_os = "linux")]
pub const PATH_EXE: &'static str = "./dwarfort";

#[cfg(target_os = "windows")]
pub const PATH_EXE: &'static str = "./Dwarf Fortress.exe";

#[static_init::dynamic]
pub static PLATFORM: String = {
  let os = std::env::consts::OS;

  let mut platform = "itch";
  if let Ok(content) = fs::read(PATH_EXE) {
    let target = "SteamAPI".as_bytes();
    if content.windows(target.len()).any(|window| window == target) {
      platform = "steam";
    }
  } else {
    const message: &str = "无法读取可执行文件以检测平台";
    utils::show_error_dialog(&message);
    panic!("{}", message);
  }

  return format!("{}-{}", os, platform);
};
