use std::sync::{Arc, Mutex};
use crate::plugin_sys::PluginSystem;
use crate::global_state::GlobalState;
use crate::ffi_wrapper::{
  MetaContext,
  get_meta_plugin_path,
  hl_lua_bridge::EntityHandle,
};

struct ModuleContext {
  state: Arc<Mutex<GlobalState>>,
  plugin_system: PluginSystem,
}

impl ModuleContext {
  pub fn new() -> Self {
    let state = Arc::new(Mutex::new(GlobalState::new()));
    
    let path = get_meta_plugin_path();
    let mut pl_dir = path.parent().unwrap().parent().unwrap().to_path_buf();
    pl_dir.push("Plugins");

    let plugin_sys = PluginSystem::mount(pl_dir, state.clone());
    plugin_sys.run_plugins();

    ModuleContext {
      state: state,
      plugin_system: plugin_sys,
    }
  }
}

impl MetaContext for ModuleContext {
  fn client_connect(&mut self, entity: EntityHandle) {
    self.plugin_system.lua().context(|ctx: rlua::Context| {
      let state = self.state.lock().unwrap();
      let _ = state.listeners.emit(&ctx, "ClientConnect", entity);
    });
  }

  fn client_put_in_server(&mut self, entity: EntityHandle) {
    self.plugin_system.lua().context(|ctx: rlua::Context| {
      let state = self.state.lock().unwrap();
      let _ = state.listeners.emit(&ctx, "PreClientPutInServer", entity);
    });
  }

  fn client_disconnect(&mut self, entity: EntityHandle) {
    self.plugin_system.lua().context(|ctx: rlua::Context| {
      let state = self.state.lock().unwrap();
      let _ = state.listeners.emit(&ctx, "ClientDisconnect", entity);
    });
  }

  fn client_put_in_server_post(&mut self, entity: EntityHandle) {
    self.plugin_system.lua().context(|ctx: rlua::Context| {
      let state = self.state.lock().unwrap();
      let _ = state.listeners.emit(&ctx, "ClientPutInServer", entity);
    });
  }

  fn client_disconnect_post(&mut self, entity: EntityHandle) {
    self.plugin_system.lua().context(|ctx: rlua::Context| {
      let state = self.state.lock().unwrap();
      let _ = state.listeners.emit(&ctx, "ClientDisconnected", entity);
    });
  }
}

pub fn module_init() -> Box<dyn MetaContext> {
  Box::new(ModuleContext::new())
}

pub fn module_shutdown(ctx: Box<dyn MetaContext>) -> () {
  drop(ctx)
}


