use super::league_client::LeagueEventsWatcher;
use crate::Lockfile;
use std::sync::mpsc::Receiver;
use std::thread;

pub fn listen(lockfile: &'static Lockfile, rx: Receiver<LockfileEvent>) {
  thread::spawn(move || {
    let mut sub = LeagueEventsWatcher::new(lockfile);

    sub.connect().unwrap();

    loop {
      match rx.recv() {
        Ok(LockfileEvent::Start) => {
          debug!("Starting events listener");
          sub.connect().unwrap();
        }
        Ok(LockfileEvent::Restart) => {
          debug!("Restarting events listener");
          sub.disconnect();
          sub.connect().unwrap();
        }
        Ok(LockfileEvent::Stop) => {
          debug!("Stopping events listener");
          sub.disconnect();
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
