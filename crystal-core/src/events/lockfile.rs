use super::league_client::LeagueEventsWatcher;
use super::league_events::LeagueEvent;
use crate::Lockfile;
use crossbeam::channel::{Receiver, Sender};
use std::thread;

pub fn listen(lockfile: &'static Lockfile, rx: Receiver<LockfileEvent>, tx: Sender<LeagueEvent>) {
  thread::spawn(move || {
    let mut sub = LeagueEventsWatcher::new(lockfile, tx);

    sub.connect().unwrap();

    loop {
      match rx.recv() {
        Ok(LockfileEvent::Start) => {
          info!("Starting events listener");
          sub.connect().unwrap();
        }
        Ok(LockfileEvent::Restart) => {
          info!("Restarting events listener");
          sub.disconnect().unwrap();
          sub.connect().unwrap();
        }
        Ok(LockfileEvent::Stop) => {
          info!("Stopping events listener");
          sub.disconnect().unwrap();
        }
        Err(e) => {
          panic!("Event listener broke!: {:?}", e);
        }
      }
    }
  });
}

pub enum LockfileEvent {
  Start,
  Restart,
  Stop,
}
