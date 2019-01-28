use std::os::raw::{c_char, c_int};

pub static mut META_INTERFACE_VERSION: *const c_char =
  b"5:13\0" as *const u8 as *const c_char;

pub const DLL_INTERFACE_VERSION: c_int = 140;
pub const ENGINE_INTERFACE_VERSION: c_int = 138;
pub const NEW_DLL_INTERFACE_VERSION: c_int = 1;
