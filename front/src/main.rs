mod components;
mod router;

use crate::components::home::HomeComponent;

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  yew::start_app::<HomeComponent>();
}
