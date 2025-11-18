// Trait for matching patterns in PE files
pub trait Matcher {
  // Matches data at the given offset with matcher, return true to stop
  fn matches(&mut self, offset: usize, data: &[u8]);
  // Window size needed for matching
  fn len(&self) -> usize;
}

mod hex;
pub use hex::HexMatcher;
