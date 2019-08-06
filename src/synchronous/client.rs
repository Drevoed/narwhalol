use crate::constants::Region;
use crate::dto::api::{ChampionInfo, ChampionMastery, Summoner};
use log::debug;
use reqwest::{Client, Method, Url};

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
    pub fn new(region: Region) -> LeagueAPI {
        let client = Client::new();
        let base_url = format!("https://{}.api.riotgames.com/lol", region.as_platform_str());
        let api_key =
            std::env::var("RIOT_API_KEY").expect("Please provide API_KEY environment variable");
        LeagueAPI {
            client,
            api_key,
            region,
            base_url,
        }
    }

    pub fn get_summoner_by_name(&self, name: &str) -> Result<Summoner, reqwest::Error> {
        let url: Url = format!("{}/summoner/v4/summoners/by-name/{}", self.base_url, name)
            .parse()
            .unwrap();
        debug!("Constructed url: {:?}", &url);
        let summoner = self
            .client
            .request(Method::GET, url)
            .header("X-Riot-Token", &self.api_key)
            .send()?
            .json()?;
        Ok(summoner)
    }

    pub fn get_champion_info(&self) -> Result<ChampionInfo, reqwest::Error> {
        let url: Url = format!("{}/platform/v3/champion-rotations", self.base_url)
            .parse()
            .unwrap();
        let champ_info: ChampionInfo = self
            .client
            .get(url)
            .header("X-Riot-Token", &self.api_key)
            .send()?
            .json()?;
        Ok(champ_info)
    }

    pub fn get_champion_masteries(
        &self,
        summoner_id: &str,
    ) -> Result<Vec<ChampionMastery>, reqwest::Error> {
        let url: Url = format!(
            "{}/champion-mastery/v4/champion-masteries/by-summoner/{}",
            self.base_url, summoner_id
        )
        .parse()
        .unwrap();
        let masteries: Vec<ChampionMastery> = self
            .client
            .get(url)
            .header("X-Riot-Token", &self.api_key)
            .send()?
            .json()?;
        Ok(masteries)
    }

    pub fn get_champion_mastery_by_id(
        &self,
        summoner_id: &str,
        champion_id: i64,
    ) -> Result<ChampionMastery, reqwest::Error> {
        let url: Url = format!(
            "{}/champion-mastery/v4/champion-masteries/by-summoner/{}/by-champion/{}",
            self.base_url, summoner_id, champion_id
        )
        .parse()
        .unwrap();

        let mastery: ChampionMastery = self
            .client
            .get(url)
            .header("X-Riot-Token", &self.api_key)
            .send()?
            .json()?;
        Ok(mastery)
    }

    pub fn get_total_mastery_score(&self, summoner_id: &str) -> Result<i32, reqwest::Error> {
        let url: Url = format!(
            "{}/champion-mastery/v4/scores/by-summoner/{}",
            self.base_url, summoner_id
        )
        .parse()
        .unwrap();

        let score = self
            .client
            .get(url)
            .header("X-Riot-Token", &self.api_key)
            .send()?
            .json()?;
        Ok(score)
    }
}

impl Default for LeagueAPI {
    fn default() -> LeagueAPI {
        LeagueAPI::new(Region::default())
    }
}
