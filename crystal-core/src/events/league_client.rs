use crate::Lockfile;
use native_tls::TlsConnector;
use std::thread;
use std::time::Duration;
use websocket::client::sync::Client;
use websocket::header::{Authorization, Basic, Headers};
use websocket::stream::sync::{TcpStream, TlsStream};
use websocket::{ClientBuilder, Message, OwnedMessage, WebSocketError};

pub struct LeagueEventsWatcher {
  status: LeagueSubscriberStatus,
  lockfile: &'static Lockfile,
  client: Option<Client<TlsStream<TcpStream>>>,
  retries: usize,
}

impl LeagueEventsWatcher {
  pub fn new(lockfile: &'static Lockfile) -> Self {
    Self {
      status: LeagueSubscriberStatus::Idle,
      lockfile,
      client: None,
      retries: 0,
    }
  }

  pub fn connect(&mut self) -> Result<(), ()> {
    if self.status == LeagueSubscriberStatus::Connected {
      debug!("Attempting to connect, but connection already started");

      return Ok(());
    }

    self.init_client().unwrap();

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
        self.disconnect();

        return Ok(());
      }
    };

    for message in client.incoming_messages() {
      let message = match message {
        Ok(m) => m,
        Err(e) => {
          debug!("Error receiving message: {:?}", e);
          self.disconnect();
          return Ok(());
        }
      };

      match message {
        OwnedMessage::Text(txt) => {
          debug!("Message: {:?}", txt);
        }
        OwnedMessage::Ping(data) => {
          debug!("Ping: {:?}", data);
        }
        OwnedMessage::Pong(data) => {
          debug!("Pong: {:?}", data);
        }
        OwnedMessage::Binary(bin) => {
          debug!("Binary: {:?}", bin);
        }
        OwnedMessage::Close(_) => {
          debug!("Closed connection");
          self.status = LeagueSubscriberStatus::Idle;
          return Ok(());
        }
      }
    }

    debug!("Listener done");

    Ok(())
  }

  pub fn disconnect(&mut self) {
    let client = match &self.client {
      Some(c) => c,
      None => {
        return;
      }
    };

    self.status = LeagueSubscriberStatus::Idle;
    client.shutdown().unwrap();
  }

  fn init_client(&mut self) -> Result<(), ()> {
    debug!("Subscribing to League Client");
    let lockfile = self.lockfile.get_details().unwrap();

    if lockfile.is_none() {
      debug!("Lockfile is not ready");
      return Ok(());
    }

    let l = lockfile.unwrap();
    let mut builder = TlsConnector::builder();
    let builder = builder.danger_accept_invalid_certs(true);
    let connector = builder.build().unwrap();

    let mut headers = Headers::new();
    headers.set(Authorization(Basic {
      username: l.username.clone(),
      password: Some(l.password.clone()),
    }));

    let addr = format!("wss://{}:{}/", l.address, l.port);
    debug!("Connecting to {:?}", addr);

    let client = ClientBuilder::new(&addr)
      .unwrap()
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
        debug!(
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
  Errored,
}
