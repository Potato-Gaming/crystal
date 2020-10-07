#[cfg(target_family = "linux")]
extern crate openssl;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

#[macro_use]
extern crate strum_macros;

#[macro_use]
extern crate snafu_derive;

pub mod events;
pub mod handlers;
pub mod lockfile;

use lockfile::Lockfile;

lazy_static! {
    pub static ref LOCKFILE: Lockfile = Lockfile::new();
}
