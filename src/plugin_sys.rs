mod luna_lib;
pub mod plugin;
pub mod events;

use std::collections::HashSet;
use std::path::{PathBuf, Path};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::io;
use std::fs;
use crate::ffi_wrapper::{log_error, log_message};
use crate::global_state::{GlobalState, GlobalStateUserData};
use crate::lua_helpers;
use self::plugin::Plugin;
use self::luna_lib::{core, listeners};


pub fn get_identifier_from_path(dir: &Path) -> String {
  // TODO: Return result, don't unwrap
  let ident = dir.file_name().unwrap()
              .to_os_string().into_string().unwrap();
  let namespace = dir.parent().unwrap()
                  .file_name().unwrap()
                  .to_os_string().into_string().unwrap();
  
  format!("{}/{}", namespace, ident)
}

fn load_plugin_from_dir(
  base_dir: &Path,
  dir: &Path,
  identifier: &str,
  visited: &mut HashSet<String>,
  loaded: &mut Vec<Plugin>,
) -> Result<(), Box<dyn Error>> {
  if visited.contains(identifier) {
    return Ok(());
  }
  
  visited.insert(identifier.to_string());
  
  if !dir.exists() {
    return Err(Box::new(io::Error::new(
      io::ErrorKind::NotFound,
      format!("Plugin directory {} doesn't exist.", dir.display()),
    )));
  }

  let plugin = Plugin::load_plugin(dir, &identifier)?;
  for dep in plugin.dependencies() {
    let dep_ident: &String = dep.0;
    let _dep_version = dep.1; // TODO: Check semver

    let mut dep_dir = base_dir.to_path_buf();
    dep_dir.extend(dep_ident.split('/'));
    load_plugin_from_dir(base_dir, &dep_dir, dep_ident, visited, loaded)?;
  } 

  loaded.push(plugin);
  Ok(())
}

fn load_plugins(dir: &Path) -> Vec<Plugin> {
  let mut plugins = Vec::new();
  let mut visited_plugins = HashSet::new();

  let ns_iter = match fs::read_dir(&dir) {
    Ok(iter) => iter,
    Err(err) => {
      // TODO: Add logging
      log_error(format!("Couldn't load from \"{}\": {}", dir.display(), err));
      return plugins;
    },
  };

  ns_iter
    .filter_map(Result::ok)
    .map(|entry| fs::read_dir(entry.path()))
    .filter_map(Result::ok)
    .flat_map(|x| x)
    .filter_map(Result::ok)
    .for_each(|entry| {
      let ident = get_identifier_from_path(&entry.path());
      let load_result = load_plugin_from_dir(
        &dir,
        &entry.path(),
        &ident,
        &mut visited_plugins,
        &mut plugins,
      );

      if let Err(e) = load_result {
        log_error(format!("Couldn't load \"{}\": {}", ident, e));
      }
    });

  log_message(format!("Loaded {} plugins.", plugins.len()));

  plugins
}

fn init_luna_libs<'lua>(
  libs: &rlua::Table<'lua>,
  ctx: &rlua::Context<'lua>,
) {
  fn replace_string_metatable<'lua>(
    ctx: &rlua::Context<'lua>,
    mt: rlua::Table<'lua>,
  ) {
    // This is hacky as hell but seems like it's the only way to change
    // the string type metatable in rlua.
    let str_object: rlua::String = ctx.create_string("").unwrap();
    let str_object: rlua::Table = unsafe { std::mem::transmute(str_object) };
    let str_mt: rlua::Table = str_object.get_metatable().unwrap();
    let str_mt: rlua::Table = str_mt.raw_get("__index").unwrap();

    // Empty the old string type metatable
    str_mt.clone()
      .pairs::<rlua::Value, rlua::Value>()
      .map(Result::unwrap)
      .for_each(|(k, _)| str_mt.raw_set(k, rlua::Nil).unwrap());

    // Add new methods
    mt.clone()
      .pairs::<rlua::Value, rlua::Value>()
      .map(Result::unwrap)
      .for_each(|(k, v)| str_mt.raw_set(k, v).unwrap());
  }

  let globals = ctx.globals();
  let lib_core: rlua::Table = ctx.create_table().unwrap();
  let lib_table: rlua::Table = ctx.create_table().unwrap();
  let lib_string: rlua::Table = ctx.create_table().unwrap();
  let lib_listeners: rlua::Table = ctx.create_table().unwrap();

  ////////// Re-map old functions to new names //////////

  // Core
  lua_helpers::map_funcs(
    &globals,
    &lib_table,
    &["tonumber", "tostring"],
    &["ToNumber", "ToString"],
  );

  // Table
  let orig_lib_table: rlua::Table = globals.raw_get("table").unwrap();
  lua_helpers::map_funcs(
    &orig_lib_table,
    &lib_table,
    &["concat", "insert", "pack", "remove", "sort", "unpack"],
    &["Concat", "Insert", "Pack", "Remove", "Sort", "Unpack"],
  );
  lua_helpers::map_funcs(
    &globals,
    &lib_table,
    &["ipairs", "pairs"],
    &["IPairs", "Pairs"],
  );

  // String
  let orig_lib_string: rlua::Table = globals.raw_get("string").unwrap();
  let old = [
    "find", "format", "gmatch", "gsub", "len", "lower",
    "match", "rep", "reverse", "sub", "upper"
  ];
  let new = [
    "Find", "Format", "GlobalMatch", "GlobalReplace", "Length",
    "ToLower", "Match", "Repeat", "Reverse", "SubString", "ToUpper"
  ];
  lua_helpers::map_funcs(&orig_lib_string, &lib_string, &old, &new);
  replace_string_metatable(ctx, lib_string.clone());

  ////////// New functions //////////
  
  // Core
  let print_to_console = ctx.create_function(core::print_to_console).unwrap();
  lib_core.raw_set("PrintToConsole", print_to_console).unwrap();

  // Listeners
  let enum_values = [
    "ClientConnect", "PreClientPutInServer", "ClientPutInServer", "ClientDisconnect", "ClientDisconnected",
    "PluginsLoaded", "PluginsWillUnload", "PluginsUnload",
  ];
  let events_enum = ctx.create_table_from(
    enum_values.iter().map(|&x| x).zip(enum_values.iter().map(|&x| x))
  ).unwrap();

  let add_listener = ctx.create_function(listeners::add_listener).unwrap();
  let remove_listener = ctx.create_function(listeners::remove_listener).unwrap();
  lib_listeners.raw_set("On", add_listener).unwrap();
  lib_listeners.raw_set("Off", remove_listener).unwrap();
  lib_listeners.raw_set("Events", events_enum).unwrap();

  libs.raw_set("Luna/Core", lib_core).unwrap();
  libs.raw_set("Luna/Table", lib_table).unwrap();
  libs.raw_set("Luna/String", lib_string).unwrap();
  libs.raw_set("Luna/Listeners", lib_listeners).unwrap();
}

fn init_plugin_libs<'lua>(
  libs: &rlua::Table<'lua>,
  plugins: &Vec<Plugin>,
  ctx: &rlua::Context<'lua>
) {
  for plugin in plugins {
    let lib = ctx.create_table().unwrap();
    libs.raw_set(plugin.identifier(), lib).unwrap();
  }
}

fn init_libs(plugins: &Vec<Plugin>, ctx: &rlua::Context) {
  let globals = ctx.globals();
  let libs_table = ctx.create_table().unwrap();
  init_plugin_libs(&libs_table, &plugins, &ctx);
  init_luna_libs(&libs_table, &ctx);
  globals.raw_set("luna_libs", libs_table).unwrap();
}

fn setup_lua_state(state: Arc<Mutex<GlobalState>>) -> rlua::Lua {
  let lua = rlua::Lua::new();

  lua.context(|ctx: rlua::Context| {
    let globals = ctx.globals();

    let global_state = GlobalStateUserData(state);
    globals.raw_set("luna_global_state", global_state).unwrap();

    globals.raw_set("luna_call_level", 0).unwrap();
  });
  
  lua
}


pub struct PluginSystem {
  plugins: Vec<Plugin>,
  lua: rlua::Lua,
}

impl PluginSystem {
  pub fn mount(directory: impl Into<PathBuf>, state: Arc<Mutex<GlobalState>>) -> Self {
    let directory = directory.into();
    let plugins = load_plugins(&directory);
    let lua = setup_lua_state(state);
    lua.context(|ctx| init_libs(&plugins, &ctx));
    
    PluginSystem {
      plugins: plugins,
      lua: lua,
    }
  }

  pub fn run_plugins(&self) {
    let mut visited = HashSet::new();

    for pl in &self.plugins {
      self.lua.context(|ctx: rlua::Context| {
        Self::run_plugin(&pl, &ctx, &mut visited);
      });
    }

    self.lua.context(|ctx: rlua::Context| {
      let globals = ctx.globals();
      let state: GlobalStateUserData = globals.get("luna_global_state").unwrap();
      let state = state.0.lock().unwrap();
      let _ = state.listeners.emit(&ctx, "PluginsLoaded", ());
    });
  }

  pub fn lua(&self) -> &rlua::Lua {
    &self.lua
  }

  fn run_plugin<'a>(
    plugin: &'a Plugin,
    ctx: &rlua::Context,
    visited: &mut HashSet<&'a str>,
  ) {
    visited.insert(plugin.identifier());

    let main_contents = fs::read_to_string(plugin.main_source_path()).unwrap();
    let plugin_handle = Arc::new(ctx.create_registry_value(core::PluginHandle::from_plugin(&plugin)).unwrap());
    let env = core::setup_environment(plugin.directory(), plugin.directory(), plugin_handle, &ctx);
    let name = format!("{}::Plugin.lua", plugin.identifier());
    let chunk = ctx.load(&main_contents)
                .set_name(&name).unwrap()
                .set_environment(env).unwrap()
                .into_function();

    let chunk: rlua::Function = match chunk {
      Ok(chunk) => chunk,
      Err(err) => {
        lua_helpers::print_lua_error(&err);
        return;
      }
    };

    let result = lua_helpers::call_lua::<_, rlua::Value>(&ctx, &chunk, ());
    if let Ok(rlua::Value::Table(table)) = result {
      Self::add_to_plugin_lib(&plugin, &ctx, &table);
    }
  }

  fn add_to_plugin_lib<'lua>(
    pl: &Plugin,
    ctx: &rlua::Context<'lua>,
    lib: &rlua::Table<'lua>
  ) {
    let globals = ctx.globals();
    let libs: rlua::Table = globals.raw_get("luna_libs").unwrap();
    let plugin_lib: rlua::Table = libs.raw_get(pl.identifier()).unwrap();
    lib.clone()
      .pairs::<rlua::Value, rlua::Value>()
      .map(Result::unwrap)
      .for_each(|(k, v)| plugin_lib.raw_set(k, v).unwrap());
  }
}

impl Drop for PluginSystem {
  fn drop(&mut self) {
    self.lua.context(|ctx: rlua::Context| {
      let globals = ctx.globals();
      let state: GlobalStateUserData = globals.get("luna_global_state").unwrap();
      let state = state.0.lock().unwrap();
      let _ = state.listeners.emit(&ctx, "PluginsWillUnload", ());
      let _ = state.listeners.emit(&ctx, "PluginsUnload", ());
    });
  }
}
