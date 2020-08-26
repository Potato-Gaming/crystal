use league_client_connector::LeagueClientConnector;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

pub fn watch_lockfile() {
    let event_timeout = Duration::from_millis(500);
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, event_timeout).unwrap();
    let lockfile = LeagueClientConnector::get_path().unwrap();

    thread::spawn(move || {
        let rx = rx;

        watcher
            .watch(lockfile, RecursiveMode::NonRecursive)
            .unwrap();

        loop {
            match rx.recv() {
                Ok(event) => match event {
                    notify::DebouncedEvent::Create { .. } => {
                        debug!("Lockfile created");
                    }
                    _ => {
                        unimplemented!();
                    }
                },
                Err(e) => {
                    panic!("Something went wrong! {:}", e);
                }
            }
        }
    });
}
