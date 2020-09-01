use crate::events;
use crossbeam::sync::{ShardedLock, ShardedLockReadGuard, ShardedLockWriteGuard};
use league_client_connector::RiotLockFile;
use league_client_connector::{LeagueClientConnector, LeagueConnectorError};
use notify::{Error as NotifyError, RecommendedWatcher, RecursiveMode, Watcher};
use snafu::{ResultExt, Snafu};
use std::sync::mpsc::{channel, RecvError, Sender};
use std::sync::PoisonError;
use std::thread;
use std::time::Duration;

use crate::LOCKFILE;

#[derive(StateData)]
pub struct Lockfile {
    pub inner: ShardedLock<Option<RiotLockFile>>,
}

impl Lockfile {
    pub fn new() -> Self {
        Self {
            inner: ShardedLock::new(None),
        }
    }

    pub fn get_details(&self) -> Result<Option<RiotLockFile>> {
        let lockfile = LOCKFILE.inner.read().context(ReadLockfile)?;
        let lockfile = lockfile.as_ref();

        match lockfile {
            Some(l) => Ok(Some(l.clone())),
            None => Ok(None),
        }
    }

    pub fn release(&self) -> Result<()> {
        // TODO: Properly handle when: https://github.com/shepmaster/snafu/issues/236
        let mut file = LOCKFILE.inner.write().context(WriteLockfile)?;
        *file = None;

        Ok(())
    }

    pub fn update(&self) -> Result<()> {
        let mut file = LOCKFILE.inner.write().context(WriteLockfile)?;
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

        Ok(())
    }

    pub fn did_change(&self) -> Result<bool> {
        let prev = self.get_details()?;
        self.update()?;
        let current = self.get_details()?;

        if prev.is_none() && current.is_none() {
            return Ok(false);
        }

        if prev.is_none() && current.is_some() {
            return Ok(true);
        }

        if prev.is_some() && current.is_none() {
            return Ok(true);
        }

        let prev = match prev {
            Some(p) => p,
            None => unreachable!(),
        };
        let current = match current {
            Some(c) => c,
            None => unreachable!(),
        };

        let changed = prev.address != current.address
            || prev.b64_auth != current.b64_auth
            || prev.port != current.port;

        Ok(changed)
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
        let lockfile_path = LeagueClientConnector::get_path()
            .context(LeagueConnectorIssue)
            .unwrap();
        LOCKFILE.update().unwrap();
        let _ = ev_tx.send(events::LockfileEvent::Start);

        watcher
            .watch(lockfile_path, RecursiveMode::NonRecursive)
            .context(WatcherIssue)
            .unwrap();

        loop {
            let event = rx.recv().context(ReceiveMessage).unwrap();

            match event {
                notify::DebouncedEvent::Create { .. } => {
                    debug!("Lockfile created");
                    if lockfile.did_change().unwrap() {
                        let _ = ev_tx.send(events::LockfileEvent::Restart);
                    }
                }
                notify::DebouncedEvent::Write { .. } => {
                    debug!("Lockfile written");
                    if lockfile.did_change().unwrap() {
                        let _ = ev_tx.send(events::LockfileEvent::Restart);
                    }
                }
                notify::DebouncedEvent::Remove { .. } => {
                    debug!("Lockfile deleted");
                    lockfile.release().unwrap();
                    let _ = ev_tx.send(events::LockfileEvent::Stop);
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

pub type Result<T, E = LockfileError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum LockfileError {
    // Disabled until https://github.com/shepmaster/snafu/issues/236
    #[snafu(display("Unable to read lockfile: {}", source))]
    ReadLockfile {
        source: PoisonError<ShardedLockReadGuard<'static, Option<RiotLockFile>>>,
    },

    WriteLockfile {
        source: PoisonError<ShardedLockWriteGuard<'static, Option<RiotLockFile>>>,
    },

    #[snafu(display("Failed in watcher: {}", source))]
    WatcherIssue { source: NotifyError },

    #[snafu(display("Something went wrong with the league connector: {}", source))]
    LeagueConnectorIssue { source: LeagueConnectorError },

    #[snafu(display("Failed to receive message: {}", source))]
    ReceiveMessage { source: RecvError },
}
