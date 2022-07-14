mod api;
mod config;

use crate::config::Config;
use notify::Watcher;
use rocket::{
  fs::{FileServer, NamedFile},
  get, launch,
  log::LogLevel,
  routes, State,
};
use std::{
  net::{IpAddr, Ipv4Addr},
  path::PathBuf,
  process::exit,
  sync::mpsc,
  thread,
  time::Duration,
};
use twin::news::NewsState;

struct BackendState {
  index_html_path: PathBuf,
}

impl BackendState {
  fn new(config: &Config) -> Self {
    Self {
      index_html_path: config.webapp_dir.join("index.html"),
    }
  }
}

#[get("/<_whocares..>", rank = 20)]
async fn serve_index_html(_whocares: PathBuf, state: &State<BackendState>) -> Option<NamedFile> {
  NamedFile::open(&state.index_html_path).await.ok()
}

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

      // serve the webapp directly, because fuck it
      let backend_state = BackendState::new(&config);
      let webapp_serve = FileServer::new(&config.webapp_dir, rocket::fs::Options::Index);

      rocket::custom(rocket_config)
        .manage(state)
        .manage(backend_state)
        .mount("/api", routes![api::root, api::latest, api::by_key])
        .mount("/", webapp_serve)
        .mount("/", routes![serve_index_html])
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
