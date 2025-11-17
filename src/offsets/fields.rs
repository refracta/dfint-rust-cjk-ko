use super::types;
use crate::CONFIG;

#[static_init::dynamic]
pub static FIELDS: types::Offsets = {
  let mut ret = types::Offsets::default();

  let fields = vec![
    ("enabler.textures", CONFIG.offsets.enabler_textures_offset),
    ("renderer.sdl_renderer", CONFIG.offsets.renderer_sdl_renderer_offset),
    ("renderer.dispx_z", CONFIG.offsets.renderer_dispx_z_offset),
    ("gps.screenx", CONFIG.offsets.gps_screenx_offset),
    ("gps.screenf", CONFIG.offsets.gps_screenf_offset),
    ("gps.uccolor", CONFIG.offsets.gps_uccolor_offset),
    ("gps.top_in_use", CONFIG.offsets.gps_top_in_use_offset),
    ("gps.dimx", CONFIG.offsets.gps_dimx_offset),
  ];

  for (name, offset) in fields {
    ret.insert(name.to_owned(), offset);
  }
  ret
};
