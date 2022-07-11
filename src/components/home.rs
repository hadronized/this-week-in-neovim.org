use std::time::Instant;

use chrono::Utc;
use yew::{html, Component};

pub struct HomeComponent;

impl Component for HomeComponent {
  type Message = ();

  type Properties = ();

  fn create(ctx: &yew::Context<Self>) -> Self {
    HomeComponent
  }

  fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
    let now = Utc::now();
    let date = now.format("%Y, %b #%M");
    let h1 = "This Week In Rust | ".to_owned() + &date.to_string();

    html! {
      <h1>{ h1 }</h1>
    }
  }
}
