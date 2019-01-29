pub mod ffi_util_funcs;

use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int};
use std::path::PathBuf;
use crate::module;
use crate::plugin_info::PLUGIN_INFO;
use crate::meta_ffi::globals::{
  ENGINE_FUNCTIONS,
  META_UTIL_FUNCS,
};
use crate::meta_ffi::types::Edict;
use self::ffi_util_funcs::FFIUtilFuncs;

// TODO: Redo this module, properly wrap unsafe functions into safe FFUtilFuncs

static mut MODULE_CONTEXT: Option<Box<dyn MetaContext>> = None;
static UTIL_FUNCS: FFIUtilFuncs = FFIUtilFuncs::new();

pub trait MetaContext {
  fn client_connected(&mut self) { }
}

pub unsafe fn game_init() {
  let ctx: Box<dyn MetaContext> = module::module_init(&UTIL_FUNCS);
  MODULE_CONTEXT = Some(ctx);
}

pub unsafe fn game_shutdown() {
  let ctx = MODULE_CONTEXT.take().unwrap();
  module::module_shutdown(ctx);
}

pub unsafe fn client_connect(
  entity: *mut Edict,
  name: *const c_char,
  address: *const c_char,
  reject_reason: *mut c_char,
) -> c_int {
  if let Some(ctx) = MODULE_CONTEXT.as_mut() {
    ctx.client_connected();
  }

  1
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


pub unsafe fn get_plugin_path() -> PathBuf {
  let func = (*META_UTIL_FUNCS).get_plugin_path;
  PathBuf::from(CStr::from_ptr(func(&PLUGIN_INFO)).to_str().unwrap_or(""))
}
