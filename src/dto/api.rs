#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summoner {
    pub profile_icon_id: i32,
    pub name: String,
    pub puuid: String,
    pub summoner_level: u64,
    pub revision_date: u64,
    pub id: String,
    pub account_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionInfo {
    pub free_champion_ids: Vec<i64>,
    pub free_champion_ids_for_new_players: Vec<i64>,
    pub max_new_player_level: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionMastery {
    pub chest_granted: bool,
    pub champion_level: i32,
    pub champion_points: i32,
    pub champion_id: i64,
    pub champion_points_until_next_level: i64,
    pub last_play_time: i64,
    pub tokens_earned: i64,
    pub champion_points_since_last_level: i64,
    pub summoner_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueInfo {
    pub queue_type: String,
    pub summoner_name: String,
    pub hot_streak: bool,
    pub wins: i64,
    pub veteran: bool,
    pub losses: i64,
    pub rank: String,
    pub tier: String,
    pub inactive: bool,
    pub fresh_blood: bool,
    pub league_id: String,
    pub summoner_id: String,
    pub league_points: i64,
}
