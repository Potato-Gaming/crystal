use crate::Lockfile;
use league_client_connector::{LeagueClientConnector, LeagueConnectorError};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

pub fn watch_lockfile(lockfile: &'static Lockfile) {
    let event_timeout = Duration::from_millis(500);
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, event_timeout).unwrap();

    thread::spawn(move || {
        let rx = rx;
        let mut league_client_started = false;

        while !league_client_started {
            match LeagueClientConnector::get_path() {
                Ok(_) => league_client_started = true,
                Err(LeagueConnectorError::NoInstallationPath { .. }) => {
                    debug!("League Client is closed, waiting 500ms");
                    // Wait for 500ms before checking again.
                    thread::sleep(event_timeout);
                }
                e => {
                    panic!("Something went wrong! {:?}", e);
                }
            }
        }

        debug!("League Client Detected!");
        let lockfile_path = LeagueClientConnector::get_path().unwrap();
        update_lockfile(lockfile);

        watcher
            .watch(lockfile_path, RecursiveMode::NonRecursive)
            .unwrap();

        loop {
            match rx.recv() {
                Ok(event) => match event {
                    notify::DebouncedEvent::Create { .. } => {
                        debug!("Lockfile created");
                        update_lockfile(lockfile);
                    }
                    notify::DebouncedEvent::Write { .. } => {
                        debug!("Lockfile written");
                        update_lockfile(lockfile);
                    }
                    notify::DebouncedEvent::Remove { .. } => {
                        debug!("Lockfile deleted");
                        let mut file = lockfile.inner.lock().unwrap();
                        *file = None;
                    }
                    notify::DebouncedEvent::NoticeWrite { .. } => {
                        debug!("Notice write");
                    }
                    notify::DebouncedEvent::NoticeRemove { .. } => {
                        debug!("Notice remove");
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

fn update_lockfile(lockfile: &Lockfile) {
    let mut file = lockfile.inner.lock().unwrap();
    match LeagueClientConnector::parse_lockfile() {
        Ok(l) => {
            *file = Some(l);
        }
        Err(LeagueConnectorError::NoInstallationPath { .. }) => {
            debug!("Attempted to update lockfile, but client appears to be closed")
        }
        e => {
            panic!("Something went wrong! {:?}", e);
        }
    }
}
