mod luna_lib;
pub mod plugin;

use std::collections::HashSet;
use std::path::{PathBuf, Path};
use std::sync::Arc;
use std::error::Error;
use std::io;
use std::fs;
use crate::ffi_wrapper::{log_error, log_message};
use self::plugin::Plugin;
use self::luna_lib::core;

fn get_identifier_from_path(dir: &Path) -> String {
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
      log_error(format!("Couldn't load from \"{}\": {}", dir.display(), err));
      return plugins;
    },
  };

  for ns_entry in ns_iter {
    let ns_entry = match ns_entry {
      Ok(e) => e,
      Err(_) => continue,
    };
    
    let pl_iter = match fs::read_dir(ns_entry.path()) {
      Ok(iter) => iter,
      Err(_) => continue,
    };

    for pl_entry in pl_iter {
      let pl_entry = match pl_entry {
        Ok(e) => e,
        Err(_) => continue,
      };

      let identifier = get_identifier_from_path(&pl_entry.path());
      let load_result = load_plugin_from_dir(
        &dir,
        &pl_entry.path(),
        &identifier,
        &mut visited_plugins,
        &mut plugins,
      );

      match load_result {
        Err(e) => {
          log_error(format!("Couldn't load \"{}\": {}", identifier, e));
          continue;
        }
        _ => { }
      }
    }
  }

  log_message(format!("Loaded {} plugins.", plugins.len()));

  plugins
}

fn init_luna_libs<'lua>(
  libs: &rlua::Table<'lua>,
  ctx: &rlua::Context<'lua>,
) {
  fn map_funcs<'lua>(
    old_table: &rlua::Table<'lua>,
    new_table: &rlua::Table<'lua>,
    old_keys: &[&str],
    new_keys: &[&str],
  ) {
    old_keys.iter().zip(new_keys.iter()).for_each(|(old, new)| {
      new_table.raw_set(
        *new,
        old_table.raw_get::<_, rlua::Value>(*old).unwrap()
      ).unwrap();
    });
  }

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

  // Core
  let print_to_console = ctx.create_function(core::print_to_console).unwrap();
  lib_core.raw_set("PrintToConsole", print_to_console).unwrap();
  map_funcs(
    &globals,
    &lib_table,
    &["tonumber", "tostring"],
    &["ToNumber", "ToString"],
  );

  // Table
  let orig_lib_table: rlua::Table = globals.raw_get("table").unwrap();
  map_funcs(
    &orig_lib_table,
    &lib_table,
    &["concat", "insert", "pack", "remove", "sort", "unpack"],
    &["Concat", "Insert", "Pack", "Remove", "Sort", "Unpack"],
  );
  map_funcs(
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
  map_funcs(&orig_lib_string, &lib_string, &old, &new);
  replace_string_metatable(ctx, lib_string.clone());

  libs.raw_set("Luna/Core", lib_core).unwrap();
  libs.raw_set("Luna/Table", lib_table).unwrap();
  libs.raw_set("Luna/String", lib_string).unwrap();
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

fn setup_lua_state() -> rlua::Lua {
  let lua = rlua::Lua::new();
  lua
}


pub struct PluginSystem {
  directory: PathBuf,
  plugins: Vec<Plugin>,
  lua: rlua::Lua,
}

impl PluginSystem {
  pub fn mount(directory: impl Into<PathBuf>) -> Self {
    let directory = directory.into();
    let plugins = load_plugins(&directory);
    let lua = setup_lua_state();
    lua.context(|ctx| init_libs(&plugins, &ctx));
    
    PluginSystem {
      directory: directory.clone(),
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
    let chunk: rlua::Chunk = ctx.load(&main_contents)
                .set_name(plugin.identifier())
                .unwrap()
                .set_environment(env)
                .unwrap();

    match chunk.call::<_, rlua::Value>(()) {
      Err(err) => {
        match err {
          rlua::Error::RuntimeError(msg) => {
            log_error(format!("Runtime error - {}", msg));
          },
          rlua::Error::CallbackError{ traceback, cause } => {
            log_error(format!("Lua callback error - {}", cause));
            traceback.lines().for_each(log_error);
          }
          _ => log_error(format!("Lua error - {}", err)),
        };
      }
      Ok(rlua::Value::Table(table)) => {
        Self::add_to_plugin_lib(&plugin, &ctx, &table);
      }
      _ => { }
    };
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
