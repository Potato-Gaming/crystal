use regex::RegexSet;
use serde_json::{Error as SerdeJsonError, Value};
use snafu::{Backtrace, ResultExt, Snafu};

/// Naive event parser from a string. Gets a string with the event and parses the relevant
/// information that Crystal is interested in.
pub fn parse_event_from(str_event: &str) -> Result<LeagueEvent> {
  let set = RegexSet::new(&[r"/lol-champ-select/", r"/lol-champ-select-legacy/"]).unwrap();

  let matches: Vec<_> = set.matches(str_event).into_iter().collect();

  if matches.len() == 0 {
    trace!("Event is not relevant");
    return Ok(LeagueEvent::NotTracked);
  }

  let raw_event: Value = serde_json::from_str(str_event).context(ParseJson)?;
  trace!("Parsed event: {:?}", raw_event);

  Ok(LeagueEvent::NotTracked)
}

#[derive(Debug, Display)]
pub enum LeagueEvent {
  LobbyEnter,
  LobbyExit,
  ChampionSelectStart,
  ChampionSelectDone,
  ChampSelected,
  NotTracked,
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
