use std::ffi::{CString, CStr};
use std::path::PathBuf;
use crate::plugin_sys::PluginSystem;
use crate::plugin_info::PLUGIN_INFO;
use crate::meta_ffi::globals::{
  ENGINE_FUNCTIONS,
  META_UTIL_FUNCS,
};


pub struct ModuleContext {
  pub plugin_path: PathBuf,
  pub plugin_sys: PluginSystem,
}

static mut MODULE_CONTEXT: Option<ModuleContext> = None;

// These 2 should probably be unsafe functionsy
pub fn init_module_context(init: impl FnOnce() -> ModuleContext) {
  unsafe { MODULE_CONTEXT = Some(init()) };
}

pub fn destroy_module_context() {
  unsafe { MODULE_CONTEXT = None };
}

#[allow(dead_code)]
pub fn server_print(message: impl AsRef<str>) {
  let func = unsafe { (*ENGINE_FUNCTIONS).server_print };
  match CString::new(message.as_ref()) {
    Ok(msg) => unsafe { func(msg.as_ptr()) },
    Err(_) => return,
  };
}

pub fn log_console(message: impl AsRef<str>) {
  let func = unsafe { (*META_UTIL_FUNCS).log_console };
  match CString::new(message.as_ref()) {
    Ok(msg) => unsafe { func(&PLUGIN_INFO, msg.as_ptr()) },
    Err(_) => return,
  };
}

pub fn log_message(message: impl AsRef<str>) {
  let func = unsafe { (*META_UTIL_FUNCS).log_message };
  match CString::new(message.as_ref()) {
    Ok(msg) => unsafe { func(&PLUGIN_INFO, msg.as_ptr()) },
    Err(_) => return,
  };
}

pub fn log_error(message: impl AsRef<str>) {
  let func = unsafe { (*META_UTIL_FUNCS).log_error };
  match CString::new(message.as_ref()) {
    Ok(msg) => unsafe { func(&PLUGIN_INFO, msg.as_ptr()) },
    Err(_) => return,
  };
}


pub fn get_plugin_path() -> PathBuf {
  unsafe {
    let func = (*META_UTIL_FUNCS).get_plugin_path;
    PathBuf::from(CStr::from_ptr(func(&PLUGIN_INFO)).to_str().unwrap_or(""))
  }
}
