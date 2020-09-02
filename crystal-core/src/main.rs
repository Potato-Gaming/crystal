#[cfg(target_family = "unix")]
extern crate openssl;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate gotham_derive;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

use crate::handlers::empty_handler;
use dotenv::dotenv;
use gotham::middleware::logger::RequestLogger;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::set::{finalize_pipeline_set, new_pipeline_set};
use gotham::router::builder::*;
use gotham::router::Router;
use std::sync::mpsc::{channel, Receiver, Sender};

mod events;
mod handlers;
mod lockfile;

use lockfile::Lockfile;

lazy_static! {
    static ref LOCKFILE: Lockfile = Lockfile::new();
}

fn main() {
    #[cfg(target_family = "unix")]
    openssl_probe::init_ssl_cert_env_vars();

    dotenv().ok();
    pretty_env_logger::init();

    let (tx, rx): (
        Sender<events::LockfileEvent>,
        Receiver<events::LockfileEvent>,
    ) = channel();

    lockfile::watch_lockfile(&LOCKFILE, tx).unwrap();
    events::listen(&LOCKFILE, rx);

    let addr = "127.0.0.1:7878";
    info!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}

fn router() -> Router {
    let pipelines = new_pipeline_set();
    let (pipelines, default) = pipelines.add(
        new_pipeline()
            .add(RequestLogger::new(log::Level::Info))
            .build(),
    );

    let pipeline_set = finalize_pipeline_set(pipelines);
    let default_chain = (default, ());

    build_router(default_chain, pipeline_set, |route| {
        route.get_or_head("/").to(empty_handler);

        route
            .get("/v1/lockfile")
            .to(handlers::lockfile::lockfile_handler);
    })
}
