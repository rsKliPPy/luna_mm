pub mod hl_lua_bridge;

use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int};
use std::path::PathBuf;
use crate::module;
use crate::plugin_info::PLUGIN_INFO;
use crate::meta_ffi::globals::{
  ENGINE_FUNCTIONS,
  META_UTIL_FUNCS,
};
use crate::meta_ffi::types::{
  Edict,
  EntVars,
  EngineStringHandle,
};
use self::hl_lua_bridge::{EntityHandle};

// TODO: Redo this module, organize things better

static mut MODULE_CONTEXT: Option<Box<dyn MetaContext>> = None;

pub trait MetaContext {
  fn client_connect(&mut self, _entity: EntityHandle) { }
  fn client_put_in_server(&mut self, _entity: EntityHandle) { }
  fn client_disconnect(&mut self, _entity: EntityHandle) { }
  fn client_put_in_server_post(&mut self, _entity: EntityHandle) { }
  fn client_disconnect_post(&mut self, _entity: EntityHandle) { }
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
  _name: *const c_char,
  _address: *const c_char,
  _reject_reason: *mut c_char,
) -> c_int {
  if let Some(ctx) = MODULE_CONTEXT.as_mut() {
    // TODO: Possibly make this a Player handle instead of an Edict handle
    let entity_handle = EntityHandle::new(&*entity);
    ctx.client_connect(entity_handle);
  }

  1
}

pub unsafe fn client_put_in_server(entity: *mut Edict) {
  if let Some(ctx) = MODULE_CONTEXT.as_mut() {
    // TODO: Possibly make this a Player handle instead of an Edict handle
    let entity_handle = EntityHandle::new(&*entity);
    ctx.client_put_in_server(entity_handle);
  }
}

pub unsafe fn client_put_in_server_post(entity: *mut Edict) {
  if let Some(ctx) = MODULE_CONTEXT.as_mut() {
    // TODO: Possibly make this a Player handle instead of an Edict handle
    let entity_handle = EntityHandle::new(&*entity);
    ctx.client_put_in_server_post(entity_handle);
  }
}

pub unsafe fn client_disconnect(entity: *mut Edict) {
  if let Some(ctx) = MODULE_CONTEXT.as_mut() {
    // TODO: Possibly make this a Player handle instead of an Edict handle
    let entity_handle = EntityHandle::new(&*entity);
    ctx.client_disconnect(entity_handle);
  }
}

pub unsafe fn client_disconnect_post(entity: *mut Edict) {
  if let Some(ctx) = MODULE_CONTEXT.as_mut() {
    // TODO: Possibly make this a Player handle instead of an Edict handle
    let entity_handle = EntityHandle::new(&*entity);
    ctx.client_disconnect_post(entity_handle);
  }
}

// These all are actually `unsafe` because they may only be called
// from the main thread. I'm not sure if I can enforce this in Lua callbacks
// in any way without making it too compilcated with types and lifetimes.
// Let's just make sure we actually call these in the main thread.
// Our main Lua state is always going to execute in the main thread anyway.

pub fn _server_print(message: impl AsRef<str>) {
  if let Ok(msg) = CString::new(message.as_ref()) {
    unsafe {
      ((*ENGINE_FUNCTIONS).server_print)(msg.as_ptr());
    }
  }
}

pub fn string_from_handle(handle: EngineStringHandle) -> String {
  unsafe { CStr::from_ptr(((*ENGINE_FUNCTIONS).sz_from_index)(handle.0)) }
    .to_str()
    .unwrap_or("")
    .into()
}

pub fn handle_from_string(s: impl AsRef<str>) -> EngineStringHandle {
  let s = CString::new(s.as_ref()).unwrap_or_default();
  EngineStringHandle(unsafe {
    ((*ENGINE_FUNCTIONS).alloc_string)(s.as_ptr())
  })
}

pub fn entvars_of_edict(edict: &Edict) -> &EntVars {
  unsafe {
    &*((*ENGINE_FUNCTIONS).get_vars_of_ent)(edict as *const Edict as *mut Edict)
  }
}

pub fn log_console(message: impl AsRef<str>) {
  if let Ok(msg) = CString::new(message.as_ref()) {
    unsafe {
      ((*META_UTIL_FUNCS).log_console)(&PLUGIN_INFO, msg.as_ptr());
    }
  }
}

pub fn log_message(message: impl AsRef<str>) {
  if let Ok(msg) = CString::new(message.as_ref()) {
    unsafe {
      ((*META_UTIL_FUNCS).log_message)(&PLUGIN_INFO, msg.as_ptr());
    }
  }
}

pub fn log_error(message: impl AsRef<str>) {
  if let Ok(msg) = CString::new(message.as_ref()) {
    unsafe {
      ((*META_UTIL_FUNCS).log_error)(&PLUGIN_INFO, msg.as_ptr());
    }
  }
}


pub fn get_meta_plugin_path() -> PathBuf {
  unsafe { CStr::from_ptr(((*META_UTIL_FUNCS).get_plugin_path)(&PLUGIN_INFO)) }
    .to_str()
    .unwrap_or("")
    .into()
}
