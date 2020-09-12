#[cfg(target_family = "unix")]
extern crate openssl;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

#[macro_use]
extern crate strum_macros;

pub mod events;
pub mod handlers;
pub mod lockfile;

use lockfile::Lockfile;

lazy_static! {
  pub static ref LOCKFILE: Lockfile = Lockfile::new();
}
