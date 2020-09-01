use crate::events;
use league_client_connector::RiotLockFile;
use league_client_connector::{LeagueClientConnector, LeagueConnectorError};
use notify::{Error as NotifyError, RecommendedWatcher, RecursiveMode, Watcher};
use snafu::{ResultExt, Snafu};
use std::sync::mpsc::{channel, RecvError, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::LOCKFILE;

#[derive(Clone, StateData)]
pub struct Lockfile {
    pub inner: Arc<Mutex<Option<RiotLockFile>>>,
}

impl Lockfile {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(None)),
        }
    }

    pub fn get_details(&self) -> Result<Option<RiotLockFile>> {
        let lockfile = LOCKFILE.inner.clone();
        // TODO: Properly handle when: https://github.com/shepmaster/snafu/issues/236
        let lockfile = lockfile.lock().unwrap();
        let lockfile: Option<&RiotLockFile> = lockfile.as_ref();

        match lockfile {
            Some(l) => Ok(Some(l.clone())),
            None => Ok(None),
        }
    }

    pub fn release(&self) -> Result<()> {
        // TODO: Properly handle when: https://github.com/shepmaster/snafu/issues/236
        let mut file = self.inner.lock().unwrap();
        *file = None;

        Ok(())
    }
}

pub fn watch_lockfile(
    lockfile: &'static Lockfile,
    ev_tx: Sender<events::LockfileEvent>,
) -> Result<()> {
    let event_timeout = Duration::from_millis(500);
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, event_timeout)
        .context(WatcherIssue)
        .unwrap();

    thread::spawn(move || -> Result<()> {
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
        let lockfile_path = LeagueClientConnector::get_path().context(LeagueConnectorIssue)?;
        update_lockfile(lockfile);
        let _ = ev_tx.send(events::LockfileEvent::Start);

        watcher
            .watch(lockfile_path, RecursiveMode::NonRecursive)
            .context(WatcherIssue)?;

        loop {
            let event = rx.recv().context(ReceiveMessage)?;

            match event {
                notify::DebouncedEvent::Create { .. } => {
                    debug!("Lockfile created");
                    update_lockfile(lockfile);
                    let _ = ev_tx.send(events::LockfileEvent::Restart);
                }
                notify::DebouncedEvent::Write { .. } => {
                    debug!("Lockfile written");
                    update_lockfile(lockfile);
                    let _ = ev_tx.send(events::LockfileEvent::Restart);
                }
                notify::DebouncedEvent::Remove { .. } => {
                    debug!("Lockfile deleted");
                    lockfile.release().unwrap();
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
            };
        }
    });

    Ok(())
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

pub type Result<T, E = LockfileError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum LockfileError {
    // Disabled until https://github.com/shepmaster/snafu/issues/236
    // #[snafu(display("Unable to get lockfile: {}", source))]
    // GetFromMutex {
    //     source: PoisonError<MutexGuard<'_, Option<RiotLockFile>>>,
    // },
    #[snafu(display("Failed in watcher: {}", source))]
    WatcherIssue { source: NotifyError },

    #[snafu(display("Something went wrong with the league connector: {}", source))]
    LeagueConnectorIssue { source: LeagueConnectorError },

    #[snafu(display("Failed to receive message: {}", source))]
    ReceiveMessage { source: RecvError },
}
