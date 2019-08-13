

use hyper::{
    client::connect::dns::GaiResolver, client::HttpConnector, Body, Client as HttpClient, Uri,
};
use hyper_tls::HttpsConnector;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub(crate) type Client = Arc<HttpClient<HttpsConnector<HttpConnector<GaiResolver>>, Body>>;
pub(crate) type Cache<K = Uri, V = String> = Arc<Mutex<HashMap<K, V>>>;
