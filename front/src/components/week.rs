use chrono::{Date, Datelike, TimeZone as _, Utc};
use reqwasm::http::Request;
use twin::news::Month;
use yew::{html, Component, Html, NodeRef, Properties};

pub struct Week {
  date: Date<Utc>,
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

    let date = Utc.ymd(props.year as _, props.month as _, props.day as _);

    Self {
      date,
      node_ref: NodeRef::default(),
    }
  }

  fn view(&self, _ctx: &yew::Context<Self>) -> Html {
    let date = &self.date;
    let pred_date = date.pred();
    let pred_month = month_from_date(&pred_date);
    let succ_date = date.succ();
    let succ_month = month_from_date(&succ_date);

    html! {
      <div class="section container">
        <nav class="level">
          <div class="level-item">
            <p class="subtitle">
              <a href={ format!("/{}/{}/{}", pred_date.year(), pred_month, pred_date.day()) }>
                <span class="icon-text">
                  <span class="icon">
                    <i class="fa-solid fa-angle-left"></i>
                  </span>
                  <span>
                    { pred_date.day()} {" "} { pred_month } {" "} { pred_date.year() }
                  </span>
                </span>
              </a>
            </p>
          </div>

          <div class="level-item">
            <p class="subtitle has-text-grey-light">
              { date.day()} {" "} { month_from_date(&date) } {" "} { date.year() }
            </p>
          </div>

          <div class="level-item">
            <p class="subtitle">
              <a href={ format!("/{}/{}/{}", succ_date.year(), succ_month, succ_date.day()) }>
                <span class="icon-text">
                  <span>
                    { succ_date.day()} {" "} { succ_month } {" "} { succ_date.year() }
                  </span>
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

/// Because chrono doesn’t have something smart enough without requiring pulling a fucking dep (num-traits), we do this
/// here because why not.
fn month_from_date(date: &Date<Utc>) -> &'static str {
  match date.month() {
    0 => "Jan",
    1 => "Feb",
    2 => "Mar",
    3 => "Apr",
    4 => "May",
    5 => "Jun",
    6 => "Jul",
    7 => "Aug",
    8 => "Sep",
    9 => "Oct",
    10 => "Nov",
    11 => "Dec",
    _ => "N/A",
  }
}
