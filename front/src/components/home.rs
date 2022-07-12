use chrono::Utc;
use reqwasm::http::Request;
use std::sync::{Arc, RwLock};
use twin::news::NewsKey;
use yew::{html, Component};

#[derive(Debug)]
pub struct HomeComponent {
  keys: Arc<RwLock<Vec<NewsKey>>>,
}

impl Component for HomeComponent {
  type Message = ();

  type Properties = ();

  fn create(ctx: &yew::Context<Self>) -> Self {
    let keys = Arc::new(RwLock::new(Vec::new()));

    let keys_writer = keys.clone();
    wasm_bindgen_futures::spawn_local(async move {
      let resp = Request::get("http://127.0.0.1/api:8000")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
      *keys_writer.write().unwrap() = resp;
    });

    HomeComponent { keys }
  }

  fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
    let now = Utc::now();
    let date = now.format("%Y, %b #%M");

    html! {
      <section class={"hero is-success"}>
        <div class={"hero-body has-text-centered"}>
          <h1 class={"title"}>{ "This Week in Neovim" }</h1>
        </div>
      </section>
    }
  }
}
