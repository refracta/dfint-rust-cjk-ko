use crate::{config, offsets};

pub fn translate_version(string: &str) -> Option<String> {
  if string == offsets::VERSION.as_str() {
    return Some(format!(
      "{string} + dfint-rust-cjk-ko/{}-{}",
      *offsets::PLATFORM,
      config::CONFIG.version
    ));
  }

  None
}
