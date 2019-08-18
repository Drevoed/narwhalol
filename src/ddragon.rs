use crate::constants::LanguageCode;
use crate::dto::ddragon::{AllChampions, ChampionExtended, ChampionFullData};
use crate::error::ClientError;
use crate::types::{Cache, Client};
use crate::utils::{cached_resp, construct_hyper_client};
use futures::compat::{Future01CompatExt, Stream01CompatExt};
use futures::{Future, FutureExt, Stream, StreamExt, TryFutureExt, TryStream, TryStreamExt};
use hyper::Uri;

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
    pub fn new(language: LanguageCode) -> Result<DDragonClient, ClientError> {
        let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();
        let client = construct_hyper_client();
        let cache: Cache = Arc::new(Mutex::new(HashMap::new()));
        let mut versions = runtime
            .block_on(
                client
                    .get(
                        "https://ddragon.leagueoflegends.com/api/versions.json"
                            .parse()
                            .unwrap(),
                    )
                    .and_then(|resp| resp.into_body().try_concat())
                    .map_ok(|chunk| {
                        let string_resp = String::from_utf8(chunk.to_vec()).unwrap();
                        let versions: Vec<String> = serde_json::from_str(&string_resp).unwrap();
                        versions
                    }),
            )
            .unwrap();
        let version = versions.remove(0);
        drop(versions);
        let base_url = format!(
            "https://ddragon.leagueoflegends.com/cdn/{}/data/{}",
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

    pub fn get_champions(&mut self) -> impl Future<Output = Result<AllChampions, ClientError>> {
        let url: Uri = format!("{}/champion.json", &self.base_url).parse().unwrap();
        cached_resp(self.client.clone(), self.cache.clone(), url, None)
    }

    pub fn get_champion(
        &mut self,
        name: &str,
    ) -> impl Future<Output = Result<ChampionFullData, ClientError>> {
        let name = name.to_owned();
        let url: Uri = format!("{}/champion/{}.json", &self.base_url, &name)
            .parse()
            .unwrap();
        cached_resp::<ChampionExtended>(self.client.clone(), self.cache.clone(), url, None)
            .map_ok(move |mut ext| ext.data.remove(&name).unwrap())
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
        let cli = DDragonClient::new(LanguageCode::RUSSIA).unwrap();
        println!("{:?}", &cli)
    }

    #[test]
    fn gets_full_champion_data() {
        let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();
        let mut client = DDragonClient::new(LanguageCode::UNITED_STATES).unwrap();
        let xayah: ChampionFullData = runtime.block_on(client.get_champion("Xayah")).unwrap();
        assert_eq!(xayah.name, "Xayah");
    }
}
