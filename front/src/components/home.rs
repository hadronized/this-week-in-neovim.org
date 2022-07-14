use reqwasm::http::Request;
use std::cmp::Reverse;
use twin::news::NewsKey;
use yew::{html, Component};

pub struct Home {
  keys: Vec<NewsKey>,
}

#[derive(Debug)]
pub enum Msg {
  GotNewsKeys(Vec<NewsKey>),
}

impl Component for Home {
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

    Home { keys: Vec::new() }
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

        <div class={"section"}>
          <h1>{ "Want to contribute?" }</h1>

          <p>{ "You have noticed something missing that you saw lately? Do not keep the candies for yourself and please feel free to
          share with us! You can open a PR at https://github.com/phaazon/this-week-in-rust-contents." }</p>

          <p>{ "Feel free to read https://github.com/phaazon/this-week-in-rust-contents/README.md#how-to-contribute to get started." }</p>
        </div>

        <footer class={"footer has-text-centered"}>
          <p>
            { "Made by " }
            <a href={"https://github.com/phaazon"}>{ "Dimitri @phaazon Sabadie" }</a>
            { " and contributors." }
          </p>
          <p>
            <a href={"https://github.com/phaazon/this-week-in-neovim.org"}>
              <span class={"icon-text has-text-link"}>
                <span class={"icon"}>
                  <i class={"fa-brands fa-github"}></i>
                </span>
                <span>{ "TWiN" }</span>
              </span>
            </a>
            { " | " }
            <a href={"https://rust-lang.org"}>
              <span class={"icon-text has-text-link"}>
                <span class={"icon"}>
                  <i class={"fa-brands fa-rust"}></i>
                </span>
                <span>{ "Rust" }</span>
              </span>
            </a>
            { " | " }
            <a href={"https://rocket.rs"}>
              <span class={"icon-text has-text-link"}>
                <span class={"icon"}>
                  <i class={"fa-solid fa-shuttle-space"}></i>
                </span>
                <span>{ "rocket-rs" }</span>
              </span>
            </a>
            { " | " }
            <a href={"https://yew.rs"}>
              <span class={"icon-text has-text-link"}>
                <span class={"icon"}>
                  <i class={"fa-solid fa-y"}></i>
                </span>
                <span>{ "Yew" }</span>
              </span>
            </a>
          </p>
        </footer>
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
