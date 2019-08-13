use crate::error::ClientError;
use crate::types::{Cache, Client};
use futures::future::{ok, Either};
use futures::{Future, Stream};
use hyper::header::HeaderValue;
use hyper::{
    Body, Client as HttpClient,
    Request, Uri,
};
use hyper_tls::HttpsConnector;

use serde::de::DeserializeOwned;


use std::fmt::Debug;
use std::sync::{Arc};


pub(crate) fn construct_hyper_client() -> Client {
    let mut https = HttpsConnector::new(4).unwrap();
    https.https_only(true);
    let cli = HttpClient::builder().build::<_, Body>(https);
    Arc::new(cli)
}

pub fn cached_resp<T: Debug + DeserializeOwned>(
    client: Client,
    cache: Cache,
    url: Uri,
    api_key: Option<&str>,
) -> impl Future<Item = T, Error = ClientError> {
    let maybe_resp: Option<T> = cache
        .lock()
        .unwrap()
        .get(&url)
        .map(|res| serde_json::from_str(res).unwrap());

    if let Some(resp) = maybe_resp {
        debug!("Found cached: {:?}", resp);
        Either::A(ok(resp))
    } else {
        let url2 = url.clone();
        let mut header = HeaderValue::from_str("").unwrap();
        if let Some(api_key) = api_key {
            header = HeaderValue::from_str(api_key).unwrap();
        }
        let _req = Request::builder();
        let req = Request::builder()
            .header("X-Riot-Token", header)
            .uri(url)
            .body(Body::from(""))
            .unwrap();
        let do_request = client.request(req);
        Either::B(
            do_request
                .and_then(|resp| {
                    let body = resp.into_body();
                    body.concat2()
                })
                .map(move |chunk| {
                    let string_response = String::from_utf8(chunk.to_vec()).unwrap();
                    let deserialized: T = serde_json::from_str(&string_response).unwrap();
                    cache.lock().unwrap().insert(url2, string_response);
                    deserialized
                })
                .map_err(|e| ClientError::Other { source: e }),
        )
    }
}
