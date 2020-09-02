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
