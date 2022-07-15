use reqwasm::http::Request;
use twin::news::{Month, News, NewsKey};
use yew::{html, Component, Html, NodeRef, Properties};

pub struct Week {
  node_ref: NodeRef,
  prev_key: Option<NewsKey>,
  next_key: Option<NewsKey>,
}

#[derive(Debug)]
pub enum Msg {
  GotWeek(Option<News>),
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
        .json()
        .await
        .unwrap();

      Msg::GotWeek(Some(news))
    });

    Self {
      node_ref: NodeRef::default(),
      prev_key: None,
      next_key: None,
    }
  }

  fn view(&self, ctx: &yew::Context<Self>) -> Html {
    let props = ctx.props();

    let prev_date_html = if let Some(prev) = self.prev_key {
      html! {
        <p class="subtitle">
          <a href={ format!("/{}/{}/{}", prev.year, prev.month, prev.day) }>
            <span class="icon-text">
              <span class="icon">
                <i class="fa-solid fa-angle-left"></i>
              </span>
              <span>
                { prev.day } {" "} { prev.month } {" "} { prev.year }
              </span>
            </span>
          </a>
        </p>

      }
    } else {
      html! {}
    };

    let next_date_html = if let Some(next) = self.next_key {
      html! {
        <p class="subtitle">
          <a href={ format!("/{}/{}/{}", next.year, next.month, next.day) }>
            <span class="icon-text">
              <span>
                { next.day } {" "} { next.month } {" "} { next.year }
              </span>
              <span class="icon">
                <i class="fa-solid fa-angle-right"></i>
              </span>
            </span>
          </a>
        </p>
      }
    } else {
      html! {}
    };

    html! {
      <div class="section container">
        <nav class="level">
          <div class="level-item">
            { prev_date_html }
          </div>

          <div class="level-item">
            <p class="subtitle has-text-grey-light">
              { props.day } {" "} { props.month } {" "} { props.year }
            </p>
          </div>

          <div class="level-item">
            { next_date_html }
          </div>
        </nav>

        <div class="content" ref={self.node_ref.clone()} />
      </div>
    }
  }

  fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
    if let Msg::GotWeek(Some(news)) = msg {
      let el = self.node_ref.cast::<web_sys::Element>().unwrap();
      el.set_inner_html(&news.html);

      self.prev_key = news.prev;
      self.next_key = news.next;
    }

    true
  }
}
