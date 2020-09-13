mod api;
pub use api::*;

use league_client::models::{LolSummonerSummoner, RegionLocale};
use snafu::{Backtrace, ResultExt, Snafu};

pub fn get_current_summoner(api: &LeagueApi) -> Result<LolSummonerSummoner> {
  let summoner = api
    .get::<LolSummonerSummoner>("/lol-summoner/v1/current-summoner")
    .context(FetchApi)?;

  Ok(summoner)
}

pub fn get_region_locale(api: &LeagueApi) -> Result<RegionLocale> {
  let locale = api
    .get::<RegionLocale>("/riotclient/get_region_locale")
    .context(FetchApi)?;

  Ok(locale)
}

pub type Result<T, E = HandlersError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum HandlersError {
  #[snafu(display("Could not fetch with League API: {}", source))]
  FetchApi {
    source: LeagueApiError,
    backtrace: Backtrace,
  },
}
