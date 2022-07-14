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
      let href = format!("/{}/{}", key.year, key.week_nb);
      html! {
        <li>
          <a href={ href }>
            { key.year } { " #" } { key.week_nb }
          </a>

          { " " }

          <a href="/latest">
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
        <div class="container section has-text-justified">
          <p class="block has-text-justified">{"
            This Week In Neovim is a hand-crafted weekly newsletter gathering everything that has happened in the past week around Neovim Core and
            in the Neovim Plugin ecosystem. Each weekly news is then separated into two main categories: Core and Plugins.
          "}</p>

          <p class="block has-text-justified">{"
            The Core part is about the changes in Neovim itself, whether it is about the nightly (main development branch) or the public stable
            release.
          "}</p>

          <p class="block has-text-justified">{"
            The Plugin part is the result of skimming main communication sources, such as "} <a href="https://www.reddit.com/r/neovim">{ "Reddit" }</a> {",
            various GitHub projects, embedded help Neovim manuals and others. It is also a contribution-based process where people can open PRs and issues
            to contribute their findings.
          "}</p>

          <p class="block has-text-justified">{"
            More on all that in the "} <a href="#want-to-contribute">{ "contributing section" }</a> {".
          "}</p>
        </div>

        <div class="container section has-text-centered">
          <ul>
            { news_list }
          </ul>
        </div>

        <div class="container section has-text-justified">
          <h1 class="title has-text-link" id="want-to-contribute">
            <a href="#want-to-contribute">
              { " Want to contribute?" }
            </a>
          </h1>

          <p class="block">
            { "You have noticed something missing that you saw lately? Do not keep the candies for yourself and please feel free to
            share with us! You can open a PR at "}
            <a href="https://github.com/phaazon/this-week-in-neovim-contents">{ "this-week-in-neggovim-contents" }</a>
            {"."}
          </p>

          <p class="block">
            { "Feel free to read "}
            <a href="https://github.com/phaazon/this-week-in-neovim-contents/blob/master/README.md#how-to-contribute">{ "how to contribute" }</a>
            { " to get started." }
          </p>
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
