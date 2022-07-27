use std::fmt::Display;

use chrono::{Datelike as _, Utc};

pub fn html_wrap(title: impl Into<String>, contents: impl Display) -> String {
  let title = title.into();
  let now = Utc::now().year();

  let title = if title.is_empty() {
    title
  } else {
    format!(" — {}", title)
  };

  format!(
    include_str!("html_wrapper.html"),
    title = title,
    contents = contents,
    now = now
  )
}
