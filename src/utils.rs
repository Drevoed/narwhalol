use crate::error::ClientError;
use crate::types::{Cache, Client};
use futures::future::{ok, Either};
use futures::{Future, Stream};
use hyper::header::HeaderValue;
use hyper::{Body, Client as HttpClient, Request, Uri};
use hyper_tls::HttpsConnector;

use serde::de::DeserializeOwned;

use std::fmt::Debug;
use std::sync::Arc;

/// Helper function that constructs an https hyper client
pub(crate) fn construct_hyper_client() -> Client {
    let mut https = HttpsConnector::new(4).unwrap();
    https.https_only(true);
    let cli = HttpClient::builder().build::<_, Body>(https);
    Arc::new(cli)
}

/// Util function that either returns deserialized response from cache or fetches response from url and then deserializes it
pub(crate) fn cached_resp<T: Debug + DeserializeOwned>(
    client: Client,
    cache: Cache,
    url: Uri,
    api_key: Option<&str>,
) -> impl Future<Item = T, Error = ClientError> {
    // get None or Some(T) from cache
    let maybe_resp: Option<T> = cache
        .lock()
        .unwrap()
        .get(&url)
        .map(|res| serde_json::from_str(res).unwrap());

    if let Some(resp) = maybe_resp {
        debug!("Found cached");
        // Return deserialized from cache
        Either::A(ok(resp))
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
        let do_request = client.request(req);
        // Return future that fetches from url and deserializes
        Either::B(
            do_request
                // Get body
                .and_then(|resp| {
                    let body = resp.into_body();
                    body.concat2()
                })
                // Deserialize body to type T, insert the url and plaintext response to cache and return deserialized
                .map(move |chunk| {
                    let string_response = String::from_utf8(chunk.to_vec()).unwrap();
                    debug!("Deserializing...");
                    let deserialized: T = serde_json::from_str(&string_response).unwrap();
                    cache.lock().unwrap().insert(url2, string_response);
                    deserialized
                })
                // Turn errors into ClientErrors with context
                .map_err(|e| ClientError::Other { source: e }),
        )
    }
}
