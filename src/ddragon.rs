use crate::constants::LanguageCode;
use crate::dto::ddragon::{AllChampions, ChampionExtended, ChampionFullData};
use crate::error::ClientError;
use crate::types::{Cache, Client};
use crate::utils::{cached_resp, construct_hyper_client};
use futures::Future;
use hyper::Uri;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct DDragonClient {
    client: Client,
    cache: Cache,
    version: String,
    base_url: String,
}

impl DDragonClient {
    pub fn new(language: LanguageCode) -> Result<DDragonClient, ClientError> {
        let client = construct_hyper_client();
        let cache: Cache = Arc::new(Mutex::new(HashMap::new()));
        /*let mut versions: Vec<String> = client
        .get("https://ddragon.leagueoflegends.com/api/versions.json")
        .send()?
        .json()?;*/
        let version = "9.15.1";
        let base_url = format!(
            "http://ddragon.leagueoflegends.com/cdn/{}/data/{}",
            version, &language
        );
        let ddragon = DDragonClient {
            version: version.into(),
            base_url,
            client,
            cache,
        };
        Ok(ddragon)
    }

    pub(crate) fn new_for_lapi(
        client: Client,
        cache: Cache,
        lang: LanguageCode,
    ) -> Result<DDragonClient, ClientError> {
        let version = "9.15.1";
        let base_url = format!(
            "http://ddragon.leagueoflegends.com/cdn/{}/data/{}",
            version, &lang
        );
        Ok(DDragonClient {
            version: version.into(),
            client,
            cache,
            base_url,
        })
    }

    pub fn get_champions(&mut self) -> impl Future<Item = AllChampions, Error = ClientError> {
        let url: Uri = format!("{}/champion.json", &self.base_url).parse().unwrap();
        cached_resp(self.client.clone(), self.cache.clone(), url, None)
    }

    pub fn get_champion(
        &mut self,
        name: &str,
    ) -> impl Future<Item = ChampionFullData, Error = ClientError> {
        let name = name.to_owned();
        let url: Uri = format!("{}/champion/{}.json", &self.base_url, &name)
            .parse()
            .unwrap();
        cached_resp(self.client.clone(), self.cache.clone(), url, None)
    }
}

/*#[cfg(test)]
mod tests {
    use crate::dto::ddragon::{AllChampions, ChampionFullData};
    use std::time::Instant;

    #[test]
    fn caches_properly() {
        let mut client = DDRAGON_CLIENT.lock().unwrap();
        let champs = client.get_champions().unwrap();
        drop(champs);
        let now = Instant::now();
        let champs: AllChampions = client.get_champions();
        assert!(now.elapsed().as_millis() < 100);
        assert_eq!("103", &champs.data.get("Ahri").unwrap().key);
    }

    #[test]
    fn gets_full_champion_data() {
        let mut client = DDRAGON_CLIENT.lock().unwrap();
        let xayah: ChampionFullData = client.get_champion("Xayah").unwrap();
        assert_eq!(xayah.name, "Xayah");
    }
}*/
