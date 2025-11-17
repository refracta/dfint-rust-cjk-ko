mod types;

mod platform;
pub use platform::*;

mod functions;
pub use functions::*;

mod globals;
pub use globals::*;

mod fields;
pub use fields::*;

mod game;

#[static_init::dynamic]
pub static GAME: game::Game = game::Game::new(&PATH_EXE).unwrap();
