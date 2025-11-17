use super::types;
use super::GAME;

#[static_init::dynamic]
pub static FUNCTIONS: types::ModuleOffsets = {
  let mut ret = types::ModuleOffsets::default();

  #[cfg(target_os = "windows")]
  let module = "self".to_owned();
  #[cfg(target_os = "linux")]
  let module = "libg_src_lib.so".to_owned();

  for (name, offset) in GAME.functions() {
    ret.insert(name.to_owned(), (module.clone(), *offset));
  }

  ret
};
