use rocket::{routes, Route};

pub mod all;
pub mod home;
pub mod not_found;
pub mod rss;
pub mod week;

pub fn routes() -> Vec<Route> {
  routes![all::all, home::home, week::by_key, week::latest, rss::rss]
}
