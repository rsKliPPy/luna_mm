use std::os::raw::{
  c_char, c_void, c_int, c_short, c_float, c_uchar as c_byte
};


#[repr(C)]
#[derive(Clone, Copy)]
pub enum MetaResult {
  Unset = 0,
  Ignored,
  Handled,
  Override,
  Supercede,
}

#[repr(C)]
pub struct MetaGlobals {
  pub result: MetaResult,
  pub prev_result: MetaResult,
  pub status: MetaResult,
  pub orig_ret: *const c_void,
  pub override_ret: *const c_void,
}

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum LoadTime {
  Never = 0,
  StartUp,
  ChangeLevel,
  AnyTime,
  AnyPause,
}

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum UnloadReason {
  Null = 0,
  IniDeleted,
  FileNewer,
  Command,
  CommandForced,
  Delayed,
  Plugin,
  PluginForced,
  Reload,
}

#[repr(C)]
pub struct PluginInfo {
  pub if_version: *const c_char,
  pub name: *const c_char,
  pub version: *const c_char,
  pub date: *const c_char,
  pub author: *const c_char,
  pub url: *const c_char,
  pub logtag: *const c_char,
  pub loadable: LoadTime,
  pub unloadable: LoadTime,
}

unsafe impl Sync for PluginInfo { }

// #[repr(C)]
// #[derive(Clone, Copy)]
// pub enum AlertType {
//   Notice = 0,
//   Console,
//   AIConsole,
//   Warning,
//   Error,
//   Logged,
// }

// #[repr(C)]
// #[derive(Clone, Copy)]
// pub enum PrintType {
//   Console = 0,
//   Center,
//   Chat,
// }

// #[repr(C)]
// #[derive(Clone, Copy)]
// pub enum ForceType {
//   ExactFile = 0,
//   ModelSameBounds,
//   ModelSpecifyBounds,
//   ModelSpecifyBoundsIfAvailable,
// }

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EngineStringHandle(pub c_int);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EngineVector3(c_float, c_float, c_float);

#[repr(C)]
pub struct Edict {
  free: c_int,
  serial_number: c_int,
  link: [*mut c_void; 2],
  head_node: c_int,
  num_leafs: c_int,
  leafnums: [c_short; 48],
  free_time: c_float,
  private_data: *mut c_void,
  entvars: EntVars,
}

impl Edict {
  pub fn is_free(&self) -> bool {
    self.free != 0
  }

  pub fn serial_number(&self) -> c_int {
    self.serial_number
  }

  pub fn private_data(&self) -> *mut c_void {
    self.private_data
  }
}

#[repr(C)]
pub struct EntVars {
  pub classname: EngineStringHandle,
  pub globalname: EngineStringHandle,

  pub origin: EngineVector3,
  pub oldorigin: EngineVector3,
  pub velocity: EngineVector3,
  pub basevelocity: EngineVector3,
  pub clbasevelocity: EngineVector3,
  pub movedir: EngineVector3,
  pub angles: EngineVector3,
  pub avelocity: EngineVector3,
  pub punchangle: EngineVector3,
  pub v_angle: EngineVector3,

  pub endpos: EngineVector3,
  pub startpos: EngineVector3,
  pub impacttime: c_float,
  pub starttime: c_float,

  pub fixangle: c_int,
  pub idealpitch: c_float,
  pub pitch_speed: c_float,
  pub ideal_yaw: c_float,
  pub yaw_speed: c_float,

  pub modelindex: c_int,
  pub model: EngineStringHandle,

  pub viewmodel: c_int,
  pub weaponmodel: c_int,

  pub absmin: EngineVector3,
  pub absmax: EngineVector3,
  pub mins: EngineVector3,
  pub maxs: EngineVector3,
  pub size: EngineVector3,

  pub ltime: c_float,
  pub nextthink: c_float,

  pub movetype: c_int, // TODO: Create a `MoveType` enum
  pub solid: c_int, // TODO: Create a `Solidity` enum

  pub skin: c_int,
  pub body: c_int,
  pub effects: c_int,

  pub gravity: c_float,
  pub friction: c_float,

  pub light_level: c_int,

  pub sequence: c_int,
  pub gaitsequence: c_int,
  pub frame: c_float,
  pub animtime: c_float,
  pub framerate: c_float,
  pub controller: [c_byte; 4],
  pub blending: [c_byte; 2],

  pub scale: c_float,

  pub rendermode: c_int, // TODO: Create a `RenderMode` enum
  pub renderamount: c_float,
  pub rendercolor: EngineVector3,
  pub renderfx: c_int, // TODO: Create a `RenderFX` enum

  pub health: c_float,
  pub frags: c_float,
  pub weapons: c_int,
  pub takedamage: c_float,

  pub deadflag: c_int,
  pub view_ofs: EngineVector3,

  pub button: c_int,
  pub impulse: c_int,

  pub chain: *mut Edict,
  pub dmg_inflictor: *mut Edict,
  pub enemy: *mut Edict,
  pub aiment: *mut Edict,
  pub owner: *mut Edict,
  pub groundentity: *mut Edict,

  pub spawnflags: c_int,
  pub flags: c_int, // TODO: Possibly create a `EdictFlags` enum

  pub colormap: c_int,
  pub team: c_int,

  pub max_health: c_float,
  pub teleport_time: c_float,
  pub armortype: c_float,
  pub armorvalue: c_float,
  pub waterlevel: c_int,
  pub watertype: c_int,

  pub target: EngineStringHandle,
  pub targetname: EngineStringHandle,
  pub netname: EngineStringHandle,
  pub message: EngineStringHandle,

  pub dmg_take: c_float,
  pub dmg_save: c_float,
  pub dmg: c_float,
  pub dmgtime: c_float,

  // Make this an array?
  pub noise: EngineStringHandle,
  pub noise1: EngineStringHandle,
  pub noise2: EngineStringHandle,
  pub noise3: EngineStringHandle,

  pub speed: c_float,
  pub air_finished: c_float,
  pub pain_finished: c_float,
  pub radsuit_finished: c_float,

  pub containing_entity: *mut Edict,

  pub playerclass: c_int,
  pub maxspeed: c_float,

  pub fov: c_float,
  pub weaponanim: c_int,

  pub pushmsec: c_int,

  pub in_duck: c_int,
  pub time_step_sound: c_int,
  pub swim_time: c_int,
  pub duck_time: c_int,
  pub step_left: c_int,
  pub fall_velocity: c_float,

  pub gamestate: c_int,

  pub oldbuttons: c_int,

  pub groupinfo: c_int,

  // Make these arrays?
  pub iuser1: c_int,
  pub iuser2: c_int,
  pub iuser3: c_int,
  pub iuser4: c_int,

  pub fuser1: c_float,
  pub fuser2: c_float,
  pub fuser3: c_float,
  pub fuser4: c_float,

  pub vuser1: EngineVector3,
  pub vuser2: EngineVector3,
  pub vuser3: EngineVector3,
  pub vuser4: EngineVector3,

  pub euser1: *mut Edict,
  pub euser2: *mut Edict,
  pub euser3: *mut Edict,
  pub euser4: *mut Edict,
}

// #[repr(C)]
// pub struct TraceResult {
  
// }

#[repr(C)]
pub struct EngineFunctions {
  pub f0: unsafe extern fn() -> (),
  pub f1: unsafe extern fn() -> (),
  pub f2: unsafe extern fn() -> (),
  pub f3: unsafe extern fn() -> (),
  pub f4: unsafe extern fn() -> (),
  pub f5: unsafe extern fn() -> (),
  pub f6: unsafe extern fn() -> (),
  pub f7: unsafe extern fn() -> (),
  pub f8: unsafe extern fn() -> (),
  pub f9: unsafe extern fn() -> (),
  pub f10: unsafe extern fn() -> (),
  pub f11: unsafe extern fn() -> (),
  pub f12: unsafe extern fn() -> (),
  pub f13: unsafe extern fn() -> (),
  pub f14: unsafe extern fn() -> (),
  pub f15: unsafe extern fn() -> (),
  pub f16: unsafe extern fn() -> (),
  pub f17: unsafe extern fn() -> (),
  pub f18: unsafe extern fn() -> (),
  pub f19: unsafe extern fn() -> (),
  pub f20: unsafe extern fn() -> (),
  pub f21: unsafe extern fn() -> (),
  pub f22: unsafe extern fn() -> (),
  pub f23: unsafe extern fn() -> (),
  pub f24: unsafe extern fn() -> (),
  pub f25: unsafe extern fn() -> (),
  pub f26: unsafe extern fn() -> (),
  pub f27: unsafe extern fn() -> (),
  pub f28: unsafe extern fn() -> (),
  pub f29: unsafe extern fn() -> (),
  pub f30: unsafe extern fn() -> (),
  pub f31: unsafe extern fn() -> (),
  pub f32: unsafe extern fn() -> (),
  pub f33: unsafe extern fn() -> (),
  pub f34: unsafe extern fn() -> (),
  pub f35: unsafe extern fn() -> (),
  pub f36: unsafe extern fn() -> (),
  pub f37: unsafe extern fn() -> (),
  pub f38: unsafe extern fn() -> (),
  pub f39: unsafe extern fn() -> (),
  pub f40: unsafe extern fn() -> (),
  pub f41: unsafe extern fn() -> (),
  pub f42: unsafe extern fn() -> (),
  pub f43: unsafe extern fn() -> (),
  pub f44: unsafe extern fn() -> (),
  pub f45: unsafe extern fn() -> (),
  pub f46: unsafe extern fn() -> (),
  pub f47: unsafe extern fn() -> (),
  pub f48: unsafe extern fn() -> (),
  pub f49: unsafe extern fn() -> (),
  pub f50: unsafe extern fn() -> (),
  pub f51: unsafe extern fn() -> (),
  pub f52: unsafe extern fn() -> (),
  pub f53: unsafe extern fn() -> (),
  pub f54: unsafe extern fn() -> (),
  pub f55: unsafe extern fn() -> (),
  pub f56: unsafe extern fn() -> (),
  pub f57: unsafe extern fn() -> (),
  pub f58: unsafe extern fn() -> (),
  pub f59: unsafe extern fn() -> (),
  pub f60: unsafe extern fn() -> (),
  pub f61: unsafe extern fn() -> (),
  pub f62: unsafe extern fn() -> (),
  pub f63: unsafe extern fn() -> (),
  pub f64: unsafe extern fn() -> (),
  pub f65: unsafe extern fn() -> (),
  pub sz_from_index: unsafe extern fn(c_int) -> *const c_char,
  pub alloc_string: unsafe extern fn(*const c_char) -> c_int,
  pub get_vars_of_ent: unsafe extern fn(*mut Edict) -> *mut EntVars,
  pub f69: unsafe extern fn() -> (),
  pub f70: unsafe extern fn() -> (),
  pub f71: unsafe extern fn() -> (),
  pub f72: unsafe extern fn() -> (),
  pub f73: unsafe extern fn() -> (),
  pub f74: unsafe extern fn() -> (),
  pub f75: unsafe extern fn() -> (),
  pub f76: unsafe extern fn() -> (),
  pub f77: unsafe extern fn() -> (),
  pub f78: unsafe extern fn() -> (),
  pub f79: unsafe extern fn() -> (),
  pub f80: unsafe extern fn() -> (),
  pub server_print: unsafe extern fn(*const c_char) -> (),
  pub f82: unsafe extern fn() -> (),
  pub f83: unsafe extern fn() -> (),
  pub f84: unsafe extern fn() -> (),
  pub f85: unsafe extern fn() -> (),
  pub f86: unsafe extern fn() -> (),
  pub f87: unsafe extern fn() -> (),
  pub f88: unsafe extern fn() -> (),
  pub f89: unsafe extern fn() -> (),
  pub f90: unsafe extern fn() -> (),
  pub f91: unsafe extern fn() -> (),
  pub f92: unsafe extern fn() -> (),
  pub f93: unsafe extern fn() -> (),
  pub f94: unsafe extern fn() -> (),
  pub f95: unsafe extern fn() -> (),
  pub f96: unsafe extern fn() -> (),
  pub f97: unsafe extern fn() -> (),
  pub f98: unsafe extern fn() -> (),
  pub f99: unsafe extern fn() -> (),
  pub f100: unsafe extern fn() -> (),
  pub f101: unsafe extern fn() -> (),
  pub f102: unsafe extern fn() -> (),
  pub f103: unsafe extern fn() -> (),
  pub f104: unsafe extern fn() -> (),
  pub f105: unsafe extern fn() -> (),
  pub f106: unsafe extern fn() -> (),
  pub f107: unsafe extern fn() -> (),
  pub f108: unsafe extern fn() -> (),
  pub f109: unsafe extern fn() -> (),
  pub f110: unsafe extern fn() -> (),
  pub f111: unsafe extern fn() -> (),
  pub f112: unsafe extern fn() -> (),
  pub f113: unsafe extern fn() -> (),
  pub f114: unsafe extern fn() -> (),
  pub f115: unsafe extern fn() -> (),
  pub f116: unsafe extern fn() -> (),
  pub f117: unsafe extern fn() -> (),
  pub f118: unsafe extern fn() -> (),
  pub f119: unsafe extern fn() -> (),
  pub f120: unsafe extern fn() -> (),
  pub f121: unsafe extern fn() -> (),
  pub f122: unsafe extern fn() -> (),
  pub f123: unsafe extern fn() -> (),
  pub f124: unsafe extern fn() -> (),
  pub f125: unsafe extern fn() -> (),
  pub f126: unsafe extern fn() -> (),
  pub f127: unsafe extern fn() -> (),
  pub f128: unsafe extern fn() -> (),
  pub f129: unsafe extern fn() -> (),
  pub f130: unsafe extern fn() -> (),
  pub f131: unsafe extern fn() -> (),
  pub f132: unsafe extern fn() -> (),
  pub f133: unsafe extern fn() -> (),
  pub f134: unsafe extern fn() -> (),
  pub f135: unsafe extern fn() -> (),
  pub f136: unsafe extern fn() -> (),
  pub f137: unsafe extern fn() -> (),
  pub f138: unsafe extern fn() -> (),
  pub f139: unsafe extern fn() -> (),
  pub f140: unsafe extern fn() -> (),
  pub f141: unsafe extern fn() -> (),
  pub f142: unsafe extern fn() -> (),
  pub f143: unsafe extern fn() -> (),
  pub f144: unsafe extern fn() -> (),
  pub f145: unsafe extern fn() -> (),
  pub f146: unsafe extern fn() -> (),
  pub f147: unsafe extern fn() -> (),
  pub f148: unsafe extern fn() -> (),
  pub f149: unsafe extern fn() -> (),
  pub f150: unsafe extern fn() -> (),
  pub f151: unsafe extern fn() -> (),
  pub f152: unsafe extern fn() -> (),
  pub f153: unsafe extern fn() -> (),
  pub f154: unsafe extern fn() -> (),
  pub f155: unsafe extern fn() -> (),
  pub f156: unsafe extern fn() -> (),
  pub f157: unsafe extern fn() -> (),
}

#[repr(C)]
pub struct DLLFunctions {
  pub game_init: unsafe extern fn() -> (),
  pub f1: unsafe extern fn() -> (),
  pub f2: unsafe extern fn() -> (),
  pub f3: unsafe extern fn() -> (),
  pub f4: unsafe extern fn() -> (),
  pub f5: unsafe extern fn() -> (),
  pub f6: unsafe extern fn() -> (),
  pub f7: unsafe extern fn() -> (),
  pub f8: unsafe extern fn() -> (),
  pub f9: unsafe extern fn() -> (),
  pub f10: unsafe extern fn() -> (),
  pub f11: unsafe extern fn() -> (),
  pub f12: unsafe extern fn() -> (),
  pub f13: unsafe extern fn() -> (),
  pub f14: unsafe extern fn() -> (),
  pub client_connect: unsafe extern fn(
    entity: *mut Edict,
    name: *const c_char,
    address: *const c_char,
    reject_reason: *mut c_char,
  ) -> c_int,
  pub client_disconnect: unsafe extern fn(entity: *mut Edict) -> (),
  pub client_kill: unsafe extern fn(entity: *mut Edict) -> (),
  pub client_put_in_server: unsafe extern fn(entity: *mut Edict) -> (),
  pub client_command: unsafe extern fn(entity: *mut Edict) -> (),
  pub client_user_info_changed: unsafe extern fn(
    entity: *mut Edict,
    info_buffer: *mut c_char,
  ) -> (),
  pub server_activate: unsafe extern fn(
    edict_list: *mut Edict,
    edict_count: c_int,
    max_clients: c_int
  ) -> (),
  pub server_deactivate: unsafe extern fn() -> (),
  pub player_pre_think: unsafe extern fn(entity: *mut Edict) -> (),
  pub player_post_think: unsafe extern fn(entity: *mut Edict) -> (),
  pub start_frame: unsafe extern fn() -> (),
  pub f26: unsafe extern fn() -> (),
  pub f27: unsafe extern fn() -> (),
  pub f28: unsafe extern fn() -> (),
  pub f29: unsafe extern fn() -> (),
  pub f30: unsafe extern fn() -> (),
  pub f31: unsafe extern fn() -> (),
  pub f32: unsafe extern fn() -> (),
  pub f33: unsafe extern fn() -> (),
  pub f34: unsafe extern fn() -> (),
  pub f35: unsafe extern fn() -> (),
  pub f36: unsafe extern fn() -> (),
  pub f37: unsafe extern fn() -> (),
  pub f38: unsafe extern fn() -> (),
  pub f39: unsafe extern fn() -> (),
  pub f40: unsafe extern fn() -> (),
  pub f41: unsafe extern fn() -> (),
  pub f42: unsafe extern fn() -> (),
  pub f43: unsafe extern fn() -> (),
  pub f44: unsafe extern fn() -> (),
  pub f45: unsafe extern fn() -> (),
  pub f46: unsafe extern fn() -> (),
  pub f47: unsafe extern fn() -> (),
  pub f48: unsafe extern fn() -> (),
  pub f49: unsafe extern fn() -> (),
}

#[repr(C)]
pub struct NewDLLFunctions {
  pub on_free_ent_private_data: unsafe extern fn(entity: *mut Edict) -> (),
  pub game_shutdown: unsafe extern fn() -> (),
  pub should_collide: unsafe extern fn(
    touched: *mut Edict,
    other: *mut Edict,
  ) -> c_int,
  #[deprecated(note = "Use cvar_value2 instead")]
  pub cvar_value: unsafe extern fn() -> (),
  pub cvar_value2: unsafe extern fn(
    entity: *const Edict,
    request_id: c_int,
    cvar_name: *const c_char,
    value: *const c_char,
  ) -> (),
}

#[repr(C)]
pub struct GameDLLFunctions {
  dllapi_table: *const DLLFunctions,
  newapi_table: *const NewDLLFunctions,
}

#[repr(C)]
#[derive(Default)]
pub struct MetaFunctions {
  pub get_entity_api: Option<
    unsafe extern fn(funcs: *mut DLLFunctions, if_vers: c_int) -> c_int
  >,
  pub get_entity_api_post: Option<
    unsafe extern fn(funcs: *mut DLLFunctions, if_vers: c_int) -> c_int
  >,
  pub get_entity_api2: Option<
    unsafe extern fn(funcs: *mut DLLFunctions, if_vers: *mut c_int) -> c_int
  >,
  pub get_entity_api2_post: Option<
    unsafe extern fn(funcs: *mut DLLFunctions, if_vers: *mut c_int) -> c_int
  >,
  pub get_new_dll_functions: Option<
    unsafe extern fn(
      funcs: *mut NewDLLFunctions,
      if_vers: *mut c_int,
    ) -> c_int
  >,
  pub get_new_dll_functions_post: Option<
    unsafe extern fn(
      funcs: *mut NewDLLFunctions,
      if_vers: *mut c_int,
    ) -> c_int
  >,
  pub get_engine_functions: Option<
    unsafe extern fn(
      funcs: *mut EngineFunctions,
      if_vers: *mut c_int,
    ) -> c_int,
  >,
  pub get_engine_functions_post: Option<
    unsafe extern fn(
      funcs: *mut EngineFunctions,
      if_vers: *mut c_int,
    ) -> c_int,
  >,
}

#[repr(C)]
pub struct GlobalVars {

}

#[repr(C)]
pub struct MetaUtilFuncs {
  pub log_console: unsafe extern fn(plid: &PluginInfo, msg: *const c_char),
  pub log_message: unsafe extern fn(plid: &PluginInfo, msg: *const c_char),
  pub log_error: unsafe extern fn(plid: &PluginInfo, msg: *const c_char),
  pub log_developer: unsafe extern fn(plid: &PluginInfo, msg: *const c_char),
  pub center_say: unsafe extern fn(plid: &PluginInfo, msg: *const c_char),
  pub center_say_parms: unsafe extern fn(),
  pub center_say_varargs: unsafe extern fn(),
  pub call_game_entity: unsafe extern fn(
    plid: &PluginInfo,
    classname: *const c_char,
    pev: *mut EntVars,
  ) -> c_int,
  pub get_user_msg_id: unsafe extern fn(
    plid: &PluginInfo,
    msg_name: *const c_char,
    size: *mut c_int,
  ) -> c_int,
  pub get_user_msg_name: unsafe extern fn(
    plid: &PluginInfo,
    msg_id: c_int,
    size: *mut c_int,
  ) -> *const c_char,
  pub get_plugin_path: unsafe extern fn(plid: &PluginInfo) -> *const c_char,
  pub get_game_info: unsafe extern fn(),
  pub load_plugin: unsafe extern fn(),
  pub unload_plugin: unsafe extern fn(),
  pub unload_plugin_by_handle: unsafe extern fn(),
  pub is_querying_client_cvar: unsafe extern fn(),
  pub make_request_id: unsafe extern fn(plid: &PluginInfo) -> c_int,
  pub get_hook_tables: unsafe extern fn(
    plid: &PluginInfo,
    engine_funcs: *mut *const EngineFunctions,
    dll_funcs: *mut *const DLLFunctions,
    new_dll_funcs: *mut *const NewDLLFunctions,
  ),
}
