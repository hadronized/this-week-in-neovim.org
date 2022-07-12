mod api;
mod config;

use crate::config::Config;
use notify::Watcher;
use rocket::{launch, log::LogLevel, routes};
use std::{
  net::{IpAddr, Ipv4Addr},
  process::exit,
  sync::mpsc,
  thread,
  time::Duration,
};
use twin::news::NewsState;

#[launch]
fn rocket() -> _ {
  match Config::load() {
    Ok(config) => {
      let mut rocket_config = rocket::Config::default();
      rocket_config.address = IpAddr::V4(Ipv4Addr::UNSPECIFIED);
      rocket_config.port = config.port;
      rocket_config.log_level = LogLevel::Debug;

      let state = NewsState::new(&config.news_root);
      run_state(&config, state.clone());

      rocket::custom(rocket_config).manage(state).mount(
        "/api",
        routes![api::root, api::latest, api::by_year_week_nb],
      )
    }

    Err(err) => {
      log::error!("cannot start: configuration error: {}", err);
      exit(1)
    }
  }
}

fn run_state(config: &Config, state: NewsState) {
  let config = config.clone();
  let _ = thread::spawn(move || {
    if let Err(err) = state
      .news_store()
      .write()
      .expect("news store")
      .populate_from_root()
    {
      log::error!(
        "cannot populate from root ({}): {}",
        config.news_root.display(),
        err
      );
    }

    watch_state(&config, state);
  });
}

fn watch_state(config: &Config, state: NewsState) {
  let (sx, rx) = mpsc::channel();
  let mut watcher = notify::watcher(sx, Duration::from_millis(200)).expect("state watcher");
  watcher
    .watch(&config.news_root, notify::RecursiveMode::NonRecursive)
    .expect("watching news root directory");

  for event in rx {
    match event {
      notify::DebouncedEvent::Create(_path) | notify::DebouncedEvent::Write(_path) => {
        // FIXME: suboptimal; we should be parsing path and use NewsStore::update instead of recomputing everything
        if let Err(err) = state
          .news_store()
          .write()
          .expect("news store")
          .populate_from_root()
        {
          log::error!("cannot repopulate news: {}", err);
        }
      }

      _ => (),
    }
  }
}
