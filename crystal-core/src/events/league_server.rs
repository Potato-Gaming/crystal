use super::league_events::LeagueEvent;
use std::sync::mpsc::Receiver;

pub fn start_websocket_server(rx: Receiver<LeagueEvent>) {
  trace!("Starting websocket server");
}
