use rocket::{
  get,
  response::{content::RawHtml, status::NotFound},
  serde::json::Json,
  State,
};

use crate::news::{NewsKey, NewsState};

#[get("/")]
pub fn root(state: &State<NewsState>) -> Json<Vec<NewsKey>> {
  let news_store = state.news_store().read().expect("news store");
  let keys = news_store.keys().cloned().collect();

  Json(keys)
}

#[get("/<year>/<week_nb>")]
pub fn by_year_week_nb(
  year: u16,
  week_nb: u8,
  state: &State<NewsState>,
) -> Result<RawHtml<String>, NotFound<String>> {
  let news_store = state.news_store().read().expect("news store");
  let key = NewsKey { year, week_nb };

  match news_store.get(&key) {
    Some(news) => Ok(RawHtml(news.html.to_owned())),
    None => Err(NotFound(format!(
      "news #{} in {} doesn’t exist",
      week_nb, year
    ))),
  }
}
