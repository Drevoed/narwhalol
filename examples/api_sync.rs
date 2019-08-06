extern crate narwhal;

use narwhal::constants::Region;
use narwhal::dto::api::Summoner;
use narwhal::synchronous::client::LeagueAPI;

fn main() {
    let client = LeagueAPI::new(Region::KR);
    let summoner: Summoner = client.get_summoner_by_name("Hide on bush").unwrap();
}
