use crate::types::Client;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use hyper::{Client as HttpClient, client::connect::dns::GaiResolver, client::HttpConnector, Uri, Body};
use hyper_tls::HttpsConnector;
use futures::{Future, Stream};
use crate::error::ClientError;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use futures::future::{Either, ok};

type Cache<'a, K = Uri, V = String> = Arc<Mutex<HashMap<K, V>>>;

lazy_static!{
    pub(crate) static ref DDRAGON_CACHE: Cache<'static> = Arc::new(Mutex::new(HashMap::new()));
    pub(crate) static ref LEAGUE_CACHE: Cache<'static> = Arc::new(Mutex::new(HashMap::new()));
}

pub(crate) fn get_deserialized_or_add_raw<T>(
    client: Arc<Client>,
    cache: Cache<'static>,
    url: Uri,
) -> impl Future<Item = T, Error = ClientError>
    where
        T: Debug + DeserializeOwned,
{
    let mut cache = cache.lock().unwrap();
    match cache.get(&url) {
        Some(resp) => {
            let returnee: T = serde_json::from_str(resp).unwrap();
            Either::A(ok(returnee))
        }
        None => Either::B(client
            .get(url.clone())
            .and_then(move |resp| {
                let body = resp.into_body();
                body.concat2()
            })
            .map(move |chunk| {
                let v = chunk.to_vec();
                let string_response = String::from_utf8(v).unwrap();
                cache.insert(url.clone(), string_response);
                let returnee: T = serde_json::from_str(cache.get(&url).unwrap())
                    .expect("Could not parse");
                returnee
            })
            .map_err(|e| {ClientError::Other {source: e}})),
    }
}

pub(crate) fn construct_hyper_client() -> Client {
    let mut https = HttpsConnector::new(4).unwrap();
    https.https_only(true);
    let cli = HttpClient::builder()
        .build::<_, Body>(https);
    cli
}
