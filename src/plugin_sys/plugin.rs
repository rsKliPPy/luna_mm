use std::collections::HashMap;
use std::path::{PathBuf, Path};
use std::error::Error;
use std::fs;
use serde_derive::{Serialize, Deserialize};


#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct PluginInfo {
  pub title: String,
  #[serde(default)]
  pub description: String,
  pub version: String,
  pub authors: Vec<String>,
  pub interface: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Manifest {
  pub info: PluginInfo,
  #[serde(default)]
  pub dependencies: HashMap<String, String>,
  #[serde(default)]
  pub metadata: HashMap<String, String>,
}

fn load_manifest(
  file_path: impl Into<PathBuf>
) -> Result<Manifest, Box<dyn Error>> {
  let contents = fs::read_to_string(file_path.into())?;
  Ok(toml::from_str(&contents)?)
}

pub struct Plugin {
  identifier: String,
  main_source_path: PathBuf,
  info: PluginInfo,
  dependencies: HashMap<String, String>,
  metadata: HashMap<String, String>,
}

impl Plugin {
  pub fn load_plugin(
    dir: impl Into<PathBuf>,
    identifier: &str,
  ) -> Result<Self, Box<dyn Error>> {
    let dir = dir.into();

    let mut main_path = dir.clone();
    main_path.push("Plugin.lua");
    if !main_path.exists() {
      return Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Plugin.lua not found"
      )));
    }
    
    let mut manifest_path = dir.clone();
    manifest_path.push("Plugin.toml");

    let manifest = load_manifest(manifest_path)?;
    
    Ok(Plugin {
      identifier: identifier.to_string(),
      main_source_path: main_path.clone(),
      info: manifest.info,
      dependencies: manifest.dependencies,
      metadata: manifest.metadata,
    })
  }

  pub fn identifier(&self) -> &str {
    &self.identifier
  }

  pub fn info(&self) -> &PluginInfo {
    &self.info
  }

  pub fn metadata(&self) -> &HashMap<String, String> {
    &self.metadata
  }

  pub fn main_source_path(&self) -> &Path {
    &self.main_source_path
  }

  pub fn dependencies(&self) -> &HashMap<String, String> {
    &self.dependencies
  }
}
