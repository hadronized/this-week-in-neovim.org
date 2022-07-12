use chrono::Utc;
use reqwasm::http::Request;
use std::{
  cmp::Reverse,
  sync::{Arc, RwLock},
};
use twin::news::NewsKey;
use yew::{html, Component};

pub struct HomeComponent {
  keys: Vec<NewsKey>,
}

#[derive(Debug)]
pub enum Msg {
  GotNewsKeys(Vec<NewsKey>),
}

impl Component for HomeComponent {
  type Message = Msg;

  type Properties = ();

  fn create(ctx: &yew::Context<Self>) -> Self {
    ctx.link().send_future(async move {
      let keys = Request::get("/api")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

      Msg::GotNewsKeys(keys)
    });

    HomeComponent { keys: Vec::new() }
  }

  fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
    let mut iter = self.keys.iter();
    let first = iter.next().map(|key| {
      html! {
        <li>{ key.year } { " #" } { key.week_nb } <span class={"tag"}>{ "latest" }</span></li>
      }
    });

    let news_list: Vec<_> = first
      .into_iter()
      .chain(iter.map(|key| {
        html! {
          <li>{ key.year } { " #" } { key.week_nb }</li>
        }
      }))
      .collect();

    html! {
      <div>
        <section class={"hero is-success"}>
          <div class={"hero-body has-text-centered"}>
            <h1 class={"title"}>{ "This Week in Neovim" }</h1>
          </div>

        </section>

        <div class={"has-text-centered"}>
          <ul>
            { news_list }
          </ul>
        </div>
      </div>
    }
  }

  fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::GotNewsKeys(mut keys) => {
        keys.sort_by_key(|&k| Reverse(k));
        self.keys = keys;
        true
      }
    }
  }
}
