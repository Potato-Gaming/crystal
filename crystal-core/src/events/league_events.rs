use league_client::models::{LolChampSelectChampSelectSession, LolChampSelectChampSelectSummoner};
use regex::RegexSet;
use route_recognizer::Router;
use serde_json::{Error as SerdeJsonError, Value};
use snafu::{Backtrace, ResultExt, Snafu};

/// Naive event parser from a string. Gets a string with the event and parses the relevant
/// information that Crystal is interested in.
pub fn parse_event_from(str_event: &str) -> Result<LeagueEvent> {
  let set = RegexSet::new(&[r"/lol-champ-select/", r"/lol-champ-select-legacy/"]).unwrap();
  let mut router = Router::new();

  let matches: Vec<_> = set.matches(str_event).into_iter().collect();

  if matches.len() == 0 {
    trace!("Event is not relevant");
    return Ok(LeagueEvent::NotTracked);
  }

  let raw_event: Value = serde_json::from_str(str_event).context(ParseJson)?;
  trace!("Parsed event: {:?}", raw_event);

  let event_data = &raw_event[2];
  let uri: String = event_data["uri"].to_string();

  trace!("URI: {}", uri);

  router.add(
    "/lol-champ-select/v1/summoner/:slot",
    AllowedRoutes::ChampSelectBySlot,
  );

  router.add(
    "/lol-champ-select/v1/session",
    AllowedRoutes::ChampSelectSession,
  );

  let recognized = match router.recognize(&uri) {
    Ok(r) => r,
    Err(e) => {
      debug!("{:?}", e);
      return Ok(LeagueEvent::NotTracked);
    }
  };

  match recognized.handler {
    AllowedRoutes::ChampSelectBySlot => {
      trace!("Champ Select by Slot {:?}", recognized.params);

      return Ok(LeagueEvent::ChampionSelectBySlotId(
        0,
        LolChampSelectChampSelectSummoner::new(),
      ));
    }
    AllowedRoutes::ChampSelectSession => {
      trace!("Champ Select Session");

      return Ok(LeagueEvent::ChampionSelectSesion(
        LolChampSelectChampSelectSession::new(),
      ));
    }
  };
}

#[derive(Debug, Display)]
pub enum LeagueEvent {
  LobbyEnter,
  LobbyExit,
  ChampionSelectStart,
  ChampionSelectDone,
  ChampionSelectBySlotId(u8, LolChampSelectChampSelectSummoner),
  ChampionSelectSesion(LolChampSelectChampSelectSession),
  NotTracked,
}

#[derive(Debug, Display)]
pub enum AllowedRoutes {
  ChampSelectBySlot,
  ChampSelectSession,
}

pub struct LeagueApiEvent {
  eventType: LeagueEventType,
  uri: String,
  data: LeagueEventData,
}

// There's definitely more event types.
#[derive(Debug, Display)]
pub enum LeagueEventType {
  Create,
  Update,
  Delete,
}

pub struct LeagueEventData {
  action: LeagueEventAction,
}

#[derive(Debug, Display)]
pub enum LeagueEventAction {
  Idle,
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
