use std::cmp::Reverse;
use twin::news::NewsKey;

pub fn news_to_rss(key: &NewsKey) -> rss::Item {
  rss::ItemBuilder::default()
    .author(Some(
      "Dimitri 'phaazon' Sabadie <dimitri.sabadie@gmail.com>".to_owned(),
    ))
    .pub_date(Some(format!(
      "{}",
      format!("{} {} {} GMT", key.day, key.month, key.year)
    )))
    .link(Some(format!(
      "https://this-week-in-neovim.org/{}/{}/{}",
      key.year, key.month, key.day
    )))
    .title(Some(format!("{} {} {}", key.day, key.month, key.year)))
    .build()
}

pub fn rss_feed<'a>(items: impl IntoIterator<Item = &'a NewsKey>) -> rss::Channel {
  let mut items: Vec<_> = items
    .into_iter()
    .map(|key| (key, news_to_rss(key)))
    .collect();
  items.sort_by_key(|(key, _)| Reverse(*key));

  let last_build_date = items
    .get(0)
    .map(|(key, _)| format!("{} {} {} GMT", key.day, key.month, key.year));

  let items: Vec<_> = items.into_iter().map(|(_, news)| news).collect();

  rss::ChannelBuilder::default()
    .title("This Week In Neovim".to_owned())
    .link("https://this-week-in-neovim.org".to_owned())
    .items(items)
    .last_build_date(last_build_date)
    .build()
}
