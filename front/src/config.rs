use serde::Deserialize;
use std::{env, fmt::Display, fs, path::PathBuf};
use yew::Properties;

pub enum ConfigError {
  Toml(toml::de::Error),
}

impl Display for ConfigError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ConfigError::Toml(e) => write!(f, "toml error: {}", e),
    }
  }
}

impl From<toml::de::Error> for ConfigError {
  fn from(e: toml::de::Error) -> Self {
    Self::Toml(e)
  }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Config {
  /// Path where to read the weekly contents.
  contents_path: PathBuf,
}

impl Properties for Config {
  type Builder = ();

  fn builder() -> Self::Builder {
    ()
  }
}

impl Config {
  pub fn load() -> Result<Self, ConfigError> {
    log::debug!("loading configuration");

    let path = env::var("TWIN_CONFIG").unwrap_or_else(|_| "config.toml".to_owned());
    log::debug!("┝ loading with path: {}", path);

    let contents = fs::read_to_string(&path).expect("config file");
    log::debug!("└ read fro")
    let config = toml::from_str(&contents)?;

    Ok(config)
  }
}
