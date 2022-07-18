use std::cmp::Reverse;
use twin::news::{NewsKey, NewsStore, News};

pub fn news_to_rss(key: &NewsKey, content: Option<&News>) -> rss::Item {
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
	.description(match content {
		Some(item) => Some(item.html.to_owned()),
		None => None,
	})
    .build()
}

pub fn rss_feed(news_store: &NewsStore) -> rss::Channel {
  let mut items: Vec<_> = news_store.keys()
    .into_iter()
    .map(|key| (key, news_to_rss(key, news_store.get(key))))
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
