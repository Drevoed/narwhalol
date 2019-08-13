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
pub mod client;
#[cfg_attr(tarpaulin, skip)]
pub mod constants;
mod ddragon;
#[cfg_attr(tarpaulin, skip)]
pub mod dto;
pub mod error;
#[cfg(feature = "synchronous")]
pub mod synchronous;
pub mod types;
pub mod utils;

use constants::{LanguageCode, Region};
use std::sync::Mutex;
use {client::LeagueClient, ddragon::DDragonClient};

