extern crate narwhalol;

use narwhalol::constants::Region;
use narwhalol::dto::api::Summoner;
use narwhalol::synchronous::client::LeagueAPI;

fn main() -> Result<(), failure::Error> {
    let client = LeagueAPI::new(Region::KR).expect("Please provide API_KEY environment variable");
    let summoner: Summoner = client.get_summoner_by_name("Hide on bush").unwrap();
    Ok(())
}
