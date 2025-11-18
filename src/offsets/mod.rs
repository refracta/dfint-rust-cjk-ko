mod types;

mod functions;
pub use functions::*;

mod globals;
pub use globals::*;

mod fields;
pub use fields::*;

mod game;

#[cfg(target_os = "linux")]
pub const PATH_EXE: &'static str = "./dwarfort";

#[cfg(target_os = "windows")]
pub const PATH_EXE: &'static str = "./Dwarf Fortress.exe";

#[static_init::dynamic]
pub static GAME: game::Game = game::Game::new(&PATH_EXE).unwrap();

#[static_init::dynamic]
pub static VERSION: String = GAME.version().to_owned();

#[static_init::dynamic]
pub static PLATFORM: String = format!("{}-{}", GAME.os(), GAME.platform());
