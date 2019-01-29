use crate::global_state::GlobalStateUserData;

pub fn add_listener<'lua>(
  ctx: rlua::Context<'lua>,
  params: (String, rlua::Function<'lua>)
) -> Result<(), rlua::Error> {
  let event_name = params.0;
  let listener = params.1;

  let globals = ctx.globals();
  let state: GlobalStateUserData = globals.get("luna_global_state").unwrap();
  let mut state = state.0.lock().unwrap();
  state.listeners.add_listener(&ctx, &event_name, listener);

  Ok(())
}

pub fn remove_listener<'lua>(
  ctx: rlua::Context<'lua>,
  params: (String, rlua::Function<'lua>)
) -> Result<(), rlua::Error> {
  let event_name = params.0;
  let listener = params.1;

  let globals = ctx.globals();
  let state: GlobalStateUserData = globals.get("luna_global_state").unwrap();
  let mut state = state.0.lock().unwrap();
  state.listeners.remove_listener(&ctx, &event_name, listener);

  Ok(())
}
