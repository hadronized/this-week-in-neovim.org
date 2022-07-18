use crate::rss::rss_feed;
use rocket::{
  get,
  request::FromParam,
  response::{content::RawXml, status::NotFound},
  serde::json::Json,
  State,
};
use std::str::FromStr;
use twin::news::{LatestNews, Month, News, NewsKey, NewsState};

#[get("/")]
pub fn root(state: &State<NewsState>) -> Json<Vec<NewsKey>> {
  let news_store = state.news_store().read().expect("news store");
  let keys = news_store.keys().cloned().collect();

  Json(keys)
}

#[get("/latest")]
pub fn latest(state: &State<NewsState>) -> Result<Json<LatestNews>, NotFound<String>> {
  let news_store = state.news_store().read().expect("news store");
  let key = news_store
    .keys()
    .max()
    .ok_or_else(|| NotFound("no latest news available".to_owned()))?;
  let Json(news) = by_key(key.year, MonthParam(key.month), key.day, state)?;

  Ok(Json(LatestNews { key: *key, news }))
}

pub struct MonthParam(Month);

impl<'a> FromParam<'a> for MonthParam {
  type Error = <Month as FromStr>::Err;

  fn from_param(param: &'a str) -> Result<Self, Self::Error> {
    param.parse().map(MonthParam)
  }
}

#[get("/<year>/<month>/<day>")]
pub fn by_key(
  year: u16,
  month: MonthParam,
  day: u8,
  state: &State<NewsState>,
) -> Result<Json<News>, NotFound<String>> {
  let news_store = state.news_store().read().expect("news store");
  let MonthParam(month) = month;
  let key = NewsKey { year, month, day };

  match news_store.get(&key) {
    Some(news) => Ok(Json(news.clone())),
    None => Err(NotFound(format!(
      "news {year}-{month}-{day} doesn’t exist",
      year = key.year,
      month = key.month,
      day = key.day,
    ))),
  }
}

#[get("/rss")]
pub fn rss(state: &State<NewsState>) -> RawXml<String> {
  let news_store = state.news_store().read().expect("news store");
  let feed = rss_feed(&news_store);
  RawXml(feed.to_string())
}
