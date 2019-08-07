use crate::constants::LanguageCode;
use crate::dto::ddragon::{AllChampions, DDragonResponse};
use reqwest::{Client, Url};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

#[derive(Debug)]
pub struct DDragonClient {
    version: String,
    client: Client,
    base_url: String,
    cache: HashMap<Url, String>,
}

impl DDragonClient {
    pub fn new(language: LanguageCode) -> Result<DDragonClient, reqwest::Error> {
        let client = Client::new();
        let mut versions: Vec<String> = client
            .get("https://ddragon.leagueoflegends.com/api/versions.json")
            .send()?
            .json()?;
        let version = versions.remove(0);
        drop(versions);
        let base_url = format!(
            "http://ddragon.leagueoflegends.com/cdn/{}/data/{}",
            &version, &language
        );
        let cache = HashMap::new();
        let ddragon = DDragonClient {
            version,
            client,
            base_url,
            cache,
        };
        Ok(ddragon)
    }

    pub fn get_champions(&mut self) -> Result<AllChampions, reqwest::Error> {
        let url: Url = format!("{}/champion.json", &self.base_url).parse().unwrap();
        self.get_parsed_or_add_raw::<AllChampions>(url)
    }

    pub fn get_champion(&mut self, name: &str) -> Result<serde_json::Value, reqwest::Error> {
        let url: Url = format!("{}/champion/{}.json", &self.base_url, name)
            .parse()
            .unwrap();
        self.get_parsed_or_add_raw(url)
    }

    fn get_parsed_or_add_raw<T: DeserializeOwned>(
        &mut self,
        url: Url,
    ) -> Result<T, reqwest::Error> {
        match self.cache.get(&url) {
            Some(resp) => {
                let returnee: T = serde_json::from_str(resp).unwrap();
                Ok(returnee)
            }
            None => {
                let response: String = self.client.get(url.clone()).send()?.text()?;
                self.cache.insert(url.clone(), response);
                let returnee = serde_json::from_str(self.cache.get(&url).unwrap()).unwrap();
                Ok(returnee)
            }
        }
    }
}
