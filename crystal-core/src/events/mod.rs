use crate::Lockfile;
use native_tls::TlsConnector;
use std::sync::mpsc::Receiver;
use std::thread;
use websocket::header::{Authorization, Basic, Headers};
use websocket::{ClientBuilder, Message, OwnedMessage};

pub fn listen(lockfile: &'static Lockfile, rx: Receiver<EventName>) {
  thread::spawn(move || {
    subscribe_to_client(lockfile);

    loop {
      match rx.recv() {
        Ok(EventName::Start) => {
          debug!("Starting events listener");
          subscribe_to_client(lockfile);
        }
        Ok(EventName::Restart) => {
          debug!("Restarting events listener");
        }
        Err(e) => {
          panic!("Event listener broke!: {:?}", e);
        }
      }
    }
  });
}

fn subscribe_to_client(lockfile: &'static Lockfile) {
  debug!("Subscribing to League Client");
  let lockfile = lockfile.get_details().unwrap();

  if lockfile.is_none() {
    debug!("Lockfile is not ready");
    return;
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

  let mut client = ClientBuilder::new(&addr)
    .unwrap()
    .add_protocol("wamp")
    .custom_headers(&headers)
    .connect_secure(Some(connector))
    .unwrap();

  let message = Message::text("[5,\"OnJsonApiEvent\"]");
  client.send_message(&message).unwrap();

  for message in client.incoming_messages() {
    let message = match message {
      Ok(m) => m,
      Err(e) => {
        debug!("Error receiving message: {:?}", e);
        return;
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
        return;
      }
    }
  }

  debug!("Listener done");
}

pub enum EventName {
  Start,
  Restart,
}
