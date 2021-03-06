# Narwhalol
[![Build Status](https://travis-ci.com/Drevoed/narwhalol.svg?branch=master)](https://travis-ci.com/Drevoed/narwhalol)
[![Code Coverage](https://codecov.io/gh/Drevoed/narwhalol/branch/master/graph/badge.svg)](https://codecov.io/gh/Drevoed/narwhalol)
[![License](https://img.shields.io/crates/l/narwhalol)](https://github.com/Drevoed/narwhal/blob/master/LICENSE.txt)
[![Latest Version](https://img.shields.io/crates/v/narwhalol)](https://crates.io/crates/narwhalol)
[![Documentation](https://docs.rs/narwhalol/badge.svg)](https://docs.rs/narwhalol)

***Narwhalol*** is a Fast and Type-safe wrapper of DDragon and League of
Legends API.

It strives to provide the most comfortable and fast experience of
getting useful data directly from Riot servers.

Zero cost abstractions and compile-time optimizations used in Rust is
what makes this library so fast.

## Advantages 
- Support of many std Traits allowing hands-free convertations between types
- Clean and concise Error messages
- Caching of identical requests
- Is in development stage, issues and bugs will be fixed ASAP
- Supports all mainstream runtimes with feature flags ([smol](https://github.com/stjepang/smol), [async-std](https://github.com/async-rs/async-std), [tokio](https://github.com/tokio-rs/tokio))
## Example

```rust,no_run
use narwhalol::LeagueClient;
use smol;

fn main() {
    let lapi = LeagueClient::new(Region::RU).unwrap();
    let sum = smol::run(async {
        lapi.get_summoner_by_name("Vetro").await.unwrap()
    });

    println!("got summoner: {:?}", &sum);
}
```
