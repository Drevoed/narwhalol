//#![deny(warnings, missing_docs, missing_debug_implementations)]
#![allow(unused_mut, unused_imports, unused_macros)]
//! Fast and easy to use wrapper for League of Legends REST API and DDragon static data API.
//!
//! Narwhalol bundles both Riot League of Legends and DDragon wrapper clients in itself.
extern crate hyper;
pub mod api;
#[cfg_attr(tarpaulin, skip)]
pub mod constants;
pub mod ddragon;
#[cfg_attr(tarpaulin, skip)]
#[allow(missing_docs)]
pub mod dto;
#[allow(missing_docs)]
pub mod error;
pub(crate) mod types;
pub(crate) mod utils;

pub use {
    api::LeagueClient,
    constants::{LanguageCode, RankedQueue, Region},
    dto::api::*,
    dto::ddragon::*,
};
