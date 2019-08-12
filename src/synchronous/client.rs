use crate::constants::Region;
use crate::dto::api::{ChampionInfo, ChampionMastery, Summoner};
use crate::error::*;
use log::{debug, trace};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Url};
use reqwest::{ClientBuilder, StatusCode};
use serde::de::DeserializeOwned;
use snafu::ResultExt;
use std::env;

/// Main type for calling League API Endpoints.
#[derive(Debug)]
pub struct LeagueAPI {
    client: Client,
    api_key: String,
    region: Region,
    base_url: String,
}

impl LeagueAPI {
    /// Constructor function for LeagueAPI struct, accepts Region type as a parameter
    pub fn new(region: Region) -> Result<LeagueAPI, env::VarError> {
        let mut headers = HeaderMap::new();
        let client_builder = ClientBuilder::new();
        let base_url = format!("https://{}.api.riotgames.com/lol", region.as_platform_str());
        let api_key = std::env::var("RIOT_API_KEY")?;
        headers.insert("X-Riot-Token", HeaderValue::from_str(&api_key).unwrap());
        let client = client_builder.default_headers(headers).build().unwrap();
        Ok(LeagueAPI {
            client,
            api_key,
            region,
            base_url,
        })
    }

    pub fn get_summoner_by_name(&self, name: &str) -> ApiResult<Summoner> {
        trace!("Getting summoner with name: {}", &name);
        let url: Url = format!("{}/summoner/v4/summoners/by-name/{}", self.base_url, name)
            .parse().context(ReqwestUrlNotParsed {})?;
        debug!("Constructed url: {:?}", &url);
        Ok(self.get_and_deserialize(url)?)
    }

    pub fn get_champion_info(&self) -> ApiResult<ChampionInfo> {
        let url: Url = format!("{}/platform/v3/champion-rotations", self.base_url)
            .parse().context(ReqwestUrlNotParsed {})?;
        Ok(self.get_and_deserialize(url)?)
    }

    pub fn get_champion_masteries(&self, summoner_id: &str) -> ApiResult<Vec<ChampionMastery>> {
        trace!("Getting champion masteries for id: {}", &summoner_id);
        let url: Url = format!(
            "{}/champion-mastery/v4/champion-masteries/by-summoner/{}",
            self.base_url, summoner_id
        )
        .parse().context(ReqwestUrlNotParsed {})?;
        Ok(self.get_and_deserialize(url)?)
    }

    pub fn get_champion_mastery_by_id(
        &self,
        summoner_id: &str,
        champion_id: u64,
    ) -> ApiResult<ChampionMastery> {
        let url: Url = format!(
            "{}/champion-mastery/v4/champion-masteries/by-summoner/{}/by-champion/{}",
            self.base_url, summoner_id, champion_id
        )
        .parse()
            .context(ReqwestUrlNotParsed {})?;
        Ok(self.get_and_deserialize(url)?)
    }

    pub fn get_total_mastery_score(&self, summoner_id: &str) -> ApiResult<i32> {
        let url: Url = format!(
            "{}/champion-mastery/v4/scores/by-summoner/{}",
            self.base_url, summoner_id
        )
        .parse().context(ReqwestUrlNotParsed {})?;
        Ok(self.get_and_deserialize(url)?)
    }

    fn get_and_deserialize<T: DeserializeOwned>(&self, url: Url) -> ApiResult<T> {
        let mut resp = self.client.get(url).send().context(ReqwestError {})?;
        ApiError::check_status(self.region.clone(), resp.status().as_u16())?;
        let deserialized: T = resp.json().context(ReqwestError {})?;
        Ok(deserialized)
    }

    #[cfg(test)]
    pub(crate) fn get_status(&self, status: u16) -> ApiResult<()> {
        ApiError::check_status(self.region.clone(), status)
    }
}

impl Default for LeagueAPI {
    fn default() -> LeagueAPI {
        LeagueAPI::new(Region::default()).expect("Please provide API_KEY environment variable")
    }
}

#[cfg(test)]
mod tests {
    use crate::dto::api::{ChampionMastery, Summoner};
    use crate::error::ApiError;
    use crate::{DDRAGON_CLIENT, LEAGUE_CLIENT};
    use crate::dto::ddragon::ChampionFullData;

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
        let lee_sin: ChampionFullData = ddragon_client.get_champion("LeeSin").unwrap();
        let summoner: Summoner = LEAGUE_CLIENT
            .get_summoner_by_name("Santorin")
            .expect("Something went wrong");
        let mastery: ChampionMastery = LEAGUE_CLIENT
            .get_champion_mastery_by_id(
                &summoner.id,
                lee_sin.key.parse().unwrap(),
            )
            .expect(&format!(
                "Could not get champion mastery for champion id {}",
                lee_sin.key
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
}
