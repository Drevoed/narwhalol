#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[cfg(feature = "async")]
extern crate hyper;
#[cfg(feature = "sync")]
extern crate reqwest;

pub mod constants;
pub mod ddragon;
pub mod dto;
pub mod http;
pub mod types;

#[cfg(test)]
mod tests;
