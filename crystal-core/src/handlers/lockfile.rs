use crate::LOCKFILE;
use futures::future;
use gotham::handler::HandlerFuture;
use gotham::helpers::http::response::create_response;
use gotham::state::State;
use hyper::StatusCode;
use league_client_connector::RiotLockFile;
use mime;

#[derive(Serialize)]
struct NoLockfile {
    error: String,
}

pub fn lockfile_handler(state: State) -> Box<HandlerFuture> {
    let lockfile = LOCKFILE.inner.clone();
    let lockfile = lockfile.lock().unwrap();
    let lockfile: Option<&RiotLockFile> = lockfile.as_ref();

    let body = match lockfile {
        Some(l) => serde_json::to_string(l).expect("Unable to parse lockfile"),
        None => {
            let empty = NoLockfile {
                error: "Lockfile is not ready yet".to_string(),
            };

            serde_json::to_string(&empty).unwrap()
        }
    };

    let res = create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body);

    let f = future::ok((state, res));

    Box::new(f)
}
