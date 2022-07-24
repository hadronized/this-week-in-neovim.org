use serde::Deserialize;
use std::{env, fmt::Display, fs, path::PathBuf};

pub enum ConfigError {
  CannotReadConfig,
  Toml(toml::de::Error),
}

impl Display for ConfigError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ConfigError::CannotReadConfig => f.write_str("cannot read configuration file"),
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
  /// Port to listen on.
  pub port: u16,

  /// Path where to read the weekly contents.
  pub news_root: PathBuf,

  /// Static directory (CSS, etc.).
  pub static_dir: PathBuf,
}

impl Config {
  pub fn load() -> Result<Self, ConfigError> {
    println!("loading configuration");

    let path = PathBuf::from(env::var("TWIN_CONFIG").unwrap_or_else(|_| "config.toml".to_owned()));

    if !path.is_file() {
      return Err(ConfigError::CannotReadConfig);
    }

    println!("┝ loading with path: {}", path.display());

    let contents = fs::read_to_string(&path).expect("config file");
    let config = toml::from_str(&contents)?;

    Ok(config)
  }
}
