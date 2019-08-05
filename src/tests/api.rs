use crate::constants::region::Region;
use crate::http::sync::client::LeagueAPI;

lazy_static! {
    static ref LEAGUE_CLIENT: LeagueAPI = LeagueAPI::new_dev_client(Region::NA);
}

#[test]
fn gets_summoner_data() {
    let summoner = LEAGUE_CLIENT
        .get_summoner_by_name("Santorin")
        .expect("Something went wrong");
    println!("Got summoner: {:#?}", &summoner);
    assert_eq!(
        &summoner.account_id,
        "rPnj4h5W6OhejxB-AO3hLOQctgZcckqV_82N_8_WuCFdO2A"
    )
}
