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
use crate::error::{*};

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

    pub fn get_summoner_by_name(&self, name: &str) -> ApiResult<Summoner> {
        trace!("Getting summoner with name: {}", &name);
        let url: Url = format!("{}/summoner/v4/summoners/by-name/{}", self.base_url, name)
            .parse()
            .unwrap();
        debug!("Constructed url: {:?}", &url);
        Ok(self.get_and_deserialize(url)?)
    }

    pub fn get_champion_info(&self) -> ApiResult<ChampionInfo> {
        let url: Url = format!("{}/platform/v3/champion-rotations", self.base_url)
            .parse()
            .unwrap();
        Ok(self.get_and_deserialize(url)?)
    }

    pub fn get_champion_masteries(
        &self,
        summoner_id: &str,
    ) -> ApiResult<Vec<ChampionMastery>> {
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
    ) -> ApiResult<ChampionMastery> {
        let url: Url = format!(
            "{}/champion-mastery/v4/champion-masteries/by-summoner/{}/by-champion/{}",
            self.base_url, summoner_id, champion_id
        )
        .parse()
        .unwrap();
        Ok(self.get_and_deserialize(url)?)
    }

    pub fn get_total_mastery_score(&self, summoner_id: &str) -> ApiResult<i32> {
        let url: Url = format!(
            "{}/champion-mastery/v4/scores/by-summoner/{}",
            self.base_url, summoner_id
        )
        .parse()
        .unwrap();
        Ok(self.get_and_deserialize(url)?)
    }

    fn get_and_deserialize<T: DeserializeOwned>(&self, url: Url) -> ApiResult<T> {
        let mut resp = self
            .client
            .get(url)
            .send().context(Other {})?;
        self.check_status(&resp.status())?;
        let deserialized: T = resp.json().context(Other {})?;
        Ok(deserialized)
    }

    fn check_status (&self, code: &StatusCode) -> ApiResult<()> {
        match code.as_u16() {
            400 => BadRequest.fail(),
            401 => Unauthorized.fail(),
            403 => Forbidden.fail(),
            404 => DataNotFound.fail(),
            405 => MethodNotAllowed.fail(),
            415 => UnsupportedMediaType.fail(),
            429 => RateLimitExceeded {limit: 0_usize}.fail(),
            500 => InternalServerError.fail(),
            502 => BadGateway.fail(),
            503 => ServiceUnavailable {region: self.region.clone()}.fail(),
            504 => GatewayTimeout.fail(),
            _ => Ok(())
        }
    }

    #[cfg(test)]
    pub(crate) fn get_status(&self, status: i32) -> ApiResult<()> {
        let url: Url = format!("https://httpstat.us/{}", status).parse().unwrap();
        let mut resp = self
            .client
            .get(url)
            .send().context(Other {})?;
        self.check_status(&resp.status())
    }
}

impl Default for LeagueAPI {
    fn default() -> LeagueAPI {
        LeagueAPI::new(Region::default()).expect("Please provide API_KEY environment variable")
    }
}

