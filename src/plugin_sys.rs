mod luna_lib;
pub mod plugin;

use std::collections::HashSet;
use std::path::{PathBuf, Path};
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
  let lib_core = ctx.create_table().unwrap();
  let print_to_console = ctx.create_function(core::print_to_console).unwrap();
  lib_core.raw_set("PrintToConsole", print_to_console).unwrap();

  libs.raw_set("Luna/Core", lib_core).unwrap();
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

  // lua.context(|ctx: rlua::Context| {
  //   let globals = ctx.globals();
  //   let lib_coroutine: rlua::Table = globals.get("coroutine").unwrap();
  //   let lib_table: rlua::Table = globals.get("table").unwrap();
  //   let lib_io: rlua::Table = globals.get("io").unwrap();
  //   let lib_os: rlua::Table = globals.get("os").unwrap();
  //   let lib_string: rlua::Table = globals.get("string").unwrap();
  //   let lib_utf8: rlua::Table = globals.get("utf8").unwrap();
  //   let lib_math: rlua::Table = globals.get("math").unwrap();
  //   let lib_package: rlua::Table = globals.get("package").unwrap();
  // });

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
    let chunk: rlua::Chunk = ctx.load(&main_contents)
                .set_name(plugin.identifier())
                .unwrap()
                .set_environment(Self::setup_environment(&plugin, &ctx))
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

  fn setup_environment<'lua>(
    plugin: &Plugin,
    ctx: &rlua::Context<'lua>,
  ) -> rlua::Table<'lua> {
    let forbid_index = ctx.create_function(core::forbid_index).unwrap();
    let env_newindex = forbid_index.clone();
    
    let pl_info = core::PluginHandle::from_plugin(&plugin);
    let require = ctx.create_function(core::require).unwrap();

    let env_mt: rlua::Table = ctx.create_table().unwrap();
    env_mt.raw_set("__index", forbid_index).unwrap();
    env_mt.raw_set("__newindex", env_newindex).unwrap();

    let environment: rlua::Table = ctx.create_table().unwrap();
    environment.raw_set("Plugin", pl_info).unwrap();
    environment.raw_set("require", require).unwrap();
    environment.raw_set("_G", environment.clone()).unwrap();
    environment.set_metatable(Some(env_mt));
    environment
  }

  fn add_to_plugin_lib<'lua>(
    pl: &Plugin,
    ctx: &rlua::Context<'lua>,
    lib: &rlua::Table<'lua>
  ) {
    let globals = ctx.globals();
    let libs: rlua::Table = globals.get("luna_libs").unwrap();
    let plugin_lib: rlua::Table = libs.get(pl.identifier()).unwrap();
    lib.clone()
      .pairs::<rlua::Value, rlua::Value>()
      .map(Result::unwrap)
      .for_each(|(k, v)| plugin_lib.raw_set(k, v).unwrap());
  }
}
