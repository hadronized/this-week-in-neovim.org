mod components;
mod router;

use crate::components::home::HomeComponent;

fn main() {
  yew::start_app::<HomeComponent>();
}
