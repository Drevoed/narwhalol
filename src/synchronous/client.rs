use crate::constants::Region;
use crate::dto::api::{ChampionInfo, ChampionMastery, Summoner};
use failure::Error;
use log::{debug, trace};
use reqwest::{Client, Method, Url};
use std::fmt;
use serde::de::DeserializeOwned;
use reqwest::StatusCode;

#[derive(Debug, Fail)]
pub struct LeagueError(u16);

impl fmt::Display for LeagueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.0 {
            400 => write!(f, "Bad Request"),
            401 => write!(f, "Unauthorized"),
            403 => write!(f, "Forbidden"),
            404 => write!(f, "Data not found"),
            405 => write!(f, "Method not allowed"),
            415 => write!(f, "Unsupported media type"),
            429 => write!(f, "Rate limit exceeded"),
            500 => write!(f, "Internal server error"),
            502 => write!(f, "Bad gateway"),
            503 => write!(f, "Service unavailable"),
            504 => write!(f, "Gateway timeout"),
            _ => unreachable!(),
        }
    }
}

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
    pub fn new(region: Region) -> Result<LeagueAPI, Error> {
        let client = Client::new();
        let base_url = format!("https://{}.api.riotgames.com/lol", region.as_platform_str());
        let api_key = std::env::var("RIOT_API_KEY")?;
        Ok(LeagueAPI {
            client,
            api_key,
            region,
            base_url,
        })
    }

    pub fn get_summoner_by_name(&self, name: &str) -> Result<Summoner, Error> {
        trace!("Getting summoner with name: {}", &name);
        let url: Url = format!("{}/summoner/v4/summoners/by-name/{}", self.base_url, name)
            .parse()
            .unwrap();
        debug!("Constructed url: {:?}", &url);
        Ok(self.get_and_deserialize(url)?)
    }

    pub fn get_champion_info(&self) -> Result<ChampionInfo, Error> {
        let url: Url = format!("{}/platform/v3/champion-rotations", self.base_url)
            .parse()
            .unwrap();
        Ok(self.get_and_deserialize(url)?)
    }

    pub fn get_champion_masteries(&self, summoner_id: &str) -> Result<Vec<ChampionMastery>, Error> {
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
    ) -> Result<ChampionMastery, Error> {
        let url: Url = format!(
            "{}/champion-mastery/v4/champion-masteries/by-summoner/{}/by-champion/{}",
            self.base_url, summoner_id, champion_id
        )
        .parse()?;
        Ok(self.get_and_deserialize(url)?)
    }

    pub fn get_total_mastery_score(&self, summoner_id: &str) -> Result<i32, Error> {
        let url: Url = format!(
            "{}/champion-mastery/v4/scores/by-summoner/{}",
            self.base_url, summoner_id
        )
        .parse()
        .unwrap();

        Ok(self.get_and_deserialize(url)?)
    }

    pub fn get_and_deserialize<T: DeserializeOwned>(&self, url: Url) -> Result<T, Error> {
        let mut resp = self
            .client
            .get(url)
            .header("X-Riot-Token", &self.api_key)
            .send()?;
        let status = resp.status().as_u16();
        if status != 200 {
            return Err(LeagueError(status).into())
        }
        let deserialized: T = resp.json()?;
        Ok(deserialized)
    }
}

impl Default for LeagueAPI {
    fn default() -> LeagueAPI {
        LeagueAPI::new(Region::default()).expect("Please provide API_KEY environment variable")
    }
}
