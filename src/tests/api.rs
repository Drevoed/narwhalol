use crate::constants::{LanguageCode, Region};
use crate::dto::api::{ChampionInfo, ChampionMastery, Summoner};
use crate::synchronous::client::LeagueAPI;
use crate::synchronous::ddragon::DDragonClient;
use std::sync::Mutex;

lazy_static! {
    static ref DDRAGON_CLIENT: Mutex<DDragonClient> =
        Mutex::new(DDragonClient::new(LanguageCode::RUSSIA).unwrap());
    static ref LEAGUE_CLIENT: LeagueAPI = LeagueAPI::new(Region::NA);
}

#[test]
fn gets_summoner_data() {
    let summoner = LEAGUE_CLIENT
        .get_summoner_by_name("Santorin")
        .expect("Something went wrong");
    assert_eq!(
        &summoner.account_id,
        "rPnj4h5W6OhejxB-AO3hLOQctgZcckqV_82N_8_WuCFdO2A"
    )
}

#[test]

fn gets_champion_info() {
    let champ_info: ChampionInfo = LEAGUE_CLIENT
        .get_champion_info()
        .expect("Could not get champion info");
    assert!(champ_info.free_champion_ids.len() > 10);
    assert!(champ_info.free_champion_ids_for_new_players.len() > 0);
    assert_ne!(champ_info.max_new_player_level, 0)
}

#[test]
fn gets_champion_masteries() {
    let summoner: Summoner = LEAGUE_CLIENT
        .get_summoner_by_name("Santorin")
        .expect("Something went wrong");
    let masteries: Vec<ChampionMastery> = LEAGUE_CLIENT
        .get_champion_masteries(&summoner.id)
        .expect("Could not get champion masteries");
    assert_ne!(masteries.len(), 0);
}

#[test]
fn gets_champion_mastery_by_id() {
    let mut ddragon_client = DDRAGON_CLIENT.lock().unwrap();
    let ahri = ddragon_client.get_champion("LeeSin").unwrap();
    let summoner: Summoner = LEAGUE_CLIENT
        .get_summoner_by_name("Santorin")
        .expect("Something went wrong");
    let mastery: ChampionMastery = LEAGUE_CLIENT
        .get_champion_mastery_by_id(
            &summoner.id,
            ahri["data"]["LeeSin"]["key"]
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),
        )
        .expect(&format!(
            "Could not get champion mastery for champion id {}",
            ahri["data"]["LeeSin"]["key"].as_str().unwrap()
        ));

    assert_eq!(mastery.champion_id, 64);
    assert_eq!(mastery.champion_level, 7);
    assert!(mastery.champion_points >= 93748)
}

#[test]
fn gets_total_mastery_score() {
    let summoner: Summoner = LEAGUE_CLIENT
        .get_summoner_by_name("Santorin")
        .expect("Something went wrong");
    let score = LEAGUE_CLIENT
        .get_total_mastery_score(&summoner.id)
        .expect("Could not get total mastery score");
    assert!(score >= 192)
}
