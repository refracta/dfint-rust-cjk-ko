use std::path::PathBuf;

const DATA_DIR: &str = "dfint-data";
const OFFSETS_DIR: &str = "offsets";
const LOOKUPS_DIR: &str = "lookups";
const DICTIONARIES_DIR: &str = "dictionaries";

pub fn data_path(subpath: &str) -> PathBuf {
  let mut ret = PathBuf::new();
  ret.push(DATA_DIR);
  ret.push(subpath);
  ret
}

pub fn offsets_path(subpath: &str) -> PathBuf {
  let mut ret = data_path(OFFSETS_DIR);
  ret.push(subpath);
  ret
}

pub fn lookups_path(subpath: &str) -> PathBuf {
  let mut ret = data_path(LOOKUPS_DIR);
  ret.push(subpath);
  ret
}

pub fn dictionaries_path(subpath: &str) -> PathBuf {
  let mut ret = data_path(DICTIONARIES_DIR);
  ret.push(subpath);
  ret
}
