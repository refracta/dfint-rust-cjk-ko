use std::path::PathBuf;

const DATA_DIR: &str = "dfint-data";

pub fn data_path(subpath: &str) -> PathBuf {
  let mut ret = PathBuf::new();
  ret.push(DATA_DIR);
  ret.push(subpath);
  ret
}
