use crate::constants::LanguageCode;
use crate::dto::ddragon::{AllChampions, ChampionExtended, ChampionFullData};
use crate::error::ClientError;
use crate::types::{Cache, Client};
use crate::utils::{cached_resp, construct_hyper_client, get_latest_ddragon_version};

use futures::{Future, FutureExt, Stream, StreamExt, TryFutureExt, TryStream, TryStreamExt};
use hyper::{Uri, Chunk};

use std::collections::HashMap;

use std::sync::{Arc, Mutex};

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
        cached_resp(self.client.clone(), self.cache.clone(), url, None).await
    }

    pub async fn get_champion(&mut self, name: &str) -> Result<ChampionFullData, ClientError> {
        let name = name.to_owned();
        let url: Uri = format!("{}/champion/{}.json", &self.base_url, &name)
            .parse()
            .unwrap();
        let resp =
            cached_resp::<ChampionExtended>(self.client.clone(), self.cache.clone(), url, None)
                .await;
        resp.map(|mut ext| ext.data.remove(&name).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::LanguageCode;
    use crate::ddragon::DDragonClient;
    use crate::dto::ddragon::{AllChampions, ChampionFullData};
    use std::time::Instant;

    #[tokio::test]
    async fn creates_proper_instance() {
        let cli = DDragonClient::new(LanguageCode::RUSSIA).await.unwrap();
        println!("{:?}", &cli)
    }

    #[tokio::test]
    async fn gets_full_champion_data() {
        let mut client = DDragonClient::new(LanguageCode::UNITED_STATES).await.unwrap();
        let xayah = client.get_champion("Xayah").await.unwrap();
        assert_eq!(xayah.name, "Xayah");
    }
}
