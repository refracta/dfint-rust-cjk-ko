use crate::{config, offsets};

pub fn translate_version(string: &str) -> Option<String> {
  if string == offsets::VERSION.as_str() {
    return Some(format!(
      "{string} + dfint-rust-cjk/{}-{}",
      *offsets::PLATFORM,
      config::CONFIG.version
    ));
  }

  None
}
