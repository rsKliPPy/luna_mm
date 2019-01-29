use std::sync::{Arc, Mutex};
use crate::plugin_sys::events::LuaEventEmitter;

pub struct GlobalState {
  pub listeners: LuaEventEmitter,
}

impl GlobalState {
  pub fn new() -> Self {
    GlobalState {
      listeners: LuaEventEmitter::new(),
    }
  }
}

// So that we can keep this in the lua state and access it at any time
#[derive(Clone)]
pub struct GlobalStateUserData(pub Arc<Mutex<GlobalState>>);
impl rlua::UserData for GlobalStateUserData { }
