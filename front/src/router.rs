use yew_router::Routable;

#[derive(Clone, Debug, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  Home,

  #[at("/latest")]
  Latest,

  #[at("/:year/:week_nb")]
  Week { year: u16, week_nb: u8 },

  #[not_found]
  #[at("/404")]
  NotFound,
}
