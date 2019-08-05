use crate::constants::region::Region;
use crate::dto::api::Summoner;
use log::debug;
use reqwest::{Client, Method, Request, Url};

#[derive(Debug)]
pub struct LeagueAPI {
    client: Client,
    api_key: String,
    region: Region,
    base_url: String,
}

impl LeagueAPI {
    pub fn new_dev_client(region: Region) -> LeagueAPI {
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
        debug!("url: {:?}", &url);
        let summoner = self
            .client
            .request(Method::GET, url)
            .header("X-Riot-Token", &self.api_key)
            .send()?
            .json()?;
        debug!("Got summoner: {:?}", &summoner);
        Ok(summoner)
    }
}
