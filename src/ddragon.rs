use crate::constants::LanguageCode;
use crate::dto::ddragon::{AllChampions, ChampionExtended, ChampionFullData};
use crate::error::{ClientError, HyperError};
use crate::types::{Cache, Client};
use crate::utils::{construct_hyper_client, get_latest_ddragon_version, CachedClient};
use log::debug;

use futures::prelude::*;
use hyper::{Uri, Request, Body};

use std::collections::HashMap;

use async_trait::async_trait;

use std::sync::{Arc, Mutex};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use hyper::header::HeaderValue;
use snafu::ResultExt;

#[derive(Debug)]
pub struct DDragonClient {
    client: Client,
    cache: Cache,
    version: String,
    base_url: String,
}

impl DDragonClient {
    pub async fn new(language: LanguageCode) -> Result<DDragonClient, ClientError> {
        let client = construct_hyper_client();
        let cache: Cache = Arc::new(Mutex::new(HashMap::new()));
        let version = get_latest_ddragon_version(client.clone()).await?;
        let base_url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/{}",
            version, &language
        );
        Ok(DDragonClient {
            version: version.into(),
            base_url,
            client,
            cache,
        })
    }

    pub(crate) async fn new_for_lapi(client: Client, cache: Cache, lang: LanguageCode) -> Result<DDragonClient, ClientError> {
        let version = get_latest_ddragon_version(client.clone()).await?;
        let base_url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/{}",
            version, &lang
        );
        Ok(DDragonClient {
            version: version.into(),
            client,
            cache,
            base_url,
        })
    }

    pub async fn get_champions(&mut self) -> Result<AllChampions, ClientError> {
        let url: Uri = format!("{}/champion.json", &self.base_url).parse().unwrap();
        self.cached_resp(url).await
    }

    pub async fn get_champion(&mut self, name: &str) -> Result<ChampionFullData, ClientError> {
        let name = name.to_owned();
        let url: Uri = format!("{}/champion/{}.json", &self.base_url, &name)
            .parse()
            .unwrap();
        let mut resp = self.cached_resp::<ChampionExtended>(url).await?;
        Ok(resp.data.remove(&name).unwrap())
    }
}

#[async_trait]
impl CachedClient for DDragonClient {
    async fn cached_resp<T: Debug + DeserializeOwned + Send>(&self, url: Uri) -> Result<T, ClientError> {
        let maybe_resp: Option<T> = self.cache
            .lock()
            .unwrap()
            .get(&url)
            .map(|res| serde_json::from_str(res).unwrap());

        if let Some(resp) = maybe_resp {
            debug!("Found cached: {:?}", resp);
            Ok(resp)
        } else {
            debug!("Nothing in cache. Fetching...");
            // We got nothing in cache, try fetching from utl
            let url2 = url.clone();
            let req = Request::builder()
                .uri(url)
                .body(Default::default())
                .unwrap();
            let resp = self.client
                .request(req)
                .await
                .context(HyperError)?;
            let body = resp.into_body();
            let bytes = hyper::body::to_bytes(body).await.unwrap();
            let string_response = String::from_utf8(bytes.to_vec()).unwrap();
                //.context(FromUTF8Error);
            debug!("Deserializing...");
            let deserialized: T = serde_json::from_str(&string_response).unwrap();
            self.cache.lock().unwrap().insert(url2, string_response);
            Ok(deserialized)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::LanguageCode;
    use crate::ddragon::DDragonClient;
    use crate::dto::ddragon::{AllChampions, ChampionFullData};
    use std::time::Instant;

    #[test]
    fn creates_proper_instance() {
        smol::run(async {
            let cli = DDragonClient::new(LanguageCode::RUSSIA).await.unwrap();
            println!("{:?}", &cli)
        })
    }

    #[test]
    fn gets_full_champion_data() {
        smol::run(async {
            let mut client = DDragonClient::new(LanguageCode::UNITED_STATES).await.unwrap();
            let xayah = client.get_champion("Xayah").await.unwrap();
            assert_eq!(xayah.name, "Xayah");
        })
    }
}
