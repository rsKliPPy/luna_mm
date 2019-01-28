#![allow(dead_code)]

use super::types::MetaResult;
use super::globals::META_GLOBALS;

pub unsafe fn set_meta_result(result: MetaResult) {
  (*META_GLOBALS).result = result;
}

pub unsafe fn meta_return_value<T>(result: MetaResult, value: T) -> T {
  set_meta_result(result);
  value
}

pub unsafe fn get_meta_result() -> MetaResult {
  (*META_GLOBALS).result
}

pub unsafe fn prev_meta_result() -> MetaResult {
  (*META_GLOBALS).prev_result
}

pub unsafe fn get_meta_orig_ret<T: Copy>() -> T {
  *((*META_GLOBALS).orig_ret as *const T)
}

pub unsafe fn get_meta_override_ret<T: Copy>() -> T {
  *((*META_GLOBALS).override_ret as *const T)
}
