use crate::lockfile::Lockfile;
use reqwest::blocking::Client;
use reqwest::header::AUTHORIZATION;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use snafu::{Backtrace, OptionExt, ResultExt, Snafu};

pub struct LeagueApi {
  lockfile: &'static Lockfile,
}

impl LeagueApi {
  pub fn new(lockfile: &'static Lockfile) -> Self {
    Self { lockfile }
  }

  pub fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
    let client = self.build_client().unwrap();
    let (base_uri, auth) = self.forge_base_uri().unwrap();

    let uri = format!("{}{}", base_uri, path);
    println!("uri {}", uri);

    let res = client
      .get(&uri)
      .header(AUTHORIZATION, &auth)
      .send()
      .context(RequestFailed)?;

    let parsed = res.json::<T>().context(JsonParse)?;

    Ok(parsed)
  }

  fn forge_base_uri(&self) -> Result<(String, String)> {
    let details = self.lockfile.get_details().unwrap();
    let details = details.context(LockfileMissing)?;
    let base_uri = format!(
      "{}://{}:{}",
      details.protocol, details.address, details.port,
    );
    let auth = format!("Basic {}", details.b64_auth);

    Ok((base_uri, auth))
  }

  fn build_client(&self) -> Result<Client> {
    let client = Client::builder()
      .danger_accept_invalid_hostnames(true)
      .danger_accept_invalid_certs(true)
      .build()
      .unwrap();

    Ok(client)
  }
}

pub type Result<T, E = LeagueApiError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum LeagueApiError {
  #[snafu(display("No Lockfile exists"))]
  LockfileMissing,

  #[snafu(display("Could not complete request: {}", source))]
  RequestFailed {
    source: reqwest::Error,
    backtrace: Backtrace,
  },

  #[snafu(display("Could not parse request: {}", source))]
  JsonParse {
    source: reqwest::Error,
    backtrace: Backtrace,
  },
}
