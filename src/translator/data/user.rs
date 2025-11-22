use std::collections::HashMap;

use crate::utils;

#[derive(Debug, serde::Deserialize)]
struct Entry {
  text: String,
  translation: String,
}

#[derive(Debug, Default)]
pub struct User {
  dict: HashMap<String, String>,
}

impl User {
  pub fn get(&self, key: &String) -> Option<&String> {
    self.dict.get(key)
  }
}

#[static_init::dynamic]
pub static USER: User = {
  let mut ret = User::default();
  let file = utils::data_path("user-dictionary.csv");
  if file.exists() {
    utils::load_csv(file, |Entry { text, translation }| {
      ret.dict.insert(text.to_lowercase(), translation);
    });
  }
  ret
};
