use rocket::{get, response::content::RawXml, State};
use std::cmp::Reverse;
use twin::news::{News, NewsKey, NewsState, NewsStore, Month};

use crate::cache::Cache;

#[get("/rss")]
pub fn rss(cache: &State<Cache>, state: &State<NewsState>) -> RawXml<String> {
  RawXml(cache.cache("/rss", || {
    let news_store = state.news_store().read().expect("news store");
    let feed = rss_feed(&news_store);
    feed.to_string()
  }))
}

fn month_to_ordinal(month: Month) -> u8 {
  return match month {
    Month::Jan => 1,
    Month::Feb => 2,
    Month::Mar => 3,
    Month::Apr => 4,
    Month::May => 5,
    Month::Jun => 6,
    Month::Jul => 7,
    Month::Aug => 8,
    Month::Sep => 9,
    Month::Oct => 10,
    Month::Nov => 11,
    Month::Dec => 12
  };
}

pub fn news_to_rss(key: &NewsKey, content: Option<&News>) -> ::rss::Item {
  ::rss::ItemBuilder::default()
    .author(Some(
      "Dimitri 'phaazon' Sabadie <dimitri.sabadie@gmail.com>".to_owned(),
    ))
    .pub_date(Some(format!(
      "{}",
      format!(
        "{}-{:02}-{:02}T00:00:00+00:00",
        key.year, month_to_ordinal(key.month), key.day
      )
    )))
    .link(Some(format!(
      "https://this-week-in-neovim.org/{}/{}/{}",
      key.year, key.month, key.day
    )))
    .title(Some(format!("{} {} {}", key.day, key.month, key.year)))
    .description(match content {
      Some(item) => Some(item.html.to_owned()),
      None => None,
    })
    .build()
}

pub fn rss_feed(news_store: &NewsStore) -> ::rss::Channel {
  let mut items: Vec<_> = news_store
    .keys()
    .into_iter()
    .map(|key| (key, news_to_rss(key, news_store.get(key))))
    .collect();
  items.sort_by_key(|(key, _)| Reverse(*key));

  let last_build_date = items
    .get(0)
    .map(|(key, _)| format!("{} {} {} GMT", key.day, key.month, key.year));

  let items: Vec<_> = items.into_iter().map(|(_, news)| news).collect();

  ::rss::ChannelBuilder::default()
    .title("This Week In Neovim".to_owned())
    .link("https://this-week-in-neovim.org".to_owned())
    .items(items)
    .last_build_date(last_build_date)
    .build()
}
