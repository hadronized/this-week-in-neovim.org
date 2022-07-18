mod api;
mod config;
mod html_wrapper;
mod routes;
mod rss;

use crate::config::Config;
use notify::Watcher;
use rocket::{catchers, launch, log::LogLevel};
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

      rocket::custom(rocket_config)
        .manage(state)
        .register("/", catchers![routes::not_found::not_found])
        // .mount("/", webapp_serve)
        .mount("/", routes::routes())
    }

    Err(err) => {
      eprintln!("cannot start: configuration error: {}", err);
      exit(1)
    }
  }
}

fn run_state(config: &Config, state: NewsState) {
  let config = config.clone();
  let _ = thread::spawn(move || {
    thread::sleep(Duration::from_secs(1)); // just wait a bit until rocket is fully initialized

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
    .watch(&config.news_root, notify::RecursiveMode::Recursive)
    .expect("watching news root directory");

  log::debug!("watching directory {}", config.news_root.display());

  for event in rx {
    log::debug!("event: {:?}", event);

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

  log::debug!("watch state exited");
}
