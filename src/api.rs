//! Easy to use async riot api wrapper.
//!
//! # Introduction
//! This module contains all the things needed to talk with Riot API.
//! The most important type here is
//! [`LeagueClient`], as it is the main way of getting the data from API. See [`LeagueClient`] for more information.
use crate::constants::{LanguageCode, RankedQueue, RankedTier, Region};
use crate::ddragon::DDragonClient;
use crate::dto::api::{ChampionInfo, ChampionMastery, LeagueInfo, Summoner};
use crate::error::*;
use crate::types::{Cache, Client};
use crate::utils::{construct_hyper_client, CachedClient};
use futures::prelude::*;

use hyper::{Body, HeaderMap, Request, Uri};
use snafu::{ensure, ResultExt};

use log::{debug, trace};

use std::collections::HashMap;
use std::env;

use crate::constants::division::Division;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::str;
use std::sync::Arc;

use async_trait::async_trait;
use hyper::header::HeaderValue;
use parking_lot::Mutex;

/// Main type for calling League API Endpoints.
/// Instances of `LeagueClient` can be created using [`new`] with a [`Region`] parameter
///
/// `LeagueClient` can have an embedded [`DDragonClient`] instance embedded in itself,
/// reference to which can be obtained using [`ddragon`]. ***NOTE***: this method will panic if
/// you don't create the instance using [`with_ddragon`].
///
/// [`new`]: #method.new
/// [`Region`]: ../constants/region/struct.Region.html
/// [`DDragonClient`]: ../ddragon/struct.DDragonClient.html
/// [`ddragon`]: #method.ddragon
/// [`with_ddragon`]: #method.with_ddragon
#[derive(Debug)]
pub struct LeagueClient {
    client: Client,
    cache: Cache,
    region: Region,
    base_url: String,
    ddragon: Option<DDragonClient>,
    api_key: String,
}

impl LeagueClient {
    /// Constructor function for LeagueAPI struct, accepts type as a parameter
    ///
    /// # Panics
    /// This will panic if you do not provide the RIOT_API_KEY environment variable with value being api token.
    pub fn new(region: Region) -> Result<LeagueClient, ClientError> {
        let base_url = format!("https://{}.api.riotgames.com/lol", region.as_platform_str());
        let api_key = std::env::var("RIOT_API_KEY").context(NoToken {})?;
        check_token(&api_key)?;
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

    /// Adds an embedded ddragon client instance to league api client that shares cache and client with parent.
    pub async fn with_ddragon(self, language: LanguageCode) -> Self {
        let ddragon =
            DDragonClient::new_for_lapi(self.client.clone(), self.cache.clone(), language)
                .await
                .unwrap();
        LeagueClient {
            ddragon: Some(ddragon),
            ..self
        }
    }

    /// Gets mutable (because of cache) reference to ddragon client embedded in lapi client.
    ///
    /// # Panics
    /// Do not call `ddragon` if [`with_ddragon`] is not called beforehand.
    ///
    /// [`with_ddragon`]: #method.with_ddragon
    pub fn ddragon(&mut self) -> &mut DDragonClient {
        match self.ddragon {
            Some(ref mut dd) => dd,
            None => panic!(
                "You are trying to access ddragon client without creating an embedded one in LeagueClient!\n\
                See with_ddragon method in League Client.\
                ")
        }
    }

    ///Get summoner by plaintext name
    /// # Example
    /// ```
    /// use narwhalol::{LeagueClient, Region, dto::api::Summoner, error::ClientError};
    ///
    /// fn main() -> Result<(), ClientError> {
    ///     smol::run(async {
    ///          let mut lapi = LeagueClient::new(Region::RU).unwrap();
    ///          let summoner = lapi.get_summoner_by_name("Vetro").await?;
    ///          assert_eq!(summoner.name, "Vetro");
    ///          Ok(())
    ///     })
    /// }
    ///
    /// ```
    ///
    pub async fn get_summoner_by_name(&self, name: &str) -> Result<Summoner, ClientError> {
        println!("Getting summoner with name: {}", &name);
        let url: Uri = format!("{}/summoner/v4/summoners/by-name/{}", self.base_url, name)
            .parse()
            .unwrap();
        debug!("Constructed url: {:?}", &url);
        self.cached_resp(url).await
    }

    pub async fn get_champion_info(&mut self) -> Result<ChampionInfo, ClientError> {
        let url: Uri = format!("{}/platform/v3/champion-rotations", self.base_url)
            .parse()
            .unwrap();
        self.cached_resp(url).await
    }

    pub async fn get_champion_masteries(
        &mut self,
        summoner_id: &str,
    ) -> Result<Vec<ChampionMastery>, ClientError> {
        trace!("Getting champion masteries for id: {}", &summoner_id);
        let url: Uri = format!(
            "{}/champion-mastery/v4/champion-masteries/by-summoner/{}",
            self.base_url, summoner_id
        )
        .parse()
        .unwrap();
        self.cached_resp(url).await
    }

    pub async fn get_champion_mastery_by_id(
        &mut self,
        summoner_id: &str,
        champion_id: u64,
    ) -> Result<ChampionMastery, ClientError> {
        let url: Uri = format!(
            "{}/champion-mastery/v4/champion-masteries/by-summoner/{}/by-champion/{}",
            self.base_url, summoner_id, champion_id
        )
        .parse()
        .unwrap();
        self.cached_resp(url).await
    }

    pub async fn get_total_mastery_score(&mut self, summoner_id: &str) -> Result<i32, ClientError> {
        let url: Uri = format!(
            "{}/champion-mastery/v4/scores/by-summoner/{}",
            self.base_url, summoner_id
        )
        .parse()
        .unwrap();
        self.cached_resp(url).await
    }

    pub async fn get_league_exp_entries(
        &mut self,
        queue: RankedQueue,
        tier: RankedTier,
        division: Division,
        pages: Option<i32>,
    ) -> Result<Vec<LeagueInfo>, ClientError> {
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

        self.cached_resp(url).await
    }

    #[cfg(test)]
    pub(crate) fn get_status(&self, status: u16) -> Result<(), ClientError> {
        ClientError::check_status(self.region.clone(), status)
    }
}

#[async_trait]
impl CachedClient for LeagueClient {
    async fn cached_resp<T: Debug + DeserializeOwned + Send>(
        &self,
        url: Uri,
    ) -> Result<T, ClientError> {
        let maybe_resp: Option<T> = self
            .cache
            .lock()
            .get(&url)
            .map(|res| serde_json::from_str(res).unwrap());

        if let Some(resp) = maybe_resp {
            debug!("Found cached: {:?}", resp);
            Ok(resp)
        } else {
            debug!("Nothing in cache. Fetching from league API...");
            // We got nothing in cache, try fetching from utl
            let url2 = url.clone();
            let header = HeaderValue::from_str(&self.api_key).unwrap();
            let req = Request::builder()
                .header("X-Riot-Token", header)
                .uri(url)
                .body(Body::default())
                .unwrap();
            let resp = self.client.request(req).await.context(HyperError)?;
            let body = resp.into_body();
            let bytes = hyper::body::to_bytes(body).await.context(HyperError)?;
            let string_response = String::from_utf8_lossy(&bytes);
            debug!("Deserializing...");
            let deserialized: T = serde_json::from_str(&string_response).unwrap();
            self.cache.lock().insert(url2, string_response.into_owned());
            Ok(deserialized)
        }
    }
}

impl Default for LeagueClient {
    fn default() -> LeagueClient {
        LeagueClient::new(Region::default()).expect("Please provide API_KEY environment variable")
    }
}

fn check_token(token: &str) -> Result<(), ClientError> {
    ensure!(
        token.contains("RGAPI"),
        WrongToken {
            token: token.to_owned()
        }
    );
    ensure!(
        token.len() == 42_usize,
        WrongToken {
            token: token.to_owned()
        }
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::LeagueClient;
    use crate::constants::{LanguageCode, RankedQueue, RankedTier, Region};

    use futures::prelude::*;
    use futures::{Future, FutureExt, TryFutureExt};
    use pretty_env_logger;

    use crate::constants::division::Division;
    use crate::dto::api::{ChampionInfo, ChampionMastery, Summoner};
    use crate::dto::ddragon::ChampionFullData;
    use crate::error::ClientError;
    use crate::types::Cache;
    use log::debug;
    use std::time::Instant;

    #[cfg(test)]
    fn print_cache(cache: Cache) {
        debug!("{:?}", cache.lock().keys().collect::<Vec<_>>())
    }

    #[test]
    fn gets_summoner_data() {
        smol::run(async {
            pretty_env_logger::init();
            let mut lapi = LeagueClient::new(Region::NA).unwrap();
            let sum = lapi.get_summoner_by_name("Santorin").await.unwrap();
            assert_eq!(
                &sum.account_id,
                "rPnj4h5W6OhejxB-AO3hLOQctgZcckqV_82N_8_WuCFdO2A"
            )
        })
    }

    #[test]
    fn lapi_caches_properly() {
        smol::run(async {
            let mut cli = LeagueClient::new(Region::RU).unwrap();
            let cache = cli.cache.clone();
            let _ = cli.get_summoner_by_name("Vetro").await.unwrap();
            let now = Instant::now();
            let _ = cli.get_summoner_by_name("Vetro").await.unwrap();
            assert!(now.elapsed().as_millis() <= 2);
            print_cache(cache);
        })
    }

    #[test]
    fn gets_champion_info() {
        smol::run(async {
            let mut lapi = LeagueClient::new(Region::default()).unwrap();
            let champ_info = lapi.get_champion_info().await.unwrap();
            assert!(champ_info.free_champion_ids.len() > 10);
            assert!(champ_info.free_champion_ids_for_new_players.len() > 0);
            assert_ne!(champ_info.max_new_player_level, 0)
        })
    }

    #[test]
    fn gets_champion_masteries() {
        smol::run(async {
            let mut lapi = LeagueClient::new(Region::NA).unwrap();
            let summoner = lapi.get_summoner_by_name("Santorin").await.unwrap();
            let masteries = lapi.get_champion_masteries(&summoner.id).await.unwrap();
            assert_ne!(masteries.len(), 0)
        })
    }

    #[test]
    fn gets_champion_mastery_by_id() {
        smol::run(async {
            let mut lapi = LeagueClient::new(Region::default())
                .unwrap()
                .with_ddragon(LanguageCode::UNITED_STATES)
                .await;
            let mut ddragon_client = lapi.ddragon();
            let lee_sin: ChampionFullData = ddragon_client.get_champion("LeeSin").await.unwrap();
            let summoner: Summoner = lapi.get_summoner_by_name("Santorin").await.unwrap();
            let mastery: ChampionMastery = lapi
                .get_champion_mastery_by_id(&summoner.id, lee_sin.key.parse().unwrap())
                .await
                .unwrap();

            assert_eq!(mastery.champion_id, 64);
            assert_eq!(mastery.champion_level, 7);
            assert!(mastery.champion_points >= 93748)
        })
    }

    #[test]
    fn gets_total_mastery_score() {
        smol::run(async {
            let mut lapi = LeagueClient::new(Region::default())
                .map_err(|e| {
                    println!("{}", e);
                    e
                })
                .unwrap();
            let summoner: Summoner = lapi.get_summoner_by_name("Santorin").await.unwrap();
            let score = lapi.get_total_mastery_score(&summoner.id).await.unwrap();
            assert!(score >= 192)
        })
    }

    #[test]
    fn gets_league_exp() -> Result<(), ClientError> {
        smol::run(async {
            let mut lapi = LeagueClient::new(Region::default()).unwrap();
            let challengers = lapi
                .get_league_exp_entries(
                    RankedQueue::SOLO,
                    RankedTier::CHALLENGER,
                    Division::I,
                    None,
                )
                .await
                .unwrap();
            assert!(challengers.len() > 0);
            Ok(())
        })
    }
}
