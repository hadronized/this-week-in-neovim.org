mod config;
mod cache;
mod html_wrapper;
mod routes;

use crate::{config::Config, cache::Cache};
use notify::Watcher;
use rocket::{
  catchers,
  fairing::AdHoc,
  fs::{FileServer, Options},
  launch,
  log::LogLevel,
};
use std::{
  net::{IpAddr, Ipv4Addr},
  process::exit,
  sync::mpsc,
  thread,
  time::Duration,
};
use twin::news::NewsState;

#[cfg(debug_assertions)]
const CACHE_TTL: Duration = Duration::from_secs(5); // 5s of HTML TTL
#[cfg(not(debug_assertions))]
const CACHE_TTL: Duration = Duration::from_secs(3600 * 24); // 1 day of HTML TTL

#[launch]
fn rocket() -> _ {
  match Config::load() {
    Ok(config) => {
      let mut rocket_config = rocket::Config::default();
      rocket_config.address = IpAddr::V4(Ipv4Addr::UNSPECIFIED);
      rocket_config.port = config.port;
      rocket_config.log_level = LogLevel::Debug;

      let (ignition_tx, ignition_rx) = mpsc::sync_channel(0);
      let state = NewsState::new(&config.news_root);
      run_state(ignition_rx, &config, state.clone());

      let cache = Cache::new(CACHE_TTL);
      cache.schedule_eviction();

      let static_fs = FileServer::new(config.static_dir, Options::default());

      rocket::custom(rocket_config)
        .manage(state)
        .manage(cache)
        .register("/", catchers![routes::not_found::not_found])
        .attach(AdHoc::on_liftoff("state_sync", move |_| {
          Box::pin(async move {
            ignition_tx.send(()).expect("state sync");
          })
        }))
        .mount("/", routes::routes())
        .mount("/static", static_fs)
    }

    Err(err) => {
      eprintln!("cannot start: configuration error: {}", err);
      exit(1)
    }
  }
}

fn run_state(ignition_rx: mpsc::Receiver<()>, config: &Config, state: NewsState) {
  let config = config.clone();
  let _ = thread::spawn(move || {
    ignition_rx
      .recv_timeout(Duration::from_secs(5))
      .expect("timeout while waiting for rocket to launch");

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
