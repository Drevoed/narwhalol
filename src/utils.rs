use crate::error::ClientError;
use crate::types::{Cache, Client};
use futures::future::{ok, Either};
use futures::prelude::*;
use futures::{Future, FutureExt, StreamExt, TryFutureExt, TryStreamExt};
use hyper::header::HeaderValue;
use hyper::{Body, Chunk, Client as HttpClient, Request, Response, Uri};
use hyper_tls::HttpsConnector;

use serde::de::DeserializeOwned;

use std::fmt::Debug;
use std::sync::Arc;

pub(crate) async fn get_latest_ddragon_version(client: Client) -> Result<String, ClientError> {
    let resp = client.get(
        "https://ddragon.leagueoflegends.com/api/versions.json"
            .parse()
            .unwrap(),
    ).await.map_err(|e| ClientError::Other { source: e })?;
    let chunk: Chunk = resp
        .into_body()
        .try_concat()
        .await
        .map_err(|e| ClientError::Other { source: e })?;
    let string_resp = String::from_utf8(chunk.to_vec()).unwrap();
    let mut versions: Vec<String> = serde_json::from_str(&string_resp).unwrap();
    let version = versions.remove(0);
    Ok(version)
}

/// Helper function that constructs an https hyper client
pub(crate) fn construct_hyper_client() -> Client {
    let mut https = HttpsConnector::new().unwrap();
    https.https_only(true);
    let cli = HttpClient::builder().build::<_, Body>(https);
    Arc::new(cli)
}

/// Util function that either returns deserialized response from cache or fetches response from url and then deserializes it
pub(crate) async fn cached_resp<T: Debug + DeserializeOwned>(
    client: Client,
    cache: Cache,
    url: Uri,
    api_key: Option<&str>,
) -> Result<T, ClientError> {
    let maybe_resp: Option<T> = cache
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
        let mut header = HeaderValue::from_str("").unwrap();
        // Add `X-Riot-Token` header anyway, the header value is empty if it's a ddragon url
        if let Some(api_key) = api_key {
            header = HeaderValue::from_str(api_key).unwrap();
        }
        let req = Request::builder()
            .header("X-Riot-Token", header)
            .uri(url)
            .body(Body::from(""))
            .unwrap();
        let resp: Response<Body> = client
            .request(req)
            .await
            .map_err(|e| ClientError::Other { source: e })?;
        let chunk: Chunk = resp
            .into_body()
            .try_concat()
            .await
            .map_err(|e| ClientError::Other { source: e })?;
        let string_response = String::from_utf8(chunk.to_vec()).unwrap();
        debug!("Deserializing...");
        let deserialized: T = serde_json::from_str(&string_response).unwrap();
        cache.lock().unwrap().insert(url2, string_response);
        Ok(deserialized)
    }
}
