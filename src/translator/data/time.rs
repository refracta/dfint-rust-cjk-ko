use crate::utils;

#[static_init::dynamic]
pub static MONTHS: utils::SimpleDict = utils::load_csv_simple(utils::translations_path("months.csv"));

#[static_init::dynamic]
pub static SEASONS: utils::SimpleDict = utils::load_csv_simple(utils::translations_path("seasons.csv"));
