use anyhow::Result;
use sdl2::{pixels::PixelFormatEnum, rect::Rect, surface::Surface, sys as sdl};
use std::collections::HashMap;
use std::io::Read;
use std::{mem, ptr};

use crate::config::CONFIG;
use crate::{df, encodings, utils};

pub const CJK_FONT_SIZE: u32 = 24;
pub const BUF_SIZE: isize = (CJK_FONT_SIZE * CJK_FONT_SIZE) as isize;

#[static_init::dynamic]
pub static mut FONT: Font = Font::new(&CONFIG.settings.font_file, CONFIG.settings.font_fallback_file.as_deref());

pub struct Font {
  font: fontdue::Font,
  fallback_font: Option<fontdue::Font>,
  cache: HashMap<char, usize>,
}

impl Font {
  fn new(path: &'static str, fallback_path: Option<&'static str>) -> Self {
    log::info!("使用字体文件：{path}");
    if let Some(fallback_path) = fallback_path {
      if !fallback_path.trim().is_empty() {
        log::info!("使用字体候补文件：{fallback_path}");
      }
    }

    Self {
      font: match Font::load(path) {
        Ok(value) => value,
        Err(message) => {
          let message = &format!("加载字体文件失败：{message}");
          utils::show_error_dialog(&message);
          panic!("{}", message);
        }
      },
      fallback_font: fallback_path
        .filter(|path| !path.trim().is_empty())
        .and_then(|path| match Font::load(path) {
          Ok(value) => Some(value),
          Err(message) => {
            log::warn!("加载字体候补文件失败，将忽略候补字体：{message}");
            None
          }
        }),
      cache: Default::default(),
    }
  }

  pub fn get(&mut self, ch: char) -> usize {
    if let Some(code) = encodings::utf8_char_to_ch437_byte(ch) {
      return df::enabler::get_curses_surface(*df::globals::ENABLER, code);
    };

    if !self.cache.contains_key(&ch) {
      let (mut metrics, mut bitmap) = self.font.rasterize(ch, CJK_FONT_SIZE as f32);
      let mut used_fallback = false;
      if metrics.width == 0 || metrics.height == 0 || !bitmap.iter().any(|&b| b != 0) {
        if let Some(fallback_font) = &self.fallback_font {
          (metrics, bitmap) = fallback_font.rasterize(ch, CJK_FONT_SIZE as f32);
          used_fallback = true;
        }
      }

      if used_fallback {
        log::debug!(
          "使用候补字体渲染字形：{ch:?} (U+{:04X}) adv=({:.2},{:.2}) size=({},{})",
          ch as u32,
          metrics.advance_width,
          metrics.advance_height,
          metrics.width,
          metrics.height
        );
      } else if metrics.width == 0 || metrics.height == 0 || !bitmap.iter().any(|&b| b != 0) {
        log::warn!("字体缺少字形：{ch:?} (U+{:04X})", ch as u32);
      }

      let mut surface = Surface::new(CJK_FONT_SIZE, CJK_FONT_SIZE, PixelFormatEnum::RGBA32).unwrap();
      surface.with_lock_mut(|buffer| {
        buffer.fill(0);

        // Keep baseline aligned at `CJK_FONT_SIZE - 3` (works for NotoSansMonoCJK* fonts),
        // and center glyphs with narrower advance width (e.g. Hangul: 22px at 24px size).
        let advance_width = metrics.advance_width.round() as i32;
        let center_x = (CJK_FONT_SIZE as i32 - advance_width) / 2;
        let dx = metrics.xmin + center_x;

        let dy = (CJK_FONT_SIZE as i32 - metrics.height as i32) - (metrics.ymin + 3);
        let dy = if dy < 0 { 0 } else { dy };

        for y in 0..metrics.height {
          for x in 0..metrics.width {
            let alpha = bitmap[y * metrics.width + x];
            if alpha == 0 {
              continue;
            }

            let offset = ((y as i32 + dy) * CJK_FONT_SIZE as i32 + x as i32 + dx) as isize;
            if offset < 0 || offset >= BUF_SIZE {
              continue;
            }
            let offset = offset as usize;

            buffer[offset * 4 + 0] = 255;
            buffer[offset * 4 + 1] = 255;
            buffer[offset * 4 + 2] = 255;
            buffer[offset * 4 + 3] = alpha;
          }
        }
      });
      let surface_ptr = surface.raw() as usize;
      mem::forget(surface);

      self.cache.insert(ch, surface_ptr);
    }

    if let Some(&surface_ptr) = self.cache.get(&ch) {
      return surface_ptr;
    } else {
      // fallback to curses space glyph
      return df::enabler::get_curses_surface(*df::globals::ENABLER, ' ' as u8);
    }
  }

  pub fn render(&mut self, string: String, fg: df::common::Color, bg: Option<df::common::Color>) -> (usize, u32) {
    let width = encodings::string_width_in_pixels(&string);
    let height = CJK_FONT_SIZE;
    let mut x = 0;
    let text_surface = Surface::new(width, height, PixelFormatEnum::RGBA32).unwrap();
    for ch in string.chars() {
      let surface_ptr = self.get(ch);
      let glyph_surface = surface_ptr as *mut sdl::SDL_Surface;
      let w = encodings::char_width_in_pixels(ch);
      let h = CJK_FONT_SIZE;
      let mut rect = Rect::new(x, 0, w, h);
      unsafe { sdl::SDL_UpperBlitScaled(glyph_surface, ptr::null(), text_surface.raw(), rect.raw_mut()) };
      x += w as i32;
    }
    let df::common::Color { r, g, b } = fg;
    unsafe { sdl::SDL_SetSurfaceColorMod(text_surface.raw(), r, g, b) };

    let surface = Surface::new(width, height, PixelFormatEnum::RGBA32).unwrap();
    if let Some(df::common::Color { r, g, b }) = bg {
      let bc = sdl2::pixels::Color::RGB(r, g, b).to_u32(&surface.pixel_format());
      unsafe { sdl::SDL_FillRect(surface.raw(), ptr::null(), bc) };
    }
    unsafe { sdl::SDL_UpperBlit(text_surface.raw(), ptr::null(), surface.raw(), ptr::null_mut()) };

    let surface_ptr = surface.raw() as usize;
    mem::forget(surface);

    return (surface_ptr, width);
  }

  fn load(path: &str) -> Result<fontdue::Font> {
    let mut file = std::fs::File::open(path)?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;

    fontdue::Font::from_bytes(data, fontdue::FontSettings::default()).map_err(|err| anyhow::anyhow!(err))
  }
}
