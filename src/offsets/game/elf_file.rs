use std::collections::HashMap;

use anyhow::Result;
use goblin::elf::Elf;
use indexmap::IndexMap;

pub struct ElfFile {
  symbols: HashMap<String, usize>,
}

impl ElfFile {
  pub fn new(data: &[u8]) -> Result<ElfFile> {
    let mut symbols = HashMap::new();

    let elf = Elf::parse(&data)?;
    for dynsym in elf.dynsyms.iter() {
      if let Some(name) = elf.dynstrtab.get_at(dynsym.st_name) {
        symbols.insert(name.to_owned(), dynsym.st_value as usize);
      }
    }

    Ok(ElfFile { symbols })
  }

  pub fn function_offsets(&self, symbols_map: IndexMap<&str, &str>) -> Result<IndexMap<String, usize>> {
    let mut offsets = IndexMap::new();

    for (symbol, name) in symbols_map.into_iter() {
      if let Some(&offset) = self.symbols.get(symbol) {
        offsets.insert(name.to_owned(), offset);
      } else {
        return Err(anyhow::anyhow!("Symbol not found: {}", symbol));
      }
    }

    Ok(offsets)
  }
}
