use std::path::PathBuf;
use super::{
  get_plugin_path,
};

pub struct FFIUtilFuncs<'a> {
  _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> FFIUtilFuncs<'a> {
  pub const fn new() -> Self {
    FFIUtilFuncs {
      _phantom: std::marker::PhantomData,
    }
  }
  
  pub fn get_meta_plugin_path(&self) -> PathBuf {
    unsafe { get_plugin_path() }
  }
}
