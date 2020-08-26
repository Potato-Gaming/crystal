#[cfg(target_family = "unix")]
extern crate openssl;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate gotham_derive;

use dotenv::dotenv;
use gotham::middleware::logger::RequestLogger;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::set::{finalize_pipeline_set, new_pipeline_set};
use gotham::router::builder::*;
use gotham::router::Router;
use gotham_middleware_diesel::{self, DieselMiddleware};
use crate::handlers::empty_handler;

mod handlers;

fn main() {
    #[cfg(target_family = "unix")]
    openssl_probe::init_ssl_cert_env_vars();

    dotenv().ok();
    env_logger::init();

    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
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
    })
}
