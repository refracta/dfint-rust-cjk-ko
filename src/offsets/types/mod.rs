use std::collections::BTreeMap;

mod os_specific_offsets;
pub use os_specific_offsets::*;

mod platform_specific_offsets;
pub use platform_specific_offsets::*;

pub type Offsets = BTreeMap<String, usize>;
pub type ModuleOffsets = BTreeMap<String, (String, usize)>;
