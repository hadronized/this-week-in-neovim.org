use crate::html_wrapper::html_wrap;
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
pub fn latest(state: &State<NewsState>) -> Result<RawHtml<String>, NotFound<String>> {
  let news_store = state.news_store().read().expect("news store");
  let key = news_store
    .keys()
    .max()
    .ok_or_else(|| NotFound("no latest news available".to_owned()))?;

  render(*key, state)
}

#[get("/<year>/<month>/<day>")]
pub fn by_key(
  year: u16,
  month: MonthParam,
  day: u8,
  state: &State<NewsState>,
) -> Result<RawHtml<String>, NotFound<String>> {
  let MonthParam(month) = month;
  let key = NewsKey { year, month, day };

  render(key, state)
}

fn render(key: NewsKey, state: &NewsState) -> Result<RawHtml<String>, NotFound<String>> {
  let store = state.news_store().read().expect("news store");
  let news = store.get(&key).ok_or_else(|| {
    NotFound(format!(
      "news {year}-{month}-{day} doesn’t exist",
      year = key.year,
      month = key.month,
      day = key.day,
    ))
  })?;

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

  Ok(RawHtml(html))
}
