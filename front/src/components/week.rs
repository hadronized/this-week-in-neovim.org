use reqwasm::http::Request;
use twin::news::{LatestNews, Month, News, NewsKey};
use yew::{html, Component, Html, NodeRef, Properties};

pub struct Week {
  date: WeekProps,
  node_ref: NodeRef,
  prev_key: Option<NewsKey>,
  next_key: Option<NewsKey>,
}

#[derive(Debug)]
pub enum Msg {
  Latest(LatestNews),
  Week(News),
}

#[derive(Clone, Default, Eq, Hash, PartialEq, Properties)]
pub struct WeekProps {
  #[prop_or_default]
  pub year: u16,

  #[prop_or_default]
  pub month: Month,

  #[prop_or_default]
  pub day: u8,
}

impl Component for Week {
  type Message = Msg;

  type Properties = WeekProps;

  fn create(ctx: &yew::Context<Self>) -> Self {
    let props: WeekProps = ctx.props().clone();

    ctx.link().send_future(async move {
      if props.year == 0 {
        let latest_news = Request::get("api/latest")
          .send()
          .await
          .unwrap()
          .json()
          .await
          .unwrap();

        Msg::Latest(latest_news)
      } else {
        let news = Request::get(&format!(
          "/api/{}/{}/{}",
          props.year, props.month, props.day
        ))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

        Msg::Week(news)
      }
    });

    Self {
      date: props,
      node_ref: NodeRef::default(),
      prev_key: None,
      next_key: None,
    }
  }

  fn view(&self, _ctx: &yew::Context<Self>) -> Html {
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
              { self.date.day } {" "} { self.date.month } {" "} { self.date.year }
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
    match msg {
      Msg::Week(news) => {
        let el = self.node_ref.cast::<web_sys::Element>().unwrap();
        el.set_inner_html(&news.html);

        self.prev_key = news.prev;
        self.next_key = news.next;
      }

      Msg::Latest(latest_news) => {
        let el = self.node_ref.cast::<web_sys::Element>().unwrap();
        el.set_inner_html(&latest_news.news.html);

        self.prev_key = latest_news.news.prev;
        self.next_key = None;

        self.date = WeekProps {
          year: latest_news.key.year,
          month: latest_news.key.month,
          day: latest_news.key.day,
        };
      }
    }

    true
  }
}
