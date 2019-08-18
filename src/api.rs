use crate::constants::{LanguageCode, RankedQueue, RankedTier, Region};
use crate::ddragon::DDragonClient;
use crate::dto::api::{ChampionInfo, ChampionMastery, LeagueInfo, Summoner};
use crate::error::*;
use crate::types::{Cache, Client};
use crate::utils::{cached_resp, construct_hyper_client};
use futures::{Future, FutureExt, TryFutureExt};

use hyper::{HeaderMap, Uri};

use log::{debug, trace};

use std::collections::HashMap;
use std::env;

use crate::constants::division::Division;
use std::str;
use std::sync::{Arc, Mutex};

/// Main type for calling League API Endpoints.
#[derive(Debug)]
pub struct LeagueClient {
    client: Client,
    cache: Cache,
    region: Region,
    base_url: String,
    pub ddragon: Option<DDragonClient>,
    api_key: String,
}

impl LeagueClient {
    /// Constructor function for LeagueAPI struct, accepts Region type as a parameter
    pub fn new(region: Region) -> Result<LeagueClient, env::VarError> {
        let _headers = HeaderMap::new();
        let base_url = format!("https://{}.api.riotgames.com/lol", region.as_platform_str());
        let api_key = std::env::var("RIOT_API_KEY")?;
        let client = construct_hyper_client();
        let cache: Cache = Arc::new(Mutex::new(HashMap::new()));
        Ok(LeagueClient {
            region,
            base_url,
            ddragon: None,
            cache,
            client,
            api_key,
        })
    }

    pub fn with_ddragon(self, language: LanguageCode) -> Result<Self, ClientError> {
        let ddragon =
            DDragonClient::new_for_lapi(self.client.clone(), self.cache.clone(), language)?;
        Ok(LeagueClient {
            ddragon: Some(ddragon),
            ..self
        })
    }

    pub fn ddragon(&mut self) -> &mut DDragonClient {
        match self.ddragon {
            Some(ref mut dd) => dd,
            None => panic!(
                "You are trying to access ddragon client without creating an embedded one in LeagueClient!\n\
                See with_ddragon method in League Client.\
                ")
        }
    }

    pub fn get_summoner_by_name(
        &mut self,
        name: &str,
    ) -> impl Future<Output = Result<Summoner, ClientError>> {
        println!("Getting summoner with name: {}", &name);
        let url: Uri = format!("{}/summoner/v4/summoners/by-name/{}", self.base_url, name)
            .parse()
            .unwrap();
        debug!("Constructed url: {:?}", &url);
        cached_resp(
            self.client.clone(),
            self.cache.clone(),
            url,
            Some(&self.api_key),
        )
    }

    pub fn get_champion_info(&mut self) -> impl Future<Output = Result<ChampionInfo, ClientError>> {
        let url: Uri = format!("{}/platform/v3/champion-rotations", self.base_url)
            .parse()
            .unwrap();
        cached_resp(
            self.client.clone(),
            self.cache.clone(),
            url,
            Some(&self.api_key),
        )
    }

    pub fn get_champion_masteries(
        &mut self,
        summoner_id: &str,
    ) -> impl Future<Output = Result<Vec<ChampionMastery>, ClientError>> {
        trace!("Getting champion masteries for id: {}", &summoner_id);
        let url: Uri = format!(
            "{}/champion-mastery/v4/champion-masteries/by-summoner/{}",
            self.base_url, summoner_id
        )
        .parse()
        .unwrap();
        cached_resp(
            self.client.clone(),
            self.cache.clone(),
            url,
            Some(&self.api_key),
        )
    }

    pub fn get_champion_mastery_by_id(
        &mut self,
        summoner_id: &str,
        champion_id: u64,
    ) -> impl Future<Output = Result<ChampionMastery, ClientError>> {
        let url: Uri = format!(
            "{}/champion-mastery/v4/champion-masteries/by-summoner/{}/by-champion/{}",
            self.base_url, summoner_id, champion_id
        )
        .parse()
        .unwrap();
        cached_resp(
            self.client.clone(),
            self.cache.clone(),
            url,
            Some(&self.api_key),
        )
    }

    pub fn get_total_mastery_score(
        &mut self,
        summoner_id: &str,
    ) -> impl Future<Output = Result<i32, ClientError>> {
        let url: Uri = format!(
            "{}/champion-mastery/v4/scores/by-summoner/{}",
            self.base_url, summoner_id
        )
        .parse()
        .unwrap();
        cached_resp(
            self.client.clone(),
            self.cache.clone(),
            url,
            Some(&self.api_key),
        )
    }

    pub fn get_league_exp_entries(
        &mut self,
        queue: RankedQueue,
        tier: RankedTier,
        division: Division,
        pages: Option<i32>,
    ) -> impl Future<Output = Result<Vec<LeagueInfo>, ClientError>> {
        let url: Uri = match pages {
            Some(p) => format!(
                "{}/league-exp/v4/entries/{}/{}/{}?page={}",
                &self.base_url, queue, tier, division, p
            )
            .parse()
            .unwrap(),
            None => format!(
                "{}/league-exp/v4/entries/{}/{}/{}",
                &self.base_url, queue, tier, division
            )
            .parse()
            .unwrap(),
        };

        cached_resp(
            self.client.clone(),
            self.cache.clone(),
            url,
            Some(&self.api_key),
        )
    }

    #[cfg(test)]
    pub(crate) fn get_status(&self, status: u16) -> impl Future<Output = Result<(), ClientError>> {
        ClientError::check_status(self.region.clone(), status)
    }
}

impl Default for LeagueClient {
    fn default() -> LeagueClient {
        LeagueClient::new(Region::default()).expect("Please provide API_KEY environment variable")
    }
}

#[cfg(test)]
mod tests {
    use super::LeagueClient;
    use crate::constants::{LanguageCode, RankedQueue, RankedTier, Region};

    use env_logger;
    use futures::prelude::*;
    use futures::{Future, FutureExt, TryFutureExt};

    use crate::constants::division::Division;
    use crate::dto::api::{ChampionInfo, ChampionMastery, Summoner};
    use crate::dto::ddragon::ChampionFullData;
    use crate::error::ClientError;
    use crate::types::Cache;
    use log::debug;
    use std::time::Instant;

    #[cfg(test)]
    fn print_cache(cache: Cache) {
        debug!("{:?}", cache.lock().unwrap().keys().collect::<Vec<_>>())
    }

    #[test]
    fn gets_summoner_data() {
        let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();
        env_logger::init();
        let sum: Summoner = runtime
            .block_on(
                LeagueClient::new(Region::NA)
                    .unwrap()
                    .get_summoner_by_name("Santorin"),
            )
            .unwrap();
        assert_eq!(
            &sum.account_id,
            "rPnj4h5W6OhejxB-AO3hLOQctgZcckqV_82N_8_WuCFdO2A"
        )
    }

    #[test]
    fn lapi_caches_properly() {
        let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();
        let mut cli = LeagueClient::new(Region::RU).unwrap();
        let cache = cli.cache.clone();
        let _ = runtime.block_on(cli.get_summoner_by_name("Vetro")).unwrap();
        let now = Instant::now();
        let _ = runtime.block_on(cli.get_summoner_by_name("Vetro")).unwrap();
        assert!(now.elapsed().as_millis() <= 2);
        print_cache(cache);
    }

    #[test]
    fn gets_champion_info() {
        let mut lapi = LeagueClient::new(Region::default()).unwrap();
        let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();
        let champ_info = runtime.block_on(lapi.get_champion_info()).unwrap();
        assert!(champ_info.free_champion_ids.len() > 10);
        assert!(champ_info.free_champion_ids_for_new_players.len() > 0);
        assert_ne!(champ_info.max_new_player_level, 0)
    }

    #[test]
    fn gets_champion_masteries() {
        let mut lapi = LeagueClient::new(Region::NA).unwrap();
        let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();
        let masteries: Vec<ChampionMastery> = runtime
            .block_on(
                lapi.get_summoner_by_name("Santorin")
                    .and_then(move |summoner| lapi.get_champion_masteries(&summoner.id)),
            )
            .unwrap();
        assert_ne!(masteries.len(), 0)
    }

    #[test]
    fn gets_champion_mastery_by_id() {
        let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();
        let mut lapi = LeagueClient::new(Region::default())
            .unwrap()
            .with_ddragon(LanguageCode::UNITED_STATES)
            .unwrap();
        let mut ddragon_client = lapi.ddragon();
        let lee_sin: ChampionFullData = runtime
            .block_on(ddragon_client.get_champion("LeeSin"))
            .unwrap();
        let summoner: Summoner = runtime
            .block_on(lapi.get_summoner_by_name("Santorin"))
            .unwrap();
        let mastery: ChampionMastery = runtime
            .block_on(lapi.get_champion_mastery_by_id(&summoner.id, lee_sin.key.parse().unwrap()))
            .unwrap();

        assert_eq!(mastery.champion_id, 64);
        assert_eq!(mastery.champion_level, 7);
        assert!(mastery.champion_points >= 93748)
    }

    #[test]
    fn gets_total_mastery_score() {
        let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();
        let mut lapi = LeagueClient::new(Region::default()).unwrap();
        let summoner: Summoner = runtime
            .block_on(lapi.get_summoner_by_name("Santorin"))
            .unwrap();
        let score = runtime
            .block_on(lapi.get_total_mastery_score(&summoner.id))
            .unwrap();
        assert!(score >= 192)
    }

    #[test]
    fn gets_league_exp() -> Result<(), ClientError> {
        let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();
        let mut lapi = LeagueClient::new(Region::default()).unwrap();
        let challengers = runtime
            .block_on(lapi.get_league_exp_entries(
                RankedQueue::SOLO,
                RankedTier::CHALLENGER,
                Division::I,
                None,
            ))
            .unwrap();
        assert!(challengers.len() > 0);
        Ok(())
    }
}
