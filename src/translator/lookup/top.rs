const LOOKUP_CSV_FILES: &[&str] = &[
  "creatures.csv",
  "plants.csv",
  "skills.csv",
  "professions.csv",
  "positions.csv",
  "materials.csv",
  "tiles.csv",
  "info-tags.csv",
  "gems.csv",
  "gems-details.csv",
  "weapons.csv",
  "armors.csv",
  "shoes.csv",
  "shields.csv",
  "helms.csv",
  "gloves.csv",
  "ammos.csv",
  "meats.csv",
  "pants.csv",
  "siegeammos.csv",
  "trapcomps.csv",
  "items.csv",
  "construction-menus.csv",
  "tasks.csv",
  "index.csv",
];
const DICTIONARY_CSV_FILES: &[&str] = &["creatures.csv", "plants.csv"];

#[static_init::dynamic]
pub static TOP: super::LookupTree = {
  let mut ret = super::LookupTree::default();

  for &file in LOOKUP_CSV_FILES {
    ret.load_lookup_csv(file);
  }

  for &file in DICTIONARY_CSV_FILES {
    ret.load_dictionary_csv(file);
  }

  // ret.dump_all("");
  ret
};
