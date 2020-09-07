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

#[macro_use]
extern crate strum_macros;

use crate::handlers::empty_handler;
use crossbeam::channel::{unbounded, Receiver, Sender};
use dotenv::dotenv;
use gotham::middleware::logger::RequestLogger;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::set::{finalize_pipeline_set, new_pipeline_set};
use gotham::router::builder::*;
use gotham::router::Router;

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
    ) = unbounded();

    let (tx_ws, rx_ws): (Sender<events::LeagueEvent>, Receiver<events::LeagueEvent>) = unbounded();

    lockfile::watch_lockfile(&LOCKFILE, tx).unwrap();
    events::listen(&LOCKFILE, rx, tx_ws);
    events::start_websocket_server(rx_ws);

    let crystal_port = std::env::var("CRYSTAL_PORT").unwrap_or(String::from("7878"));

    let addr = format!("127.0.0.1:{}", crystal_port);
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
