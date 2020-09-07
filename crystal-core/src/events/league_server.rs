use super::league_events::LeagueEvent;
// use league_client::models::{LolChampSelectChampSelectSession, LolChampSelectChampSelectSummoner};
use crossbeam::channel::{Receiver, RecvError};
use snafu::{Backtrace, ResultExt, Snafu};
use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;

pub fn start_websocket_server(rx: Receiver<LeagueEvent>) {
  let port = std::env::var("CRYSTAL_WS_PORT").unwrap_or(String::from("7879"));
  let addr = format!("127.0.0.1:{}", port);

  info!("Starting websocket server in: {}", addr);
  thread::spawn(move || {
    let rx = rx;
    let server = Server::bind(addr).context(WebsocketServer).unwrap();

    for request in server.filter_map(Result::ok) {
      let rx = rx.clone();
      thread::spawn(move || {
        let mut client = request.use_protocol("rust-websocket").accept().unwrap();
        let ip = client.peer_addr().unwrap();

        info!("Connection from {}", ip);

        loop {
          let event = rx.recv().context(ReceiveMessage).unwrap();
          let message = OwnedMessage::Text(event.to_string());

          client.send_message(&message).unwrap();
        }
      });
    }
  });
}

pub type Result<T, E = LeagueServerError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum LeagueServerError {
  #[snafu(display("Something went wrong with websocket server: {}", source))]
  WebsocketServer {
    backtrace: Backtrace,
    source: std::io::Error,
  },

  // #[snafu(display("Could not accept a new connection: {:?}", cause))]
  // AcceptConnection {
  //   backtrace: Backtrace,
  //   #[snafu(source)]
  //   cause: (std::net::TcpStream, std::io::Error),
  // },
  #[snafu(display("Unable to receive message: {}", source))]
  ReceiveMessage {
    backtrace: Backtrace,
    source: RecvError,
  },
}
