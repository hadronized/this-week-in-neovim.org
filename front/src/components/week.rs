use reqwasm::http::Request;

use twin::news::Month;
use web_sys::{Element, Node};
use yew::{html, virtual_dom::VNode, Component, Html, NodeRef, Properties};

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

  #[prop_or(Month::Jan)]
  pub month: Month,

  #[prop_or_default]
  pub day: u8,
}

impl Component for Week {
  type Message = Msg;

  type Properties = WeekProps;

  fn create(ctx: &yew::Context<Self>) -> Self {
    let props = ctx.props();

    let url = if props.year == 0 {
      "api/latest".to_owned()
    } else {
      format!("/api/{}/{}/{}", props.year, props.month, props.day)
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

  fn view(&self, ctx: &yew::Context<Self>) -> Html {
    let props = ctx.props();
    html! {
      <div class="section container">
        <nav class="level">
          <div class="level-item">
            <p class="subtitle">
              <a href="/">
                <span class="icon-text">
                  <span class="icon">
                    <i class="fa-solid fa-angle-left"></i>
                  </span>
                  <span>{"Previous day"}</span>
                </span>
              </a>
            </p>
          </div>

          <div class="level-item">
            <p class="subtitle has-text-grey-light">
              { props.day} {" "} { props.month } {" "} { props.year }
            </p>
          </div>

          <div class="level-item">
            <p class="subtitle">
              <a href="/">
                <span class="icon-text">
                  <span>{"Next day"}</span>
                  <span class="icon">
                    <i class="fa-solid fa-angle-right"></i>
                  </span>
                </span>
              </a>
            </p>
          </div>
        </nav>

        <div class="content" ref={self.node_ref.clone()} />
      </div>
    }
  }

  fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
    let html = match msg {
      Msg::GotWeek(Some(html)) => html,
      _ => "nope".to_owned(),
    };

    let el = self.node_ref.cast::<web_sys::Element>().unwrap();
    el.set_inner_html(&html);
    // inject_tags_with_attributes(el);

    true
  }
}

/// Iterate on tags and add the approriate classes we might want. For instance, raw HTML (coming from a Markdown
/// document) is unlikely to have .title, .block, etc. etc. so we are going to automatically add them here.
fn inject_tags_with_attributes(el: Element) {
  match el.tag_name().as_str() {
    "H1" => {
      let _ = el.set_attribute("class", "title");
    }

    "H2" => {
      let _ = el.set_attribute("class", "subtitle");
    }

    "P" => {
      let _ = el.set_attribute("class", "has-text-justified");
    }

    "BLOCKQUOTE" => {
      let _ = el.set_attribute("class", "has-text-justified");
    }

    _ => (),
  }

  let children = el.children();
  for i in 0..children.length() {
    let child = children.get_with_index(i);
    if let Some(child) = child {
      inject_tags_with_attributes(child);
    }
  }
}
