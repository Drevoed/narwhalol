use crate::http::sync::client::LeagueAPI;
use crate::constants::region::Region;

lazy_static!{
    static ref LEAGUE_CLIENT: LeagueAPI = LeagueAPI::new_dev_client(Region::NA);
}

#[test]
fn gets_summoner_data() {
    let summoner = LEAGUE_CLIENT.get_summoner_by_name("Santorin").expect("Something went wrong");
    println!("Got summoner: {:?}", &summoner);
    assert_eq!(&summoner.account_id, "CU8u5t6mieXsvcG5YdeVEsJictHK2is6QBO7mRJN1OaY-_c")
}
