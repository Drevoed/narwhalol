use crate::constants::{LanguageCode, Region};
use crate::dto::api::{ChampionInfo, ChampionMastery, Summoner};
use crate::synchronous::client::{LeagueAPI};
use crate::synchronous::ddragon::DDragonClient;
use std::sync::Mutex;
use crate::error::ApiError;

lazy_static! {
    static ref DDRAGON_CLIENT: Mutex<DDragonClient> =
        Mutex::new(DDragonClient::new(LanguageCode::RUSSIA).unwrap());
    static ref LEAGUE_CLIENT: LeagueAPI =
        LeagueAPI::new(Region::NA).expect("Please provide API_KEY environment variable");
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

fn gets_champion_info() -> Result<(), ApiError> {
    let champ_info = LEAGUE_CLIENT.get_champion_info()?;
    assert!(champ_info.free_champion_ids.len() > 10);
    assert!(champ_info.free_champion_ids_for_new_players.len() > 0);
    Ok(assert_ne!(champ_info.max_new_player_level, 0))
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

#[cfg(test)]
mod api_error_tests {
    use crate::error::*;
    use super::LEAGUE_CLIENT;
    use crate::constants::Region;

    macro_rules! assert_matches {
    ($expression:expr, $($pattern:tt)+) => {
        match $expression {
            $($pattern)+ => (),
            ref e => panic!("Asserion failed: `{:?}` does not match `{}`", e, stringify!($($pattern)+))
        }
    };
}

    #[test]
    fn returns_correct_status_codes() {
        let bad_r_err = LEAGUE_CLIENT.get_status(400).unwrap_err();
        let unauthorized_err = LEAGUE_CLIENT.get_status(401).unwrap_err();
        let forbidden_err = LEAGUE_CLIENT.get_status(403).unwrap_err();
        let not_found_err = LEAGUE_CLIENT.get_status(404).unwrap_err();
        let method_not_allowed_err = LEAGUE_CLIENT.get_status(405).unwrap_err();
        let unsupported_media_err = LEAGUE_CLIENT.get_status(415).unwrap_err();
        let rate_err = LEAGUE_CLIENT.get_status(429).unwrap_err();
        let internal_err = LEAGUE_CLIENT.get_status(500).unwrap_err();
        let bad_g_err = LEAGUE_CLIENT.get_status(502).unwrap_err();
        let service_err = LEAGUE_CLIENT.get_status(503).unwrap_err();
        let gateway_t_err = LEAGUE_CLIENT.get_status(504).unwrap_err();
        assert_matches!(bad_r_err, ApiError::BadRequest);
        assert_matches!(unauthorized_err, ApiError::Unauthorized);
        assert_matches!(forbidden_err, ApiError::Forbidden);
        assert_matches!(not_found_err, ApiError::DataNotFound);
        assert_matches!(method_not_allowed_err, ApiError::MethodNotAllowed);
        assert_matches!(unsupported_media_err, ApiError::UnsupportedMediaType);
        assert_matches!(rate_err, ApiError::RateLimitExceeded {limit: 0});
        assert_matches!(internal_err, ApiError::InternalServerError);
        assert_matches!(bad_g_err, ApiError::BadGateway);
        assert_matches!(service_err, ApiError::ServiceUnavailable {region: Region::NA});
        assert_matches!(gateway_t_err, ApiError::GatewayTimeout)
    }
}
