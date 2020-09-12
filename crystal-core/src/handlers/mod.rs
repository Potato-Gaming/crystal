use crate::lockfile::Lockfile;
use league_client::models::LolSummonerSummoner;
use reqwest::blocking::Client;
use reqwest::header::AUTHORIZATION;
use snafu::{Backtrace, OptionExt, ResultExt, Snafu};

pub fn get_current_summoner(lockfile: &'static Lockfile) -> Result<LolSummonerSummoner> {
  let details = lockfile.get_details().unwrap();
  let details = details.context(LockfileMissing)?;
  let base_uri = format!(
    "{}://{}:{}",
    details.protocol, details.address, details.port,
  );
  let uri = format!("{}{}", base_uri, "/lol-summoner/v1/current-summoner");
  let auth = format!("Basic {}", details.b64_auth);

  let client = Client::builder()
    .danger_accept_invalid_hostnames(true)
    .danger_accept_invalid_certs(true)
    .build()
    .unwrap();

  let res = client
    .get(&uri)
    .header(AUTHORIZATION, auth)
    .send()
    .context(RequestFailed)?;

  let summoner = res.json::<LolSummonerSummoner>().context(JsonParse)?;

  Ok(summoner)
}

pub type Result<T, E = HandlersError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum HandlersError {
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
