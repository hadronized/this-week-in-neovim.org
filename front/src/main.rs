mod components;
mod router;

use crate::router::Route;
use yew::{function_component, html};
use yew_router::{BrowserRouter, Switch};

#[function_component(App)]
fn app() -> Html {
  html! {
    <BrowserRouter>
      <Switch<Route> render={Switch::render(Route::switch)} />
    </BrowserRouter>
  }
}

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  yew::start_app::<App>();
}
