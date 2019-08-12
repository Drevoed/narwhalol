extern crate narwhalol;

use narwhalol::client::LeagueClient;
use narwhalol::constants::Region;
use narwhalol::dto::api::Summoner;

fn main() {
    let client =
        LeagueClient::new(Region::KR).expect("Please provide API_KEY environment variable");
    let _summoner: Summoner = client.get_summoner_by_name("Hide on bush").unwrap();
}
