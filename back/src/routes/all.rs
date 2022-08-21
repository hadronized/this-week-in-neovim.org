use crate::{cache::Cache, html_wrapper::html_wrap};
use rocket::{get, response::content::RawHtml, State};
use std::cmp::Reverse;
use twin::news::NewsState;

#[get("/all")]
pub fn all(cache: &State<Cache>, state: &State<NewsState>) -> RawHtml<String> {
  RawHtml(cache.cache("/all", || render(state)))
}

fn render(state: &NewsState) -> String {
  let store = state.news_store().read().expect("news store");
  let mut keys: Vec<_> = store.keys().collect();
  let keys_len = keys.len();

  keys.sort_by_key(|&&k| Reverse(k));

  let news_list: Vec<_> = keys
    .into_iter()
    .enumerate()
    .map(|(k, key)| {
      let href = format!("/{}/{}/{:02}", key.year, key.month, key.day);
      let k = keys_len - k;

      format!(
        include_str!("./home_listing.html"),
        href = href,
        key_year = key.year,
        key_month = key.month,
        key_day = key.day,
        k = k,
      )
    })
    .collect();

  let html = format!(include_str!("all.html"), news_list = news_list.join(""));

  html_wrap("", html)
}
