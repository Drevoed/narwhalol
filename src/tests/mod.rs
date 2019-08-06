pub mod api;
pub mod ddragon;

use crate::constants::RankedQueue;

#[test]
fn ranked_queue_properly_converts_to_str_ref() {
    let five_x_fixe = RankedQueue::SOLO;
    assert_eq!(&five_x_fixe, "RANKED_SOLO_5x5")
}
