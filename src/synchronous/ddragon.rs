use crate::dto::ddragon::DDragonResponse;
use reqwest::Client;
use std::collections::HashMap;

pub struct DDragonClient {
    client: Client,
    base_url: String,
    cache: HashMap<String, DDragonResponse>,
}

impl DDragonClient {}
