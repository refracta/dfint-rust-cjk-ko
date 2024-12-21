use regex::Regex;

use crate::encodings;

use super::data;

#[static_init::dynamic]
static DATE_REGEX: Regex = Regex::new(r"^(\d+)(?:st|nd|rd|th) (.*)$").unwrap();

fn translate_date(string: &str) -> Option<String> {
  if let Some(caps) = DATE_REGEX.captures(string) {
    if let Some(date_str_match) = caps.get(1) {
      if let Some(month_match) = caps.get(2) {
        if let Ok(date) = date_str_match.as_str().parse::<usize>() {
          let month = month_match.as_str();
          if let Some(month_translated) = data::MONTHS.get(month) {
            return Some(format!("{month_translated} {date} 日"));
          }
        }
      }
    }
  }

  None
}

fn translate_season(season: &str) -> Option<String> {
  if let Some(season_translated) = data::SEASONS.get(season) {
    return Some(season_translated.to_owned());
  }

  None
}

fn translate_year(string: &str) -> Option<String> {
  if let Some((prefix, year_str)) = string.split_once(' ') {
    if prefix == "Year" {
      if let Ok(year) = year_str.parse::<usize>() {
        return Some(format!("{year} 年"));
      }
    }
  }

  None
}

pub fn translate_interface(view_opt: Option<&str>, location_opt: Option<&str>, string: &str) -> Option<(String, i32)> {
  if let Some(view) = view_opt {
    let location = location_opt.unwrap_or("");

    if view == "dwarfmode" {
      let translation_opt = match location {
        "date" => translate_date(string),
        "season" => translate_season(string),
        "year" => translate_year(string),
        _ => None,
      };

      if let Some(translation) = translation_opt {
        return Some((translation, 0));
      }
    }

    if let Some(dictionaries) = data::INTERFACES.get(view) {
      if let Some(dictionary) = dictionaries.get(location) {
        if let Some((translation, alignment)) = dictionary.get(string) {
          let string_length = encodings::string_width_in_pixels(string);
          let translation_length = encodings::string_width_in_pixels(translation);
          let length_diff = string_length as i32 - translation_length as i32;
          let horizontal_shift = match alignment {
            data::Alignment::LEFT => 0,
            data::Alignment::CENTER => length_diff / 2,
            data::Alignment::RIGHT => length_diff,
          };
          return Some((translation.to_owned(), horizontal_shift));
        }
      }
    }
  }

  None
}
