use reqwasm::http::Request;

use yew::{html, Component, NodeRef, Properties};

pub struct Week {
  node_ref: NodeRef,
}

#[derive(Debug)]
pub enum Msg {
  GotWeek(Option<String>),
}

#[derive(Eq, Hash, PartialEq, Properties)]
pub struct WeekProps {
  #[prop_or_default]
  pub year: u16,

  #[prop_or_default]
  pub week_nb: u8,
}

impl Component for Week {
  type Message = Msg;

  type Properties = WeekProps;

  fn create(ctx: &yew::Context<Self>) -> Self {
    let props = ctx.props();

    let url = if props.year == 0 {
      "api/latest".to_owned()
    } else {
      format!("/api/{}/{}", props.year, props.week_nb)
    };

    ctx.link().send_future(async move {
      let news = Request::get(&url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

      Msg::GotWeek(Some(news))
    });

    Self {
      node_ref: NodeRef::default(),
    }
  }

  fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
    html! {
      <div class="container" ref={self.node_ref.clone()} />
    }
  }

  fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
    let html = match msg {
      Msg::GotWeek(Some(html)) => html,
      _ => "nope".to_owned(),
    };

    self
      .node_ref
      .cast::<web_sys::Element>()
      .unwrap()
      .set_inner_html(&html);

    true
  }
}
