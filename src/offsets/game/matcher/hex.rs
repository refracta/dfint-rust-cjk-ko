use std::collections::HashMap;

use anyhow::Result;

use super::Matcher;

enum HexToken {
  Value(u8),
  Digit,
  Wildcard,
}

pub struct HexMatcher {
  keys: Vec<String>,
  patterns: HashMap<String, Vec<HexToken>>,
  result: Option<HashMap<String, Vec<u32>>>,
}

impl HexMatcher {
  pub fn new() -> HexMatcher {
    HexMatcher {
      keys: Vec::new(),
      patterns: HashMap::new(),
      result: None,
    }
  }

  pub fn add_pattern(&mut self, name: &str, pattern: &str) -> Result<()> {
    let mut tokens = Vec::new();
    for hex in pattern.split(' ') {
      let value = if hex == "??" {
        HexToken::Wildcard
      } else if hex == "##" {
        HexToken::Digit
      } else {
        HexToken::Value(u8::from_str_radix(hex, 16)?)
      };
      tokens.push(value);
    }

    self.patterns.insert(name.to_owned(), tokens);
    self.keys.push(name.to_owned());
    Ok(())
  }

  pub fn keys(&self) -> &Vec<String> {
    &self.keys
  }

  pub fn data_len(&self, key: &str) -> usize {
    self.patterns.get(key).map(|v| v.len()).unwrap_or(0)
  }

  pub fn get_results(&self, key: &str) -> Vec<u32> {
    match &self.result {
      Some(results) => results.get(key).cloned().unwrap_or_default(),
      None => Vec::new(),
    }
  }
}

impl Matcher for HexMatcher {
  fn matches(&mut self, offset: usize, data: &[u8]) {
    let result = self.result.get_or_insert_default();

    'pattern: for (name, tokens) in &self.patterns {
      // note data is alwasys at least as long as tokens due to windows size
      for (token, byte) in tokens.iter().zip(data.iter()) {
        match token {
          HexToken::Wildcard => continue,
          HexToken::Digit if byte.is_ascii_digit() => continue,
          HexToken::Value(expected) if *expected == *byte => continue,
          _ => continue 'pattern,
        }
      }

      // all tokens matched
      result.entry(name.clone()).or_insert_with(Vec::new).push(offset as u32);
    }
  }

  fn len(&self) -> usize {
    self.patterns.values().map(|v| v.len()).max().unwrap_or(0)
  }
}
