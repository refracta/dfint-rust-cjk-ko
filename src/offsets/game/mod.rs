mod matcher;

#[cfg(target_os = "linux")]
mod elf_file;
#[cfg(target_os = "windows")]
mod pe_file;

mod globals;

mod game;
pub use game::Game;
