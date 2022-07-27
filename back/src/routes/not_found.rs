use crate::html_wrapper::html_wrap;
use rocket::{catch, response::content::RawHtml};

#[catch(404)]
pub fn not_found() -> RawHtml<String> {
  RawHtml(html_wrap("Not found!", include_str!("not_found.html")))
}
