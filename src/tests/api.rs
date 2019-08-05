use crate::constants::region::Region;
use crate::dto::api::{ChampionInfo, ChampionMastery, Summoner};
use crate::http::synchronous::client::LeagueAPI;

lazy_static! {
    static ref LEAGUE_CLIENT: LeagueAPI = LeagueAPI::new(Region::NA);
}

#[test]
fn gets_summoner_data() {
    let summoner = LEAGUE_CLIENT
        .get_summoner_by_name("Santorin")
        .expect("Something went wrong");
    println!("Got summoner: {:#?}", &summoner);
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
    println!("Got champion info: {:#?}", &champ_info);
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
    let summoner: Summoner = LEAGUE_CLIENT
        .get_summoner_by_name("Santorin")
        .expect("Something went wrong");
    let mastery: ChampionMastery = LEAGUE_CLIENT
        .get_champion_mastery_by_id(&summoner.id, 64)
        .expect(&format!(
            "Could not get champion mastery for champion id {}",
            64
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
