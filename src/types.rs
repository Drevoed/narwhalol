use crate::error::ClientError;
use futures::prelude::*;
use hyper::{Client as HttpClient, client::HttpConnector, client::connect::dns::GaiResolver, Body};
use hyper_tls::HttpsConnector;

pub(crate) type Client = HttpClient<HttpsConnector<HttpConnector<GaiResolver>>, Body>;