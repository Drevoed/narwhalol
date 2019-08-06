use crate::constants::LanguageCode;
use crate::dto::ddragon::AllChampions;
use crate::synchronous::ddragon::DDragonClient;
use std::sync::Mutex;
use std::time::Instant;

lazy_static! {
    static ref DDRAGON_CLIENT: Mutex<DDragonClient> =
        Mutex::new(DDragonClient::new(LanguageCode::RUSSIA).unwrap());
}

#[test]
fn ddragon_caches_properly() {
    let mut client = DDRAGON_CLIENT.lock().unwrap();
    let champs = client.get_champions().unwrap();
    drop(champs);
    let now = Instant::now();
    let champs: AllChampions = client.get_champions().unwrap();
    assert!(now.elapsed().as_millis() < 100);
    assert_eq!("103", &champs.data.get("Ahri").unwrap().key);
}
