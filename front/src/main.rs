mod components;
mod config;
mod router;

use crate::{components::home::HomeComponent, config::Config};

fn main() {
  wasm_logger::init(wasm_logger::Config::default());

  log::info!("starting…");

  match Config::load() {
    Ok(config) => {
      log::info!("running with config:\n{:#?}", config);
      yew::start_app_with_props::<HomeComponent>(config);
    }

    Err(e) => {
      log::error!("cannot start; config error: {}", e);
    }
  }
}
