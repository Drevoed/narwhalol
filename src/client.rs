use crate::constants::{LanguageCode, Region};
use crate::ddragon::DDragonClient;
use crate::dto::api::{ChampionInfo, ChampionMastery, Summoner};
use crate::error::*;
use futures::future::ok;
use futures::{Future, Stream};
use hyper::client::connect::dns::GaiResolver;
use hyper::{
    client::HttpConnector, header::HeaderValue, Body, Client as HttpClient, HeaderMap, Uri,
};
use hyper::Request;
use hyper_tls::HttpsConnector;
use log::{debug, trace};
use serde::de::DeserializeOwned;
use snafu::ResultExt;
use std::collections::HashMap;
use std::env;
use std::fmt::Debug;

type Client = HttpClient<HttpsConnector<HttpConnector<GaiResolver>>, Body>;

/// Main type for calling League API Endpoints.
#[derive(Debug)]
pub struct LeagueClient {
    client: Client,
    api_key: String,
    region: Region,
    base_url: String,
    pub ddragon: Option<DDragonClient>,
    cache: HashMap<Uri, String>,
    default_request: Request<()>
}

fn construct_hyper_client() -> Client {
    let mut https = HttpsConnector::new(4).unwrap();
    https.https_only(true);
    let cli = HttpClient::builder()
        .build::<_, Body>(https);
    cli
}

impl LeagueClient {
    /// Constructor function for LeagueAPI struct, accepts Region type as a parameter
    pub fn new(region: Region) -> Result<LeagueClient, env::VarError> {
        let mut headers = HeaderMap::new();
        let base_url = format!("https://{}.api.riotgames.com/lol", region.as_platform_str());
        let api_key = std::env::var("RIOT_API_KEY")?;
        let client = construct_hyper_client();
        let default_request = Request::builder()
            .header("X-Riot-Token", HeaderValue::from_str(&api_key).unwrap()).body(()).unwrap();
        Ok(LeagueClient {
            client,
            api_key,
            region,
            base_url,
            ddragon: None,
            cache: HashMap::new(),
            default_request
        })
    }

    pub fn ddragon(self, language: LanguageCode) -> Result<Self, ClientError> {
        let ddragon = DDragonClient::new(language).context(ReqwestError {})?;
        Ok(LeagueClient {
            ddragon: Some(ddragon),
            ..self
        })
    }

    pub fn get_summoner_by_name(&mut self, name: &str) -> impl Future<Item = Summoner, Error = ClientError> {
        trace!("Getting summoner with name: {}", &name);
        let url: Url = format!("{}/summoner/v4/summoners/by-name/{}", self.base_url, name)
            .parse().unwrap();
        debug!("Constructed url: {:?}", &url);
        self.get_deserialized_or_add_raw(url)
    }

    pub fn get_champion_info(&mut self) -> impl Future<Item = ChampionInfo, Error = ClientError> {
        let url: Url = format!("{}/platform/v3/champion-rotations", self.base_url)
            .parse().unwrap();
        self.get_deserialized_or_add_raw(url)
    }

    pub fn get_champion_masteries(
        &mut self,
        summoner_id: &str,
    ) -> impl Future<Item = Vec<ChampionMastery>, Error = ClientError> {
        trace!("Getting champion masteries for id: {}", &summoner_id);
        let url: Url = format!(
            "{}/champion-mastery/v4/champion-masteries/by-summoner/{}",
            self.base_url, summoner_id
        )
        .parse().unwrap();
        self.get_deserialized_or_add_raw(url)
    }

    pub fn get_champion_mastery_by_id(
        &mut self,
        summoner_id: &str,
        champion_id: u64,
    ) -> impl Future<Item = ChampionMastery, Error = ClientError> {
        let url: Url = format!(
            "{}/champion-mastery/v4/champion-masteries/by-summoner/{}/by-champion/{}",
            self.base_url, summoner_id, champion_id
        )
        .parse().unwrap();
        self.get_deserialized_or_add_raw(url)
    }

    pub fn get_total_mastery_score(&mut self, summoner_id: &str) -> impl Future<Item = i32, Error = ClientError> {
        let url: Uri = format!(
            "{}/champion-mastery/v4/scores/by-summoner/{}",
            self.base_url, summoner_id
        )
        .parse().unwrap();
        self.get_deserialized_or_add_raw(url)
    }

    fn get_deserialized_or_add_raw<T>(
        &mut self,
        url: Uri,
    ) -> impl Future<Item = T, Error = ClientError>
    where
        T: Debug + DeserializeOwned,
    {
        match self.cache.get(&url) {
            Some(resp) => {
                let returnee: T = serde_json::from_str(resp).unwrap();
                ok(returnee)
            }
            None => self
                .client
                .get(url.clone())
                .and_then(|resp| {
                    let body = resp.into_body();
                    body.concat2()
                })
                .map(|body| {
                    let string_response = String::from(body.into_bytes());
                    self.cache.insert(url.clone(), string_response);
                    let returnee: T = serde_json::from_str(self.cache.get(&url).unwrap())
                        .expect("Could not parse");
                    returnee
                }),
        }
    }

    #[cfg(test)]
    pub(crate) fn get_status(&self, status: u16) -> ApiResult<()> {
        ClientError::check_status(self.region.clone(), status)
    }

    #[cfg(test)]
    pub(crate) fn print_cache(&self) {
        println!("cache: {:#?}", self.cache.keys().collect::<Vec<_>>())
    }
}

impl Default for LeagueClient {
    fn default() -> LeagueClient {
        LeagueClient::new(Region::default()).expect("Please provide API_KEY environment variable")
    }
}

#[cfg(test)]
mod tests {
    use crate::dto::api::{ChampionMastery, Summoner};
    use crate::dto::ddragon::ChampionFullData;
    use crate::error::ClientError;
    use crate::{DDRAGON_CLIENT, LEAGUE_CLIENT};

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
    }
}
