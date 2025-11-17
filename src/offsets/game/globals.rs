use std::collections::HashMap;

pub fn parse_globals(data: &[u8], global_table_offset: usize) -> HashMap<usize, usize> {
  let mut globals = HashMap::new();

  for chunk in data[global_table_offset..].chunks(24) {
    if chunk.len() < 24 {
      break;
    }

    let symbol_address = usize::from_le_bytes(chunk[0..8].try_into().unwrap());
    let global_address = usize::from_le_bytes(chunk[8..16].try_into().unwrap());
    if symbol_address == 0 {
      break;
    }

    globals.insert(symbol_address, global_address);
  }

  globals
}
