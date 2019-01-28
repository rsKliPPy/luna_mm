use std::ptr::{null, null_mut};
use super::types::{
  EngineFunctions,
  GlobalVars,
  MetaUtilFuncs,
  MetaGlobals,
  GameDLLFunctions,
  DLLFunctions,
  NewDLLFunctions,
};

pub static mut ENGINE_FUNCTIONS: *const EngineFunctions = null();
pub static mut GLOBAL_VARS: *const GlobalVars = null();
pub static mut META_UTIL_FUNCS: *const MetaUtilFuncs = null();
pub static mut META_GLOBALS: *mut MetaGlobals = null_mut();
pub static mut GAME_DLL_FUNCTIONS: *const GameDLLFunctions = null();
pub static mut DLL_HOOK_TABLE: *mut DLLFunctions = null_mut();
pub static mut DLL_HOOK_TABLE_POST: *mut DLLFunctions = null_mut();
pub static mut NEWDLL_HOOK_TABLE: *mut NewDLLFunctions = null_mut();
pub static mut NEWDLL_HOOK_TABLE_POST: *mut NewDLLFunctions = null_mut();
pub static mut ENGINE_HOOK_TABLE: *mut EngineFunctions = null_mut();
pub static mut ENGINE_HOOK_TABLE_POST: *mut EngineFunctions = null_mut();
