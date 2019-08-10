use crate::constants::Region;
use crate::dto::api::{ChampionInfo, ChampionMastery, Summoner};
use log::{debug, trace};
use reqwest::{StatusCode, ClientBuilder};
use reqwest::{Client, Method, Url};
use serde::de::DeserializeOwned;
use std::env;
use std::fmt;
use reqwest::header::{self, HeaderMap, HeaderValue};
use snafu::ResultExt;
use crate::error::{ApiError, Other};

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
        let client = client_builder
            .default_headers(headers)
            .build()
            .unwrap();
        Ok(LeagueAPI {
            client,
            api_key,
            region,
            base_url,
        })
    }

    pub fn get_summoner_by_name(&self, name: &str) -> Result<Summoner, ApiError> {
        trace!("Getting summoner with name: {}", &name);
        let url: Url = format!("{}/summoner/v4/summoners/by-name/{}", self.base_url, name)
            .parse()
            .unwrap();
        debug!("Constructed url: {:?}", &url);
        Ok(self.get_and_deserialize(url)?)
    }

    pub fn get_champion_info(&self) -> Result<ChampionInfo, ApiError> {
        let url: Url = format!("{}/platform/v3/champion-rotations", self.base_url)
            .parse()
            .unwrap();
        Ok(self.get_and_deserialize(url)?)
    }

    pub fn get_champion_masteries(
        &self,
        summoner_id: &str,
    ) -> Result<Vec<ChampionMastery>, ApiError> {
        trace!("Getting champion masteries for id: {}", &summoner_id);
        let url: Url = format!(
            "{}/champion-mastery/v4/champion-masteries/by-summoner/{}",
            self.base_url, summoner_id
        )
        .parse()
        .unwrap();
        Ok(self.get_and_deserialize(url)?)
    }

    pub fn get_champion_mastery_by_id(
        &self,
        summoner_id: &str,
        champion_id: i64,
    ) -> Result<ChampionMastery, ApiError> {
        let url: Url = format!(
            "{}/champion-mastery/v4/champion-masteries/by-summoner/{}/by-champion/{}",
            self.base_url, summoner_id, champion_id
        )
        .parse()
        .unwrap();
        Ok(self.get_and_deserialize(url)?)
    }

    pub fn get_total_mastery_score(&self, summoner_id: &str) -> Result<i32, ApiError> {
        let url: Url = format!(
            "{}/champion-mastery/v4/scores/by-summoner/{}",
            self.base_url, summoner_id
        )
        .parse()
        .unwrap();
        Ok(self.get_and_deserialize(url)?)
    }

    pub fn get_and_deserialize<T: DeserializeOwned>(&self, url: Url) -> Result<T, ApiError> {
        let mut resp = self
            .client
            .get(url)
            .send().context(Other {})?;
        if !resp.status().is_success() {
            if resp.status().as_u16() == 503 {
                return Err(self.region.clone().into())
            }
            return Err(resp.into())
        }
        let deserialized: T = resp.json().context(Other {})?;
        Ok(deserialized)
    }
}

impl Default for LeagueAPI {
    fn default() -> LeagueAPI {
        LeagueAPI::new(Region::default()).expect("Please provide API_KEY environment variable")
    }
}

