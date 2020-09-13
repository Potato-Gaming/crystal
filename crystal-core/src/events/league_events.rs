use league_client::models::{
  LolChampSelectChampSelectSession, LolChampSelectChampSelectSummoner, LolGameflowGameflowSession,
  LolSummonerSummoner, RegionLocale,
};
use route_recognizer::Router;
use serde::{Deserialize, Serialize};
use serde_json::{Error as SerdeJsonError, Value};
use snafu::{Backtrace, ResultExt, Snafu};

/// Naive event parser from a string. Gets a string with the event and parses the relevant
/// information that Crystal is interested in.
pub fn parse_event_from(str_event: &str) -> Result<LeagueEvent> {
  let mut router = Router::new();

  let raw_event: Value = serde_json::from_str(str_event).context(ParseJson)?;

  let event_data = &raw_event[2];
  let uri: &str = match event_data["uri"].as_str() {
    Some(u) => u,
    None => {
      trace!("Invalid event");
      return Ok(LeagueEvent::NotTracked);
    }
  };

  trace!("Event data: {:?}", event_data["data"]);

  router.add(
    "/lol-champ-select/v1/summoners/:slot",
    AllowedRoutes::ChampSelectBySlot,
  );

  router.add(
    "/lol-champ-select/v1/session",
    AllowedRoutes::ChampSelectSession,
  );

  router.add("/lol-gameflow/v1/session", AllowedRoutes::Gameflow);

  router.add(
    "/lol-summoner/v1/current-summoner",
    AllowedRoutes::CurrentSummoner,
  );

  router.add("/riotclient/region-locale", AllowedRoutes::RegionLocale);

  let recognized = match router.recognize(uri) {
    Ok(r) => r,
    Err(e) => {
      debug!("{:?}", e);
      return Ok(LeagueEvent::NotTracked);
    }
  };

  match recognized.handler {
    AllowedRoutes::ChampSelectBySlot => {
      info!("Champ Select by Slot {:?}", recognized.params);
      let reparsed =
        serde_json::from_str::<LeagueEventData<LolChampSelectChampSelectSummoner>>(str_event)
          .context(ParseJson)?;

      let slot: usize = recognized.params["slot"].parse().unwrap();

      return Ok(LeagueEvent::ChampionSelectBySlotId(slot, reparsed.2.data));
    }
    AllowedRoutes::ChampSelectSession => {
      info!("Champ Select Session");

      let reparsed =
        serde_json::from_str::<LeagueEventData<LolChampSelectChampSelectSession>>(str_event)
          .context(ParseJson)?;

      return Ok(LeagueEvent::ChampionSelectSesion(reparsed.2.data));
    }
    AllowedRoutes::Gameflow => {
      info!("Game Flow Session");

      let reparsed = serde_json::from_str::<LeagueEventData<LolGameflowGameflowSession>>(str_event)
        .context(ParseJson)?;

      return Ok(LeagueEvent::Gameflow(reparsed.2.data));
    }
    AllowedRoutes::CurrentSummoner => {
      info!("Current Summoner");

      let reparsed = serde_json::from_str::<LeagueEventData<LolSummonerSummoner>>(str_event)
        .context(ParseJson)?;

      return Ok(LeagueEvent::CurrentSummoner(reparsed.2.data));
    }
    AllowedRoutes::RegionLocale => {
      info!("Region Locale");

      let reparsed =
        serde_json::from_str::<LeagueEventData<RegionLocale>>(str_event).context(ParseJson)?;

      return Ok(LeagueEvent::RegionLocale(reparsed.2.data));
    }
  };
}

type LeagueEventData<T> = (i32, String, EventData<T>);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct EventData<T> {
  pub uri: String,
  pub event_type: String,
  pub data: T,
}

#[derive(Debug, Display, Serialize)]
pub enum LeagueEvent {
  ChampionSelectBySlotId(usize, LolChampSelectChampSelectSummoner),
  ChampionSelectSesion(LolChampSelectChampSelectSession),
  Gameflow(LolGameflowGameflowSession),
  CurrentSummoner(LolSummonerSummoner),
  RegionLocale(RegionLocale),
  NotTracked,
}

#[derive(Debug, Display)]
pub enum AllowedRoutes {
  ChampSelectBySlot,
  ChampSelectSession,
  Gameflow,
  CurrentSummoner,
  RegionLocale,
}

pub type Result<T, E = LeagueEventError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum LeagueEventError {
  #[snafu(display("Parsing json failed: {}", source))]
  ParseJson {
    source: SerdeJsonError,
    backtrace: Backtrace,
  },
}
