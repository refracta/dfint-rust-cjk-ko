use std::path::PathBuf;

use crate::utils::data_path;

const LOOKUPS_DIR: &str = "lookups";
const DICTIONARIES_DIR: &str = "dictionaries";

fn lookups_path(subpath: &str) -> PathBuf {
  let mut ret = data_path(LOOKUPS_DIR);
  ret.push(subpath);
  ret
}

fn dictionaries_path(subpath: &str) -> PathBuf {
  let mut ret = data_path(DICTIONARIES_DIR);
  ret.push(subpath);
  ret
}

#[static_init::dynamic]
pub static TOP: super::LookupTree = {
  let mut ret = super::LookupTree::default();

  let lookup_directory = data_path(LOOKUPS_DIR);
  let mut lookup_csv_files = std::fs::read_dir(lookup_directory)
    .unwrap()
    .filter_map(|entry| {
      let entry = entry.unwrap();
      let path = entry.path();
      if path.extension().and_then(|s| s.to_str()) == Some("csv") {
        path.file_name().and_then(|s| s.to_str()).map(|s| s.to_owned())
      } else {
        None
      }
    })
    .collect::<Vec<_>>();
  lookup_csv_files.push(data_path("user_lookup.csv").to_str().unwrap().to_owned());

  for file in &lookup_csv_files {
    ret.load_lookup_csv(lookups_path(file));
  }

  let dictionaries_directory = data_path(DICTIONARIES_DIR);
  let dictionary_csv_files = std::fs::read_dir(dictionaries_directory)
    .unwrap()
    .filter_map(|entry| {
      let entry = entry.unwrap();
      let path = entry.path();
      if path.extension().and_then(|s| s.to_str()) == Some("csv") {
        path.file_name().and_then(|s| s.to_str()).map(|s| s.to_owned())
      } else {
        None
      }
    })
    .collect::<Vec<_>>();

  for file in &dictionary_csv_files {
    ret.load_dictionary_csv(dictionaries_path(file));
  }

  // ret.dump_all("");
  ret
};
