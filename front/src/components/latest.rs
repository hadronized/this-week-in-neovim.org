use reqwasm::http::Request;
use yew::{html, Component, NodeRef};

pub struct Latest {
  node_ref: NodeRef,
}

#[derive(Debug)]
pub enum Msg {
  GotLatest(String),
}

impl Component for Latest {
  type Message = Msg;

  type Properties = ();

  fn create(ctx: &yew::Context<Self>) -> Self {
    ctx.link().send_future(async move {
      let news = Request::get("/api/latest")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

      Msg::GotLatest(news)
    });

    Self {
      node_ref: NodeRef::default(),
    }
  }

  fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
    html! {
      <div ref={self.node_ref.clone()} />
    }
  }

  fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::GotLatest(html) => {
        self
          .node_ref
          .cast::<web_sys::Element>()
          .unwrap()
          .set_inner_html(&html);

        true
      }
    }
  }
}
