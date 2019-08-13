use crate::error::ClientError;
use crate::types::{Cache, Client};
use futures::future::{ok, Either};
use futures::{Async, Future, Stream};
use hyper::header::HeaderValue;
use hyper::{
    client::connect::dns::GaiResolver, client::HttpConnector, Body, Client as HttpClient, Request,
    Uri,
};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde::export::PhantomData;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use tokio::prelude::*;

#[derive(Debug)]
pub struct CacheFutureSpawner {
    client: Client,
    cache: Cache,
    api_key: Option<String>,
}

impl CacheFutureSpawner
{
    pub fn new(client: Client, cache: Cache, api_key: Option<String>) -> Self {
        CacheFutureSpawner {
            client,
            cache,
            api_key,
        }
    }
}

impl CacheFutureSpawner {
    pub fn spawn_cache_fut<T: Debug + DeserializeOwned>(&self, url: Uri) -> CacheFuture<T> {
        let cli_clone = self.client.clone();
        let cache_clone = self.cache.clone();
        let api_key_clone = self.api_key.clone();
        CacheFuture::new(cli_clone, cache_clone, url, api_key_clone)
    }

    #[cfg(test)]
    pub(crate) fn print_cache(&self) {
        let cache = self.cache.lock().unwrap();
        println!("cache: {:#?}", cache.keys().collect::<Vec<_>>())
    }
}

pub struct CacheFuture<T> {
    client: Client,
    cache: Cache,
    url: Uri,
    api_key: Option<String>,
    _phantom: PhantomData<T>,
}

impl<T> CacheFuture<T>
where
    T: Debug + DeserializeOwned,
{
    pub fn new(client: Client, cache: Cache, url: Uri, api_key: Option<String>) -> Self {
        CacheFuture {
            client,
            cache,
            url,
            api_key,
            _phantom: PhantomData,
        }
    }
}

impl<T> Future for CacheFuture<T>
where
    T: Debug + DeserializeOwned,
{
    type Item = T;
    type Error = ClientError;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        let cache = self.cache.lock().unwrap();
        let url = self.url.clone();
        let url2 = url.clone();
        let req = match &self.api_key {
            //Riot api request
            Some(api_key) => {
                dbg!("got api key: {}", api_key);
                Request::builder()
                    .header(
                        "X-Riot-Token",
                        HeaderValue::from_str(api_key).unwrap(),
                    )
                    .uri(url)
                    .body(Body::from(""))
                    .unwrap()
            }
            //DDragon request
            None => {
                Request::builder()
                    .uri(url)
                    .body(Body::from(""))
                    .unwrap()
            }
        };
        match cache.get(&url2) {
            Some(resp) => {
                let deserialized: T = serde_json::from_str(resp).unwrap();
                return Ok(Async::Ready(deserialized));
            }
            None => {
                dbg!("got no cache");
                dbg!("beginning poll");
                match self.client.request(req).poll() {
                    Ok(Async::Ready(resp)) => {
                        dbg!("ready");
                        let body = resp.into_body();
                        match body.concat2().poll() {
                            Ok(Async::Ready(chunk)) => {
                                dbg!("ready");
                                let string_resp = String::from_utf8(chunk.to_vec()).unwrap();
                                let deserialized: T = serde_json::from_str(&string_resp).unwrap();
                                return Ok(Async::Ready(deserialized));
                            }
                            Ok(Async::NotReady) => {dbg!("not ready"); return Ok(Async::NotReady)},
                            Err(e) => {dbg!(&e); return Err(ClientError::Other { source: e })},
                        }
                    }
                    Ok(Async::NotReady) => {dbg!("not ready"); return Ok(Async::NotReady)},
                    Err(e) => {dbg!(&e); return Err(ClientError::Other { source: e })},
                }
                },
        }
    }
}

pub(crate) fn construct_hyper_client() -> Client {
    /*let mut https = HttpsConnector::new(4).unwrap();
    https.https_only(true);
    let cli = HttpClient::builder().build::<_, Body>(https);
    Arc::new(cli)*/
    Arc::new(hyper::Client::new())
}
