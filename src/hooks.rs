use anyhow::Result;
use raw::{delete_cxxstring, new_cxxstring_n_chars};
use retour::static_detour;

use super::{df, encodings, offsets, screen, translator, utils};

use r#macro::hook;

pub unsafe fn attach_all() -> Result<()> {
  attach_addst()?;
  attach_top_addst()?;
  attach_addst_flag()?;
  attach_gps_allocate()?;
  attach_update_all()?;
  attach_update_tile()?;
  attach_draw_horizontal_nineslice()?;
  attach_draw_nineslice()?;

  Ok(())
}

pub unsafe fn enable_all() -> Result<()> {
  enable_addst()?;
  enable_top_addst()?;
  enable_addst_flag()?;
  enable_gps_allocate()?;
  enable_update_all()?;
  enable_update_tile()?;
  enable_draw_horizontal_nineslice()?;
  enable_draw_nineslice()?;

  Ok(())
}

pub unsafe fn disable_all() -> Result<()> {
  disable_addst()?;
  disable_top_addst()?;
  disable_addst_flag()?;
  disable_gps_allocate()?;
  disable_update_all()?;
  disable_update_tile()?;
  disable_draw_horizontal_nineslice()?;
  disable_draw_nineslice()?;

  Ok(())
}

#[hook]
fn addst(gps: usize, string_address: usize, just: u8, space: i32) {
  let bt = utils::backtrace();
  let string = encodings::read_raw_string(string_address);

  let text = screen::Text::new(translator::TRANSLATOR.write().translate("addst", &string, &bt)).by_gps(gps);
  let width = screen::SCREEN.write().add_text(text);

  let dummy_ptr = new_cxxstring_n_chars(width, ' ' as u8);
  unsafe { original!(gps, dummy_ptr, just, space) };
  delete_cxxstring(dummy_ptr);
}

#[hook]
fn addst_flag(gps: usize, string_address: usize, just: u8, space: i32, sflag: u32) {
  let bt = utils::backtrace();
  let string = encodings::read_raw_string(string_address);

  let text =
    screen::Text::new(translator::TRANSLATOR.write().translate("addst_flag", &string, &bt)).by_gps(gps).with_sflag(sflag);
  let width = screen::SCREEN.write().add_text(text);

  let dummy_ptr = new_cxxstring_n_chars(width, ' ' as u8);
  unsafe { original!(gps, dummy_ptr, just, space, sflag) };
  delete_cxxstring(dummy_ptr);
}

#[hook]
fn top_addst(gps: usize, string_address: usize, just: u8, space: i32) {
  let bt = utils::backtrace();
  let string = encodings::read_raw_string(string_address);

  let text = screen::Text::new(translator::TRANSLATOR.write().translate("top_addst", &string, &bt)).by_gps(gps);
  let width = screen::SCREEN_TOP.write().add_text(text);

  let dummy_ptr = new_cxxstring_n_chars(width, ' ' as u8);
  unsafe { original!(gps, dummy_ptr, just, space) };
  delete_cxxstring(dummy_ptr);
}

#[hook]
fn draw_nineslice(texpos: usize, sy: i32, sx: i32, ey: i32, ex: i32, flag: u8) {
  unsafe { original!(texpos, sy, sx, ey, ex, flag) };
  if flag & 1 == 1 {
    let cover = screen::Cover::new(sx, sy, ex, ey);
    screen::SCREEN.write().add_cover(cover);
  }
}

#[hook]
fn draw_horizontal_nineslice(texpos: usize, sy: i32, sx: i32, ey: i32, ex: i32, flag: u8) {
  unsafe { original!(texpos, sy, sx, ey, ex, flag) };
  if flag & 1 == 1 {
    let cover = screen::Cover::new(sx, sy, ex, ey);
    screen::SCREEN.write().add_cover(cover);
  }
}

#[hook]
fn gps_allocate(renderer: usize, x: i32, y: i32, screen_x: u32, screen_y: u32, tile_dim_x: u32, tile_dim_y: u32) {
  // graphicst::resize is inlined in Windows, hook gps_allocate instead
  unsafe { original!(renderer, x, y, screen_x, screen_y, tile_dim_x, tile_dim_y) };
  screen::SCREEN.write().resize(x, y);
  screen::SCREEN_TOP.write().resize(x, y);
}

#[hook]
fn update_all(renderer: usize) {
  unsafe { original!(renderer) };

  if df::gps::top_in_use(*df::globals::GPS) {
    screen::SCREEN_TOP.write().render(renderer);
    screen::SCREEN_TOP.write().clear();
  }
}

#[hook]
fn update_tile(renderer: usize, x: i32, y: i32) {
  unsafe { original!(renderer, x, y) };
  let dim = df::gps::borrow_dim(*df::globals::GPS);

  // hack to render text after the last update_tile in update_all
  if (x != dim.x - 1 || y != dim.y - 1) {
    return;
  }

  screen::SCREEN.write().render(renderer);
  screen::SCREEN.write().clear();
}
