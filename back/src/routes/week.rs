use crate::{cache::Cache, html_wrapper::html_wrap};
use rocket::{
  get,
  request::FromParam,
  response::{content::RawHtml, status::NotFound},
  State,
};
use std::str::FromStr;
use twin::news::{Month, NewsKey, NewsState};

pub struct MonthParam(Month);

impl<'a> FromParam<'a> for MonthParam {
  type Error = <Month as FromStr>::Err;

  fn from_param(param: &'a str) -> Result<Self, Self::Error> {
    param.parse().map(MonthParam)
  }
}

#[get("/latest")]
pub fn latest(
  cache: &State<Cache>,
  state: &State<NewsState>,
) -> Result<RawHtml<String>, NotFound<String>> {
  let news_store = state.news_store().read().expect("news store");
  let key = news_store
    .keys()
    .max()
    .ok_or_else(|| NotFound("no latest news available".to_owned()))?;

  render(*key, cache, state).ok_or_else(|| NotFound("no latest news available".to_owned()))
}

#[get("/<year>/<month>/<day>")]
pub fn by_key(
  year: u16,
  month: MonthParam,
  day: u8,
  cache: &State<Cache>,
  state: &State<NewsState>,
) -> Result<RawHtml<String>, NotFound<String>> {
  let MonthParam(month) = month;
  let key = NewsKey { year, month, day };

  render(key, cache, state)
    .ok_or_else(|| NotFound(format!("news {year}-{month}-{day} doesn’t exist")))
}

fn render(key: NewsKey, cache: &Cache, state: &NewsState) -> Option<RawHtml<String>> {
  cache
    .cache_if_any(&format!("/{}/{}/{}", key.year, key.month, key.day), || {
      let store = state.news_store().read().expect("news store");
      let news = store.get(&key)?;

      // if we have prev and/or next key, we need to generate the html for them
      let prev_date = if let Some(prev) = news.prev {
        format!(
          include_str!("prev_date.html"),
          day = prev.day,
          month = prev.month,
          year = prev.year
        )
      } else {
        "".to_owned()
      };

      let next_date = if let Some(next) = news.next {
        format!(
          include_str!("next_date.html"),
          day = next.day,
          month = next.month,
          year = next.year
        )
      } else {
        "".to_owned()
      };

      let html = html_wrap(format!(
        include_str!("week.html"),
        prev_date = prev_date,
        next_date = next_date,
        day = key.day,
        month = key.month,
        year = key.year,
        contents = news.html
      ));

      Some(html)
    })
    .map(RawHtml)
}
