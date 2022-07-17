use std::cmp::Reverse;

use rocket::{get, response::content::RawHtml, State};
use twin::news::NewsState;

#[get("/")]
pub fn home(state: &State<NewsState>) -> RawHtml<String> {
  view(state)
}

fn view(state: &NewsState) -> RawHtml<String> {
  let store = state.news_store().read().expect("news store");
  let mut keys: Vec<_> = store.keys().collect();
  let keys_len = keys.len();

  keys.sort_by_key(|&&k| Reverse(k));

  let mut iter = keys.iter();

  let first = iter.next().map(|key| {
    let href = format!("/{}/{}/{:02}", key.year, key.month, key.day);
    format!(
      "<li class=\"is-size-2\">
        <a href={href}>
          {key_year} {key_month} {key_day:02} #{keys_len}
        </a>

        <a href=\"/latest\">
          <span class=\"tag is-link is-large\">latest</span>
        </a>
      </li>",
      key_year = key.year,
      key_month = key.month,
      key_day = key.day
    )
  });

  let news_list: Vec<_> = first
    .into_iter()
    .chain(iter.enumerate().map(|(k, key)| {
      let href = format!("/{}/{}/{:02}", key.year, key.month, key.day);
      let k = keys_len - k - 1;

      format!(
        "<li class=\"is-size-2\">
          <a href={href}>
            {key_year} {key_month} {key_day:02} #{k}
          </a>
        </li>",
        key_year = key.year,
        key_month = key.month,
        key_day = key.day
      )
    }))
    .collect();

  let html = format!(
    include_str!("home.html"),
    keys_len = keys_len,
    news_list = news_list.join("")
  );

  // let news_list =
  RawHtml(html)
}
