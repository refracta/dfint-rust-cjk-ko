use super::types;
use super::GAME;

#[static_init::dynamic]
pub static GLOBALS: types::ModuleOffsets = {
  let mut ret = types::ModuleOffsets::default();

  for (name, offset) in GAME.globals() {
    ret.insert(name.to_owned(), ("self".to_owned(), *offset));
  }

  ret
};
