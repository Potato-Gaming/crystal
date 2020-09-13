use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  /// Fetches current summoner.
  CurrentSummoner { callback: String, error: String },
}
