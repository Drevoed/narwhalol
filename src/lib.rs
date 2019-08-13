#![doc(html_root_url = "https://docs.rs/tokio/0.1.22")]

//! Fast and easy to use wrapper for League of Legends REST API and DDragon static data API.
//!
//! Narwhalol bundles both Riot League of Legends and DDragon wrapper clients in itself.

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate hyper;
pub mod client;
#[cfg_attr(tarpaulin, skip)]
pub mod constants;
pub mod ddragon;
#[cfg_attr(tarpaulin, skip)]
pub mod dto;
pub mod error;
pub mod types;
pub mod utils;
