// Snapshot the server guest's actual sneak input at the state-swap boundary.
// This also covers input and an interaction received in the same server tick.
use std::{collections::BTreeMap, sync::{Mutex, OnceLock}};
fn states() -> &'static Mutex<BTreeMap<u128, bool>> {
    static STATES: OnceLock<Mutex<BTreeMap<u128, bool>>> = OnceLock::new();
    STATES.get_or_init(|| Mutex::new(BTreeMap::new()))
}
pub(super) fn record(player: u128, sneaking: bool) {
    let mut states = states().lock().unwrap_or_else(|e| e.into_inner());
    if states.len() > 4096 { states.clear(); }
    states.insert(player, sneaking);
}
pub(super) fn is_sneaking(player: Option<u128>) -> bool {
    player.and_then(|id| states().lock().unwrap_or_else(|e| e.into_inner()).get(&id).copied()).unwrap_or(false)
}
