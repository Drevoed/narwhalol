use crate::constants::LanguageCode;
use crate::dto::ddragon::{AllChampions, ChampionExtended, ChampionFullData};
use crate::error::ClientError;
use crate::types::{Client, Cache};
use crate::utils::{construct_hyper_client, CacheFutureSpawner};
use futures::Future;
use hyper::Uri;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct DDragonClient<T>
    where T: Debug + DeserializeOwned
{
    version: String,
    base_url: String,
    spawner: CacheFutureSpawner<T>
}

impl<T> DDragonClient<T>
    where T: DeserializeOwned + Debug
{
    pub fn new(language: LanguageCode) -> Result<DDragonClient<T>, ClientError> {
        let client = construct_hyper_client();
        let cache: Cache = Arc::new(Mutex::new(HashMap::new()));
        let spawner = CacheFutureSpawner::new(client, cache, None);
        /*let mut versions: Vec<String> = client
        .get("https://ddragon.leagueoflegends.com/api/versions.json")
        .send()?
        .json()?;*/
        let version = "9.15.1";
        let base_url = format!(
            "http://ddragon.leagueoflegends.com/cdn/{}/data/{}",
            version, &language
        );
        let cache = HashMap::new();
        let ddragon = DDragonClient {
            version: version.into(),
            base_url,
            spawner
        };
        Ok(ddragon)
    }

    pub fn get_champions(&mut self) -> impl Future<Item = AllChampions, Error = ClientError> {
        let url: Uri = format!("{}/champion.json", &self.base_url).parse().unwrap();
        get_deserialized_or_add_raw(self.client.clone(), DDRAGON_CACHE.clone(), url)
    }

    pub fn get_champion(
        &mut self,
        name: &str,
    ) -> impl Future<Item = ChampionFullData, Error = ClientError> {
        let name = name.to_owned();
        let url: Uri = format!("{}/champion/{}.json", &self.base_url, &name)
            .parse()
            .unwrap();
        get_deserialized_or_add_raw::<ChampionExtended>(
            self.client.clone(),
            DDRAGON_CACHE.clone(),
            url,
        )
        .map(move |mut ext| ext.data.remove(&name).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::dto::ddragon::{AllChampions, ChampionFullData};
    use crate::DDRAGON_CLIENT;
    use std::time::Instant;

    #[test]
    fn caches_properly() {
        let mut client = DDRAGON_CLIENT.lock().unwrap();
        let champs = client.get_champions().unwrap();
        drop(champs);
        let now = Instant::now();
        let champs: AllChampions = client.get_champions().unwrap();
        assert!(now.elapsed().as_millis() < 100);
        assert_eq!("103", &champs.data.get("Ahri").unwrap().key);
    }

    #[test]
    fn gets_full_champion_data() {
        let mut client = DDRAGON_CLIENT.lock().unwrap();
        let xayah: ChampionFullData = client.get_champion("Xayah").unwrap();
        assert_eq!(xayah.name, "Xayah");
    }
}
