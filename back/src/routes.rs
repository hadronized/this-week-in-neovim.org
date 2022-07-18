use rocket::{routes, Route};

pub mod home;
pub mod not_found;
pub mod week;

pub fn routes() -> Vec<Route> {
  routes![home::home, week::by_key, week::latest]
}
