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

pub mod types;
pub mod ddragon;
pub mod http;
pub mod dto;
pub mod constants;

#[cfg(test)]
mod tests;
