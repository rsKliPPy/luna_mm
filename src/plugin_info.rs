use std::os::raw::c_char;
use crate::meta_ffi::types::{PluginInfo, LoadTime};
use crate::meta_ffi::constant::META_INTERFACE_VERSION;

pub static PLUGIN_INFO: PluginInfo = PluginInfo {
  if_version: unsafe { META_INTERFACE_VERSION },
  name: b"Luna\0" as *const u8 as *const c_char,
  version: b"0.1.0\0" as *const u8 as *const c_char,
  date: b"Unk\0" as *const u8 as *const c_char,
  author: b"KliPPy\0" as *const u8 as *const c_char,
  url: b"https://github.com/rsKliPPy/luna_mm\0" as *const u8 as *const c_char,
  logtag: b"LUNA\0" as *const u8 as *const c_char,
  loadable: LoadTime::StartUp,
  unloadable: LoadTime::Never,
};
