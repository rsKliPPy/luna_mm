use std::collections::HashMap;
use std::sync::Arc;
use std::path::Path;
use crate::plugin_sys::plugin::Plugin;
use crate::ffi_wrapper::log_console;

// TODO: Possibly take &rlua::Context in `from_plugin` and create plugin info
// tables in the registry.
#[derive(Clone)]
pub struct PluginInfoHandle {
  pub title: String,
  pub description: String,
  pub version: String,
  pub authors: Vec<String>,
}

pub struct PluginHandle {
  pub identifier: String,
  pub info: PluginInfoHandle,
  pub metadata: HashMap<String, String>,
}


impl PluginHandle {
  pub fn from_plugin(plugin: &Plugin) -> Self {
    let info = &plugin.info();
    PluginHandle {
      identifier: plugin.identifier().to_string(),
      info: PluginInfoHandle {
        title: info.title.clone(),
        description: info.description.clone(),
        version: info.version.clone(),
        authors: info.authors.clone(),
      },
      metadata: plugin.metadata().clone(),
    }
  }
}

impl rlua::UserData for PluginInfoHandle {
  fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("GetTitle", |_, handle: &Self, ()| {
      Ok(handle.title.clone())
    });
    methods.add_method("GetDescription", |_, handle: &Self, ()| {
      Ok(handle.description.clone())
    });
    methods.add_method("GetVersion", |_, handle: &Self, ()| {
      Ok(handle.version.clone())
    });
    methods.add_method("GetAuthors", |_, handle: &Self, ()| {
      Ok(handle.authors.clone())
    });
  }
}

impl rlua::UserData for PluginHandle {
  fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("GetIdentifier", |_, handle: &Self, ()| {
      Ok(handle.identifier.clone())
    });
    methods.add_method("GetInfo", |ctx: rlua::Context, handle: &Self, ()| {
      ctx.create_userdata(handle.info.clone())
    });
    methods.add_method("GetMetadata", |ctx: rlua::Context, handle: &Self, ()| {
      let iter = handle.metadata.iter().map(
        |(k, v)| (k.as_str(), v.as_str())
      );
      ctx.create_table_from(iter)
    });
  }
}

// TODO: Better organize these functions into other modules
pub fn setup_environment<'lua>(
  plugin_directory: &Path,
  current_directory: &Path,
  plugin_key: Arc<rlua::RegistryKey>,
  ctx: &rlua::Context<'lua>,
) -> rlua::Table<'lua> {
  let forbid_index = ctx.create_function(forbid_index).unwrap();
  let env_newindex = forbid_index.clone();

  let plugin_handle: rlua::Value = ctx.registry_value(&*plugin_key).unwrap();
  
  let require = ctx.create_function(
    require_with_context(
      plugin_directory,
      current_directory,
      plugin_key,
    ),
  ).unwrap();

  let env_mt: rlua::Table = ctx.create_table().unwrap();
  env_mt.raw_set("__index", forbid_index).unwrap();
  env_mt.raw_set("__newindex", env_newindex).unwrap();

  let environment: rlua::Table = ctx.create_table().unwrap();
  environment.raw_set("Plugin", plugin_handle).unwrap();
  environment.raw_set("require", require).unwrap();
  environment.set_metatable(Some(env_mt));
  environment
}

pub fn require_with_context(base: &Path, current: &Path, plugin_key: Arc<rlua::RegistryKey>) -> impl Fn(rlua::Context, String) -> rlua::Result<rlua::Table> {
  let base = base.to_path_buf();
  let current = current.to_path_buf();

  move |ctx: rlua::Context, lib: String| {
    let globals = ctx.globals();

    // Load the file, set the environment, execute it and
    // propagate any errors to the parent pcall.
    // TODO: Check whether this file has been loaded already and return
    // the previously cached value.
    if lib.starts_with('/') || lib.starts_with("./") {
      let mut file_path: std::path::PathBuf = if lib.starts_with('/') {
        base.clone()
      } else {
        current.clone()
      };

      file_path.extend(lib.split('/').skip(1));
      file_path.set_extension("lua");

      log_console(format!("File path: {}", file_path.display()));

      let contents = match std::fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) => return Err(rlua::Error::RuntimeError(format!("{}", err))),
      };

      let env = setup_environment(&base, &current, Arc::clone(&plugin_key), &ctx);

      // TODO: Set environment and name
      let value = ctx.load(&contents)
                  .set_environment(env)
                  .unwrap()
                  .call::<_, rlua::Value>(())?;

      Ok(match value {
        rlua::Value::Table(table) => table,
        _ => ctx.create_table().unwrap(),
      })
    } else {
      let libs_table: rlua::Table = globals.raw_get("luna_libs").unwrap();
      libs_table.raw_get(lib)
    }
  }
}

pub fn forbid_index(
  _: rlua::Context,
  params: (rlua::Table, String)
) -> Result<(), rlua::Error> {
  let key = params.1;
  Err(rlua::Error::RuntimeError(format!("Attempt to access global {}", key)))
}

pub fn print_to_console(_: rlua::Context, message: String) -> rlua::Result<()> {
  log_console(message);
  Ok(())
}
