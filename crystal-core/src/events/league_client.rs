use super::league_events::{parse_event_from, LeagueEvent};
use crate::lockfile::{Lockfile, LockfileError};
use crossbeam::channel::Sender;
use native_tls::{Error as TlsError, TlsConnector};
use snafu::{Backtrace, OptionExt, ResultExt, Snafu};
use std::thread;
use std::time::Duration;
use websocket::client::sync::Client;
use websocket::client::ParseError;
use websocket::header::{Authorization, Basic, Headers};
use websocket::stream::sync::{TcpStream, TlsStream};
use websocket::{ClientBuilder, Message, OwnedMessage, WebSocketError};

pub struct LeagueEventsWatcher {
  status: LeagueSubscriberStatus,
  lockfile: &'static Lockfile,
  client: Option<Client<TlsStream<TcpStream>>>,
  retries: usize,
  tx: Sender<LeagueEvent>,
}

impl LeagueEventsWatcher {
  pub fn new(lockfile: &'static Lockfile, tx: Sender<LeagueEvent>) -> Self {
    Self {
      status: LeagueSubscriberStatus::Idle,
      lockfile,
      client: None,
      retries: 0,
      tx,
    }
  }

  pub fn connect(&mut self) -> Result<()> {
    if self.status == LeagueSubscriberStatus::Connected {
      debug!("Attempting to connect, but connection already started");

      return Ok(());
    }

    self.init_client()?;

    let client = match &mut self.client {
      Some(client) => client,
      None => {
        return Ok(());
      }
    };

    self.status = LeagueSubscriberStatus::Connected;
    let message = Message::text("[5,\"OnJsonApiEvent\"]");

    match client.send_message(&message) {
      Ok(_) => {}
      Err(e) => {
        debug!("Unable to send a message, closing connection: {:?}", e);
        self.disconnect()?;

        return Ok(());
      }
    };

    for message in client.incoming_messages() {
      let message = match message {
        Ok(m) => m,
        Err(e) => {
          debug!("Error receiving message: {:?}", e);
          self.disconnect()?;
          return Ok(());
        }
      };

      match message {
        OwnedMessage::Text(txt) => {
          let event = parse_event_from(&txt).unwrap();
          debug!("Event {:?}", event);
          match self.tx.send(event) {
            Ok(_) => {}
            Err(_) => {
              warn!("Could not send message");
            }
          }
        }
        OwnedMessage::Ping(data) => {
          trace!("Ping: {:?}", data);
        }
        OwnedMessage::Pong(data) => {
          trace!("Pong: {:?}", data);
        }
        OwnedMessage::Binary(bin) => {
          trace!("Binary: {:?}", bin);
        }
        OwnedMessage::Close(_) => {
          debug!("Closed connection");
          self.status = LeagueSubscriberStatus::Idle;
          return Ok(());
        }
      }
    }

    info!("Listener done");

    Ok(())
  }

  pub fn disconnect(&mut self) -> Result<()> {
    let client = match &self.client {
      Some(c) => c,
      None => {
        return Ok(());
      }
    };

    self.status = LeagueSubscriberStatus::Idle;
    client.shutdown().context(Shutdown)?;

    Ok(())
  }

  fn init_client(&mut self) -> Result<()> {
    trace!("Subscribing to League Client");
    let lockfile = self.lockfile.get_details().context(LockfileIssue)?;

    if lockfile.is_none() {
      info!("Lockfile is not ready");
      return Ok(());
    }

    let l = lockfile.context(NoLockfile)?;
    let mut builder = TlsConnector::builder();
    let builder = builder.danger_accept_invalid_certs(true);
    let connector = builder.build().context(TlsIssue)?;

    let mut headers = Headers::new();
    headers.set(Authorization(Basic {
      username: l.username.clone(),
      password: Some(l.password.clone()),
    }));

    let addr = format!("wss://{}:{}/", l.address, l.port);
    info!("Connecting to {:?}", addr);

    let client = ClientBuilder::new(&addr)
      .context(ParseClient)?
      .add_protocol("wamp")
      .custom_headers(&headers)
      .connect_secure(Some(connector));

    let client = match client {
      Ok(c) => c,
      Err(WebSocketError::IoError(err)) => {
        let timeout = Duration::from_millis(500);
        thread::sleep(timeout);

        if self.retries >= 3 {
          panic!("Unable to connect to websocket: {:?}", err);
        }

        self.retries += 1;
        warn!(
          "Unable to connect to websocket, retrying for {} time",
          self.retries
        );

        return self.init_client();
      }
      Err(e) => panic!("{:?}", e),
    };

    self.client = Some(client);
    self.retries = 0;

    Ok(())
  }
}

#[derive(PartialEq)]
pub enum LeagueSubscriberStatus {
  Idle,
  Connected,
}

pub type Result<T, E = LeagueSubscriberError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum LeagueSubscriberError {
  #[snafu(display("Something went wrong with the lockfile: {}", source))]
  LockfileIssue {
    source: LockfileError,
    backtrace: Backtrace,
  },

  #[snafu(display("No lockfile"))]
  NoLockfile { backtrace: Backtrace },

  #[snafu(display("Something went wrong with the TLS: {}", source))]
  TlsIssue {
    source: TlsError,
    backtrace: Backtrace,
  },

  #[snafu(display("Something went wrong parsing the websocket client: {}", source))]
  ParseClient {
    source: ParseError,
    backtrace: Backtrace,
  },

  #[snafu(display("Could not shotdown client: {}", source))]
  Shutdown {
    source: std::io::Error,
    backtrace: Backtrace,
  },
}
