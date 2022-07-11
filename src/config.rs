use std::path::PathBuf;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
  /// Path where to read the weekly contents.
  contents_path: PathBuf,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      contents_path: PathBuf::from("config.toml"),
    }
  }
}
