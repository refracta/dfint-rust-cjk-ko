pub fn parse_hex_as_usize(string: &str) -> Option<usize> {
  usize::from_str_radix(string.strip_prefix("0x").unwrap(), 16).ok()
}
