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

// TODO: Redo this module, properly wrap unsafe functions into safe FFUtilFuncs

static mut MODULE_CONTEXT: Option<Box<dyn MetaContext>> = None;

pub trait MetaContext {
  fn client_connected(&mut self) { }
}

pub unsafe fn game_init() {
  let ctx: Box<dyn MetaContext> = module::module_init();
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

// These all are marker as `unsafe` because they may only be called
// from the main thread. I'm not sure if I can enforce this in Lua callbacks
// in any way without making it too compilcated with types and lifetimes.

#[allow(dead_code)]
pub unsafe fn server_print(message: impl AsRef<str>) {
  let func = (*ENGINE_FUNCTIONS).server_print;
  match CString::new(message.as_ref()) {
    Ok(msg) => func(msg.as_ptr()),
    Err(_) => return,
  };
}

pub unsafe fn log_console(message: impl AsRef<str>) {
  let func = (*META_UTIL_FUNCS).log_console;
  match CString::new(message.as_ref()) {
    Ok(msg) => func(&PLUGIN_INFO, msg.as_ptr()),
    Err(_) => return,
  };
}

pub unsafe fn log_message(message: impl AsRef<str>) {
  let func = (*META_UTIL_FUNCS).log_message;
  match CString::new(message.as_ref()) {
    Ok(msg) => func(&PLUGIN_INFO, msg.as_ptr()),
    Err(_) => return,
  };
}

pub unsafe fn log_error(message: impl AsRef<str>) {
  let func = (*META_UTIL_FUNCS).log_error;
  match CString::new(message.as_ref()) {
    Ok(msg) => func(&PLUGIN_INFO, msg.as_ptr()),
    Err(_) => return,
  };
}


pub unsafe fn get_meta_plugin_path() -> PathBuf {
  let func = (*META_UTIL_FUNCS).get_plugin_path;
  PathBuf::from(CStr::from_ptr(func(&PLUGIN_INFO)).to_str().unwrap_or(""))
}
