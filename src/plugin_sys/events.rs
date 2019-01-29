use std::collections::HashMap;

pub struct LuaEventEmitter {
  handlers: HashMap<String, Vec<rlua::RegistryKey>>,
}

impl LuaEventEmitter {
  pub fn new() -> Self {
    LuaEventEmitter {
      handlers: HashMap::new(),
    }
  }

  pub fn add_listener<'lua>(&mut self, ctx: &rlua::Context<'lua>, event_name: &str, func: rlua::Function<'lua>) {
    if self.listener_exists(ctx, event_name, &func).is_none() {
      let key = ctx.create_registry_value(func).unwrap();
      let listeners = self.handlers.entry(event_name.to_string()).or_default();
      listeners.push(key);
    }
  }

  pub fn remove_listener<'lua>(&mut self, ctx: &rlua::Context<'lua>, event_name: &str, func: rlua::Function<'lua>) {
    if let Some(idx) = self.listener_exists(ctx, event_name, &func) {
      if let Some(listeners) = self.handlers.get_mut(event_name) {
        listeners.remove(idx);
      }
    }
  }

  pub fn emit<'lua, TParams>(&self, ctx: &rlua::Context<'lua>, event_name: &str, params: TParams) where TParams: rlua::ToLuaMulti<'lua> + Clone {
    if let Some(listeners) = self.handlers.get(event_name) {
      listeners
        .iter()
        .map(|key| ctx.registry_value::<rlua::Function>(key).unwrap())
        .for_each(|f: rlua::Function| f.call(params.clone()).unwrap())
    }
  }

  fn listener_exists<'lua>(&self, ctx: &rlua::Context<'lua>, event_name: &str, func: &rlua::Function<'lua>) -> Option<usize> {
    // Currently there's no way to `rawequal` two Lua values
    // so we use this probably extremely slow method
    // TODO: Change when a method to check equality between
    // two Lua values becomes available.
    let expr = "function(l, r) return rawequal(l, r) end";
    let eq_func: rlua::Function<'lua> = ctx.load(expr).eval().unwrap();
    let funcs_equal = |l: &rlua::Function<'lua>, r: &rlua::Function<'lua>| {
      eq_func.call::<_, bool>((l.clone(), r.clone())).unwrap()
    };
    
    let listeners_vec = self.handlers.get(event_name);

    if let Some(listeners_vec) = listeners_vec {
      listeners_vec
        .iter()
        .map(|f| ctx.registry_value::<rlua::Function>(f))
        .map(Result::unwrap)
        .position(|f| funcs_equal(&f, &func))
    } else {
      None
    }
  }
}
