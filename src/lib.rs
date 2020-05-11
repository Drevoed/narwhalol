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

#[cfg(test)]
mod tests {
    use crate::{Summoner, LeagueClient, Region};
    use std::time::Duration;

    #[test]
    #[cfg(feature = "async_std_rt")]
    fn ensure_different_runtimes_work_with_lib() {
        pretty_env_logger::try_init().unwrap_or(());
        let lapi = LeagueClient::new(Region::RU).unwrap();
        let sum = async_std::task::block_on(async {
            lapi.get_summoner_by_name("Vetro").await.unwrap()
        });

        log::debug!("summoner from async_std: {:?}", sum);
        assert_eq!("Vetro", &sum.name);

        std::thread::sleep(Duration::from_secs(10));
    }

    #[test]
    #[cfg(feature = "tokio_rt")]
    fn ensure_different_runtimes_work_with_lib() {
        pretty_env_logger::try_init().unwrap_or(());
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        println!("in tokio rt");

        let sum = rt.block_on(async {
            let sum = get_summoner_vetro().await;
            sum
        });

        println!("summoner from tokio: {:?}", sum);
        assert_eq!("Vetro", &sum.name);

        std::thread::sleep(Duration::from_secs(10));
    }

    #[test]
    #[cfg(feature = "smol_rt")]
    fn ensure_different_runtimes_work_with_lib() {
        pretty_env_logger::try_init().unwrap_or(());
        let sum = smol::run(async {
            let sum = get_summoner_vetro().await;
            sum
        });

        log::debug!("summoner from smol_rt: {:?}", sum);
        assert_eq!("Vetro", &sum.name);

        std::thread::sleep(Duration::from_secs(10));
    }
}
