use std::os::raw::{c_char, c_int};
use crate::meta_ffi::globals;
use crate::plugin_info::PLUGIN_INFO;
use crate::meta_ffi::util::set_meta_result;
use crate::meta_ffi::constant::{
  DLL_INTERFACE_VERSION,
  NEW_DLL_INTERFACE_VERSION,
  ENGINE_INTERFACE_VERSION,
};
use crate::meta_ffi::types::{
  PluginInfo,
  LoadTime,
  UnloadReason,
  EngineFunctions,
  GlobalVars,
  MetaUtilFuncs,
  MetaGlobals,
  DLLFunctions,
  NewDLLFunctions,
  GameDLLFunctions,
  MetaFunctions,
  MetaResult,
};


#[export_name = "GiveFnptrsToDll"]
pub unsafe extern fn give_fnptrs_to_dll(
  funcs: *const EngineFunctions,
  globals: *mut GlobalVars,
) {
  globals::ENGINE_FUNCTIONS = funcs;
  globals::GLOBAL_VARS = globals;
}

#[export_name = "Meta_Init"]
pub unsafe extern fn meta_init() {
  
}

#[export_name = "Meta_Query"]
pub unsafe extern fn meta_query(
  _if_version: *const c_char,
  plugin_info: *mut *const PluginInfo,
  meta_util_funcs: *const MetaUtilFuncs,
) -> c_int {
  // TODO: Check interface version
  
  *plugin_info = &PLUGIN_INFO;
  globals::META_UTIL_FUNCS = meta_util_funcs;
  
  1
}

#[export_name = "Meta_Attach"]
pub unsafe extern fn meta_attach(
  now: LoadTime,
  meta_funcs_table: *mut MetaFunctions,
  meta_globals: *mut MetaGlobals,
  gamedll_funcs: *const GameDLLFunctions,
) -> c_int {
  if now > PLUGIN_INFO.loadable {
    // TODO: Notify
    return 0;
  }

  globals::META_GLOBALS = meta_globals;
  globals::GAME_DLL_FUNCTIONS = gamedll_funcs;

  *meta_funcs_table = MetaFunctions {
    get_entity_api2: Some(get_entity_api2),
    get_entity_api2_post: Some(get_entity_api2_post),
    get_new_dll_functions: Some(get_new_dll_functions),
    get_new_dll_functions_post: Some(get_new_dll_functions_post),
    get_engine_functions: Some(get_engine_functions),
    get_engine_functions_post: Some(get_engine_functions_post),
    ..Default::default()
  };

  1
}

#[export_name = "Meta_Detach"]
pub unsafe extern fn meta_detach(
  now: LoadTime,
  reason: UnloadReason
) -> c_int {
  if now > PLUGIN_INFO.unloadable && reason != UnloadReason::CommandForced {
    // TODO: Notify
    return 0;
  }

  1
}

unsafe extern fn get_entity_api2(
  funcs: *mut DLLFunctions,
  if_vers: *mut c_int,
) -> c_int {
  if *if_vers != DLL_INTERFACE_VERSION {
    // TODO: Notify
    *if_vers = DLL_INTERFACE_VERSION;
    return 0;
  }

  globals::DLL_HOOK_TABLE = funcs;

  (*globals::DLL_HOOK_TABLE).game_init = game_init;

  1
}

unsafe extern fn get_entity_api2_post(
  funcs: *mut DLLFunctions,
  if_vers: *mut c_int,
) -> c_int {
  if *if_vers != DLL_INTERFACE_VERSION {
    // TODO: Notify
    *if_vers = DLL_INTERFACE_VERSION;
    return 0;
  }

  globals::DLL_HOOK_TABLE_POST = funcs;

  1
}

unsafe extern fn get_new_dll_functions(
  funcs: *mut NewDLLFunctions,
  if_vers: *mut c_int,
) -> c_int {
  if *if_vers != NEW_DLL_INTERFACE_VERSION {
    // TODO: Notify
    *if_vers = NEW_DLL_INTERFACE_VERSION;
    return 0;
  }

  globals::NEWDLL_HOOK_TABLE = funcs;

  1
}

unsafe extern fn get_new_dll_functions_post(
  funcs: *mut NewDLLFunctions,
  if_vers: *mut c_int,
) -> c_int {
  if *if_vers != NEW_DLL_INTERFACE_VERSION {
    // TODO: Notify
    *if_vers = NEW_DLL_INTERFACE_VERSION;
    return 0;
  }

  globals::NEWDLL_HOOK_TABLE_POST = funcs;

  (*globals::NEWDLL_HOOK_TABLE_POST).game_shutdown = game_shutdown;

  1
}

unsafe extern fn get_engine_functions(
  funcs: *mut EngineFunctions,
  if_vers: *mut c_int,
) -> c_int {
  if *if_vers != ENGINE_INTERFACE_VERSION {
    // TODO: Notify
    *if_vers = ENGINE_INTERFACE_VERSION;
    return 0;
  }

  globals::ENGINE_HOOK_TABLE = funcs;

  1
}

unsafe extern fn get_engine_functions_post(
  funcs: *mut EngineFunctions,
  if_vers: *mut c_int,
) -> c_int {
  if *if_vers != ENGINE_INTERFACE_VERSION {
    // TODO: Notify
    *if_vers = ENGINE_INTERFACE_VERSION;
    return 0;
  }

  globals::ENGINE_HOOK_TABLE_POST = funcs;

  1
}

unsafe extern fn game_init() {
  crate::module::module_init();
  set_meta_result(MetaResult::Ignored)
}

unsafe extern fn game_shutdown() {
  crate::module::module_shutdown();
  set_meta_result(MetaResult::Ignored)
}
