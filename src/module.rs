use crate::plugin_sys::PluginSystem;
use crate::ffi_wrapper::{
  ModuleContext,
  get_plugin_path,
  init_module_context,
  destroy_module_context,
};

pub fn module_init() -> () {
  init_module_context(|| {
    let path = get_plugin_path();
    let mut pl_dir = path.parent().unwrap().parent().unwrap().to_path_buf();
    pl_dir.push("Plugins");

    let plugin_sys = PluginSystem::mount(pl_dir);
    plugin_sys.run_plugins();
  
    ModuleContext {
      plugin_path: path,
      plugin_sys: plugin_sys,
    }
  });
}

pub fn module_shutdown() -> () {
  destroy_module_context();
}


