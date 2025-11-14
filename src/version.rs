use std::fs;

use crate::{offsets::PATH_EXE, utils};

#[static_init::dynamic]
pub static VERSION: String = {
  let mut message: &str = "无法读取可执行文件以检测版本";
  if let Ok(content) = fs::read(PATH_EXE) {
    // format: "XX.XX\0"
    let mut version: Option<String> = None;
    let mut max_found = 0f32;
    content.windows(6).for_each(|window| {
      if window[0].is_ascii_digit()
        && window[1].is_ascii_digit()
        && window[2] == b'.'
        && window[3].is_ascii_digit()
        && window[4].is_ascii_digit()
        && window[5] == 0
      {
        let digit: f32 = str::from_utf8(&window[0..5]).unwrap().parse().unwrap();
        if digit > max_found {
          max_found = digit;
          version = Some(str::from_utf8(&window[0..5]).unwrap().to_string());
        }
      }
    });

    if let Some(version) = version {
      return version;
    }

    message = "无法在可执行文件中找到版本信息";
  }

  utils::show_error_dialog(&message);
  panic!("{}", message);
};
