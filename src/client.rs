use crate::constants::{LanguageCode, Region};
use crate::ddragon::DDragonClient;
use crate::dto::api::{ChampionInfo, ChampionMastery, Summoner};
use crate::error::*;
use crate::types::{Client, Cache};
use crate::utils::{construct_hyper_client, CacheFutureSpawner};
use futures::future::{ok, Either};
use futures::{Future, Stream};
use hyper::client::connect::dns::GaiResolver;
use hyper::Request;
use hyper::{
    client::HttpConnector, header::HeaderValue, Body, Client as HttpClient, HeaderMap, Uri,
};
use hyper_tls::HttpsConnector;
use log::{debug, trace};
use serde::de::DeserializeOwned;
use snafu::ResultExt;
use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
use std::str;
use std::sync::{Arc, Mutex};

/// Main type for calling League API Endpoints.
#[derive(Debug)]
pub struct LeagueClient
{
    spawner: CacheFutureSpawner,
    region: Region,
    base_url: String,
    pub ddragon: Option<DDragonClient>,
}

impl LeagueClient
{
    /// Constructor function for LeagueAPI struct, accepts Region type as a parameter
    pub fn new(region: Region) -> Result<LeagueClient, env::VarError> {
        let mut headers = HeaderMap::new();
        let base_url = format!("http://{}.api.riotgames.com/lol", region.as_platform_str());
        let api_key = std::env::var("RIOT_API_KEY")?;
        let client = construct_hyper_client();
        let cache: Cache = Arc::new(Mutex::new(HashMap::new()));
        let spawner = CacheFutureSpawner::new(client, cache, Some(api_key));
        Ok(LeagueClient {
            region,
            base_url,
            ddragon: None,
            spawner
        })
    }

    pub fn ddragon(self, language: LanguageCode) -> Result<Self, ClientError> {
        let ddragon = DDragonClient::new(language)?;
        Ok(LeagueClient {
            ddragon: Some(ddragon),
            ..self
        })
    }

    pub fn get_summoner_by_name(
        &mut self,
        name: &str,
    ) -> impl Future<Item = Summoner, Error = ClientError> {
        println!("Getting summoner with name: {}", &name);
        let url: Uri = format!("{}/summoner/v4/summoners/by-name/{}", self.base_url, name)
            .parse()
            .unwrap();
        debug!("Constructed url: {:?}", &url);
        self.spawner.spawn_cache_fut(url)
    }

    pub fn get_champion_info(&mut self) -> impl Future<Item = ChampionInfo, Error = ClientError> {
        let url: Uri = format!("{}/platform/v3/champion-rotations", self.base_url)
            .parse()
            .unwrap();
        self.spawner.spawn_cache_fut(url)
    }

    pub fn get_champion_masteries(
        &mut self,
        summoner_id: &str,
    ) -> impl Future<Item = Vec<ChampionMastery>, Error = ClientError> {
        trace!("Getting champion masteries for id: {}", &summoner_id);
        let url: Uri = format!(
            "{}/champion-mastery/v4/champion-masteries/by-summoner/{}",
            self.base_url, summoner_id
        )
        .parse()
        .unwrap();
        self.spawner.spawn_cache_fut(url)
    }

    pub fn get_champion_mastery_by_id(
        &mut self,
        summoner_id: &str,
        champion_id: u64,
    ) -> impl Future<Item = ChampionMastery, Error = ClientError> {
        let url: Uri = format!(
            "{}/champion-mastery/v4/champion-masteries/by-summoner/{}/by-champion/{}",
            self.base_url, summoner_id, champion_id
        )
        .parse()
        .unwrap();
        self.spawner.spawn_cache_fut(url)
    }

    pub fn get_total_mastery_score(
        &mut self,
        summoner_id: &str,
    ) -> impl Future<Item = i32, Error = ClientError> {
        let url: Uri = format!(
            "{}/champion-mastery/v4/scores/by-summoner/{}",
            self.base_url, summoner_id
        )
        .parse()
        .unwrap();
        self.spawner.spawn_cache_fut(url)
    }

    #[cfg(test)]
    pub(crate) fn get_status(&self, status: u16) -> Result<(), ClientError> {
        ClientError::check_status(self.region.clone(), status)
    }
}

impl Default for LeagueClient
{
    fn default() -> LeagueClient {
        LeagueClient::new(Region::default()).expect("Please provide API_KEY environment variable")
    }
}

#[cfg(test)]
mod tests {
    use crate::dto::api::{ChampionMastery, Summoner};
    use crate::dto::ddragon::ChampionFullData;
    use crate::error::ClientError;
    use crate::client::LeagueClient;
    use crate::constants::Region;
    use futures::Future;
    use futures::future::lazy;
    use env_logger;

    #[test]
    fn gets_summoner_data() {
        env_logger::init();
        tokio::run(lazy(|| {
            LeagueClient::new(Region::NA)
                .unwrap()
                .get_summoner_by_name("Santorin")
                .map(|sum| {
                    println!("{:?}", &sum);
                    assert_eq!(
                        &sum.account_id,
                        "rPnj4h5W6OhejxB-AO3hLOQctgZcckqV_82N_8_WuCFdO2A"
                    )
                })
                .map_err(|cli_err| println!("{}", cli_err))
        })
        )
    }

    /*#[test]
    fn gets_champion_info() -> Result<(), ClientError> {
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
            .get_champion_mastery_by_id(&summoner.id, lee_sin.key.parse().unwrap())
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
    }*/
}
