use crate::ffi_wrapper::log_error;

pub fn map_funcs<'lua>(
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

pub fn print_lua_error(error: &rlua::Error) {
  // TODO: Add logging
  match error {
    rlua::Error::RuntimeError(msg) => log_error(format!("{}", msg)),
    rlua::Error::CallbackError{ traceback, cause } => {
      log_error(format!("{}", cause));
      traceback.lines().for_each(log_error);
    }
    _ => log_error(format!("{}", error)),
  }
}

pub fn call_lua<'lua, TParams, TReturn>(
  ctx: &rlua::Context<'lua>,
  func: &rlua::Function<'lua>,
  params: TParams,
) -> rlua::Result<TReturn>
where
  TParams: rlua::ToLuaMulti<'lua>,
  TReturn: rlua::FromLuaMulti<'lua>,
{
  let globals = ctx.globals();
  let call_level: usize = globals.raw_get("luna_call_level").unwrap();
  globals.raw_set("luna_call_level", call_level + 1).unwrap();

  let result = func.call(params);

  globals.raw_set("luna_call_level", call_level).unwrap();

  // If we are at the bottom of the call stack, print the error
  if call_level == 0 {
    if let Err(err) = &result {
      print_lua_error(&err);
    }
  }

  result
}
