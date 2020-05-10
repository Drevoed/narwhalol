use crate::error::{ClientError, HyperError};
use crate::types::{Cache, Client, SmolExecutor, SmolConnector};
use futures::prelude::*;
use hyper::header::HeaderValue;
use hyper::{Body, Client as HttpClient, Request, Response, Uri};
use log::debug;

use serde::de::DeserializeOwned;

use async_trait::async_trait;

use std::fmt::Debug;
use std::sync::Arc;
use snafu::ResultExt;
use crate::error::*;

#[async_trait]
pub(crate) trait CachedClient {
    async fn cached_resp<T: Debug + DeserializeOwned + Send>(&self, url: Uri) -> Result<T, ClientError>;
}

pub(crate) async fn get_latest_ddragon_version(client: Client) -> Result<String, ClientError> {
    let resp = client.get(
        "https://ddragon.leagueoflegends.com/api/versions.json"
            .parse()
            .unwrap(),
    )
        .await
        .context(HyperError)?;
    let body = resp
        .into_body()
        .try_fold(Vec::new(), |mut body, chunk| async move {
            body.extend_from_slice(&chunk);
            Ok(body)
        })
        .await
        .context(HyperError)?;
    let string_resp = String::from_utf8(body).context(FromUTF8Error)?;
    let mut versions: Vec<String> = serde_json::from_str(&string_resp).unwrap();
    let version = versions.remove(0);
    Ok(version)
}

/// Helper function that constructs an https hyper client
pub(crate) fn construct_hyper_client() -> Client {
    let cli = HttpClient::builder()
        .executor(SmolExecutor)
        .build::<_, Body>(SmolConnector);
    Arc::new(cli)
}
