use crate::offsets;
use crate::utils::OFFSETS;

#[static_init::dynamic]
pub static ENABLER: usize = OFFSETS.get(offsets::GLOBALS.get("enabler").unwrap());

#[static_init::dynamic]
pub static GPS: usize = OFFSETS.get(offsets::GLOBALS.get("gps").unwrap());
