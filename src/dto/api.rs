use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summoner {
    pub profile_icon_id: i32,
    pub name: String,
    pub puuid: String,
    pub summoner_level: u64,
    pub revision_date: u64,
    pub id: String,
    pub account_id: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChampionInfo {
    free_champion_ids: Vec<i64>,
    free_champion_ids_for_new_players: Vec<i64>,
    max_new_player_level: i64,
}