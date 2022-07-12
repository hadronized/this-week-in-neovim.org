use std::{
  collections::HashMap,
  fmt::Display,
  fs, io,
  path::{Path, PathBuf},
  sync::{Arc, RwLock},
};

use serde::Serialize;

#[derive(Debug)]
pub enum NewsError {
  IOError(io::Error),
  CannotParseYear(String),
  CannotParseWeek(String),
}

impl Display for NewsError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      NewsError::IOError(e) => write!(f, "IO error: {}", e),
      NewsError::CannotParseYear(p) => write!(f, "cannot parse year directory: {}", p),
      NewsError::CannotParseWeek(p) => write!(f, "cannot parse week file name: {}", p),
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
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NewsKey {
  pub year: u16,
  pub week_nb: u8,
}

fn file_name_to_week_nb(name: &str) -> Result<u8, NewsError> {
  // the format is week-NN.md, so the len() must always be 10; NN starts at 5 and ends at 6
  if name.len() != 10 {
    return Err(NewsError::CannotParseWeek(name.to_owned()));
  }

  let nn = &name[5..=6];

  nn.parse()
    .map_err(|_| NewsError::CannotParseWeek(name.to_owned()))
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
        if entry.path().is_dir() {
          let year = entry
            .file_name()
            .to_str()
            .and_then(|name| name.parse().ok())
            .ok_or(NewsError::CannotParseYear(format!(
              "{:?}",
              entry.file_name()
            )))?;

          for week_entry in fs::read_dir(entry.path())? {
            if let Ok(week_entry) = week_entry {
              if week_entry.path().is_file() {
                let week_nb = week_entry // TODO
                  .file_name()
                  .to_str()
                  .ok_or(NewsError::CannotParseWeek(format!(
                    "{:?}",
                    entry.file_name()
                  )))
                  .and_then(file_name_to_week_nb)?;
                let key = NewsKey { year, week_nb };

                self.update(key)?;
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
    let path = format!(
      "{}/{}/week-{}.md",
      self.root_path.display(),
      key.year,
      key.week_nb
    );

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
