use std::collections::HashMap;

pub fn load_csv<T: serde::de::DeserializeOwned, P: AsRef<std::path::Path>, F>(path: P, f: F)
where
  F: FnMut(T),
{
  csv::Reader::from_reader(std::fs::File::open(path).unwrap()).deserialize::<T>().map(|it| it.unwrap()).for_each(f);
}

pub type SimpleDict = HashMap<String, String>;
pub fn load_csv_simple(path: impl AsRef<std::path::Path>) -> SimpleDict {
  let mut ret = SimpleDict::default();

  load_csv(path, |SimpleDictEntry { text, text_translation }| {
    if !text_translation.is_empty() {
      ret.insert(text, text_translation);
    }
  });

  ret
}

#[derive(Debug, serde::Deserialize)]
struct SimpleDictEntry {
  text: String,
  text_translation: String,
}
