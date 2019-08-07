#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure_derive;
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
pub mod dto;
#[cfg(feature = "synchronous")]
pub mod synchronous;
pub mod types;

#[cfg(test)]
mod tests;
