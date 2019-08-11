#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[cfg(feature = "asynchronous")]
extern crate hyper;
#[cfg(feature = "synchronous")]
extern crate reqwest;

#[cfg(feature = "asynchronous")]
pub mod asynchronous;
pub mod constants;
#[cfg_attr(tarpaulin, skip)]
pub mod dto;
pub mod error;
#[cfg(feature = "synchronous")]
pub mod synchronous;
pub mod types;
pub mod utils;

use constants::{LanguageCode, Region};
use std::sync::Mutex;
use synchronous::{client::LeagueAPI, ddragon::DDragonClient};

#[cfg(test)]
lazy_static! {
    pub(crate) static ref DDRAGON_CLIENT: Mutex<DDragonClient> =
        Mutex::new(DDragonClient::new(LanguageCode::UNITED_STATES).unwrap());
    pub(crate) static ref LEAGUE_CLIENT: LeagueAPI =
        LeagueAPI::new(Region::NA).expect("Please provide API_KEY environment variable");
}
