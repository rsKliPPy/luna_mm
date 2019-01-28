use std::collections::HashMap;
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

pub fn forbid_index(
  _: rlua::Context,
  params: (rlua::Table, String)
) -> Result<(), rlua::Error> {
  let key = params.1;
  Err(rlua::Error::RuntimeError(format!("Tried to access global {}", key)))
}

// TODO: Let this be a callable per-plugin userdatum?
pub fn require(ctx: rlua::Context, lib: String) -> rlua::Result<rlua::Table> {
  let globals = ctx.globals();
  let libs_table: rlua::Table = globals.raw_get("luna_libs").unwrap();
  libs_table.raw_get(lib)
}

pub fn print_to_console(_: rlua::Context, message: String) -> rlua::Result<()> {
  log_console(message);
  Ok(())
}
