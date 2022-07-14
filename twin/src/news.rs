use serde::{de::IntoDeserializer, Deserialize, Serialize};
use std::{
  collections::HashMap,
  fmt::Display,
  fs, io,
  path::{Path, PathBuf},
  str::FromStr,
  sync::{Arc, RwLock},
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Month {
  Jan,
  Feb,
  Mar,
  Apr,
  May,
  Jun,
  Jul,
  Aug,
  Sep,
  Oct,
  Nov,
  Dec,
}

impl Display for Month {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.serialize(f)
  }
}

impl FromStr for Month {
  type Err = serde::de::value::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Self::deserialize(s.into_deserializer())
  }
}

#[derive(Debug)]
pub enum NewsError {
  IOError(io::Error),
  CannotParseYear(String),
  CannotParseMonth(String),
  CannotParseDay(String),
}

impl Display for NewsError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      NewsError::IOError(e) => write!(f, "IO error: {}", e),
      NewsError::CannotParseYear(p) => write!(f, "cannot parse year directory: {}", p),
      NewsError::CannotParseMonth(p) => write!(f, "cannot parse month directory: {}", p),
      NewsError::CannotParseDay(p) => write!(f, "cannot parse day file: {}", p),
    }
  }
}

impl From<io::Error> for NewsError {
  fn from(e: io::Error) -> Self {
    Self::IOError(e)
  }
}

#[derive(Debug)]
pub struct News {
  pub html: String,
}

impl News {
  pub fn parse_from_md(md: impl AsRef<str>) -> Self {
    let opts = pulldown_cmark::Options::all();
    let parser = pulldown_cmark::Parser::new_ext(md.as_ref(), opts);

    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    News { html }
  }

  pub fn load_from_md(path: impl AsRef<Path>) -> Result<Self, NewsError> {
    let content = fs::read_to_string(path)?;
    let news = Self::parse_from_md(&content);
    Ok(news)
  }
}

/// Key used to uniquely refer to a weekly news.
///
/// It is composed of the year and week number.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NewsKey {
  pub year: u16,
  pub month: Month,
  pub day: u8,
}

impl NewsKey {
  pub fn to_path(&self, root: impl AsRef<Path>) -> PathBuf {
    PathBuf::from(format!(
      "{root}/{year}/{month}/{day:02}.md",
      root = root.as_ref().display(),
      year = self.year,
      month = self.month,
      day = self.day
    ))
  }
}

fn file_name_to_day(name: &str) -> Result<u8, NewsError> {
  // the format is NN.md, so the len() must always be 5
  if name.len() != 5 {
    return Err(NewsError::CannotParseDay(name.to_owned()));
  }

  let nn = &name[0..2];

  nn.parse()
    .map_err(|_| NewsError::CannotParseDay(name.to_owned()))
}

#[derive(Debug)]
pub struct NewsStore {
  root_path: PathBuf,
  news: HashMap<NewsKey, News>,
}

impl NewsStore {
  /// Create a new empty news store.
  pub fn new(root_path: impl Into<PathBuf>) -> Self {
    let root_path = root_path.into();
    let news = HashMap::new();
    Self { root_path, news }
  }

  /// Get all the keys
  pub fn keys<'a>(&'a self) -> impl Iterator<Item = &'a NewsKey> {
    self.news.keys()
  }

  /// Get a news from a key, if exists.
  pub fn get(&self, key: &NewsKey) -> Option<&News> {
    self.news.get(key)
  }

  /// Populate the store by scanning the root directory adding all of its content.
  pub fn populate_from_root(&mut self) -> Result<(), NewsError> {
    for entry in fs::read_dir(&self.root_path)? {
      if let Ok(entry) = entry {
        log::debug!("traversing year {}", entry.path().display());

        if entry.path().is_dir() {
          let year = entry
            .file_name()
            .to_str()
            .and_then(|name| name.parse().ok())
            .ok_or_else(|| NewsError::CannotParseYear(format!("{:?}", entry.file_name())))?;

          for month_entry in fs::read_dir(entry.path())? {
            if let Ok(month_entry) = month_entry {
              log::debug!("traversing month {}", month_entry.path().display());

              if month_entry.path().is_dir() {
                let month: Month = month_entry
                  .file_name()
                  .to_str()
                  .and_then(|name| name.parse().ok())
                  .ok_or_else(|| {
                    NewsError::CannotParseMonth(format!("{:?}", month_entry.file_name()))
                  })?;

                for day_entry in fs::read_dir(month_entry.path())? {
                  if let Ok(day_entry) = day_entry {
                    log::debug!("found day {}", day_entry.path().display());

                    if day_entry.path().is_file() {
                      let day = day_entry
                        .file_name()
                        .to_str()
                        .ok_or_else(|| {
                          NewsError::CannotParseDay(format!("{:?}", day_entry.file_name()))
                        })
                        .and_then(file_name_to_day)?;

                      let key = NewsKey { year, month, day };
                      let _ = self.update(key)?;
                    }
                  }
                }
              }
            }
          }
        }
      }
    }

    Ok(())
  }

  /// Update (or create) a weekly news by reading the Markdown news and converting it to HTML.
  ///
  /// If some HTML was already present for that key, it is returned.
  pub fn update(&mut self, key: NewsKey) -> Result<Option<News>, NewsError> {
    let path = key.to_path(&self.root_path);

    log::debug!("updating news key: {:?} (path={})", key, path.display());

    let news = News::load_from_md(path)?;
    let previous_news = self.news.insert(key, news);

    Ok(previous_news)
  }
}

/// Sharable news.
#[derive(Clone, Debug)]
pub struct NewsState {
  news_store: Arc<RwLock<NewsStore>>,
}

impl NewsState {
  pub fn new(news_root_path: impl Into<PathBuf>) -> Self {
    let news_store = Arc::new(RwLock::new(NewsStore::new(news_root_path)));

    Self { news_store }
  }

  pub fn news_store(&self) -> &Arc<RwLock<NewsStore>> {
    &self.news_store
  }
}
