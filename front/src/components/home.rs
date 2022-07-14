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
        <li>{ key.year } { " #" } { key.week_nb } { " " }
          <a href={"/latest"}>
            <span class={"tag is-link"}>{ "latest" }</span>
          </a>
        </li>
      }
    });

    let news_list: Vec<_> = first
      .into_iter()
      .chain(iter.map(|key| {
        let href = format!("/{}/{}", key.year, key.week_nb);
        html! {
          <li>
            <a href={ href }>
              { key.year } { " #" } { key.week_nb }
            </a>
          </li>
        }
      }))
      .collect();

    html! {
      <div>
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
