use crate::Lockfile;
// use futures::future::{self, Loop};
// use futures::sink::Sink;
// use futures::stream::Stream;
// use futures::Future;
use native_tls::{TlsConnector, TlsConnectorBuilder};
use std::sync::mpsc::Receiver;
use std::thread;
// use tokio::runtime::current_thread::Runtime;
use websocket::header::{Authorization, Basic, Headers};
// use websocket::r#async::client::{Client, ClientNew};
// use websocket::r#async::TcpStream;
// use websocket::result::WebSocketError;
use websocket::{ClientBuilder, Message, OwnedMessage};

pub fn listen(lockfile: &'static Lockfile, rx: Receiver<EventName>) {
  thread::spawn(move || {
    // let mut runtime = tokio::runtime::current_thread::Builder::new()
    //   .build()
    //   .unwrap();
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
  let err = "Unsupported message";
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
    // .connect_insecure()
    .connect_secure(Some(connector))
    .unwrap();

  // let (mut receiver, mut sender) = client.split().unwrap();

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
  // .async_connect_secure(Some(connector));

  // let listener = client
  //   .and_then(|(duplex, _)| {
  //     // let client: Client<TcpStream> = duplex;
  //     let message = Message::text("[5,\"OnJsonApiEvent\"]");
  //     duplex.send_message(&message).unwrap();

  //     future::loop_fn(duplex, |stream| {
  //       stream
  //         .into_future()
  //         .or_else(|(err, stream)| {
  //           debug!("Could not receive message: {:?}", err);
  //           stream.send(OwnedMessage::Close(None)).map(|s| (None, s))
  //         })
  //         .and_then(|(msg, stream)| -> Box<dyn Future<Item = _, Error = _>> {
  //           match msg {
  //             Some(OwnedMessage::Text(txt)) => {
  //               debug!("Got Message: {:?}", txt);
  //               Box::new(
  //                 stream
  //                   .send(OwnedMessage::Text(txt))
  //                   .map(|s| Loop::Continue(s)),
  //               )
  //             }
  //             Some(OwnedMessage::Binary(bin)) => {
  //               debug!("Got binary: {:?}", bin);

  //               Box::new(
  //                 stream
  //                   .send(OwnedMessage::Binary(bin))
  //                   .map(|s| Loop::Continue(s)),
  //               )
  //             }
  //             Some(OwnedMessage::Ping(data)) => {
  //               debug!("Got Ping Data: {:?}", data);

  //               Box::new(
  //                 stream
  //                   .send(OwnedMessage::Pong(data))
  //                   .map(|s| Loop::Continue(s)),
  //               )
  //             }
  //             Some(OwnedMessage::Close(_)) => {
  //               debug!("Closing stream");

  //               Box::new(
  //                 stream
  //                   .send(OwnedMessage::Close(None))
  //                   .map(|_| Loop::Break(())),
  //               )
  //             }
  //             Some(OwnedMessage::Pong(_)) => Box::new(future::ok(Loop::Continue(stream))),
  //             None => Box::new(future::ok(Loop::Break(()))),
  //           }
  //         })
  //     })
  //   })
  //   .map(|_| {
  //     debug!("In Map!");
  //   })
  //   .or_else(|err| {
  //     debug!("Error! {:?}", err);
  //     Ok(()) as Result<(), ()>
  //   });

  // runtime.block_on(listener).ok();

  // let listener = client
  //   .and_then(|(s, _)| {
  //     s.into_future().map_err(|e| {
  //       debug!("Error: {:?}", e.0);

  //       e.0
  //     })
  //   })
  //   .and_then(|(msg, _)| match msg {
  //     Some(OwnedMessage::Text(txt)) => {
  //       debug!("Got Message: {:?}", txt);
  //       Ok(())
  //       // Ok(txt.parse().unwrap())
  //     }
  //     e => {
  //       debug!("Something went wrong: {:?}", e);

  //       Err(WebSocketError::ProtocolError(err))
  //     }
  //   });

  // match runtime.block_on(listener) {
  //   Ok(msg) => {
  //     debug!("Listening... {:?}", msg);
  //   }
  //   Err(e) => {
  //     debug!("Failed to listen to events: {:?}", e);
  //   }
  // };

  debug!("Listener done");
}

pub enum EventName {
  Start,
  Restart,
}
