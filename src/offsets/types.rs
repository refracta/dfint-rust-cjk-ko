use std::collections::BTreeMap;

pub type Offsets = BTreeMap<String, usize>;
pub type ModuleOffsets = BTreeMap<String, (String, usize)>;
