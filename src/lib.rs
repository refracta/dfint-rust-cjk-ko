#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

mod config;
mod df;
mod encodings;
mod font;
mod hooks;
mod offsets;
mod screen;
mod translator;
mod utils;
mod watchdog;

use crate::config::CONFIG;

#[static_init::constructor]
#[no_mangle]
extern "C" fn attach() {
  std::env::set_var("RUST_BACKTRACE", "1");

  log::info!("dfint 버전: {}", CONFIG.version);
  log::info!("dfint 플랫폼: {}", *offsets::PLATFORM);
  log::info!("폰트 파일: {:?}", CONFIG.settings.font_file);

  match unsafe { hooks::attach_all() } {
    Ok(_) => log::debug!("한국어 번역이 활성화되었습니다"),
    Err(err) => {
      log::error!("unable to attach hooks, {:?}", err);
      utils::show_error_dialog("한국어 번역을 활성화할 수 없습니다");
      return;
    }
  };
  watchdog::install();

  log::info!("{:#?}", offsets::FIELDS.clone());
}

#[static_init::destructor]
#[no_mangle]
extern "C" fn detach() {
  unsafe {
    watchdog::uninstall();
    let _ = hooks::disable_all();
    log::debug!("한국어 번역이 비활성화되었습니다");
  }
}

#[no_mangle]
extern "C" fn super_secret_dfint_sign() -> u8 {
  69
}
