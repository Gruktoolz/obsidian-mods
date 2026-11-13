// The block states are known here: note which of them stay visible.
fn xtoxray_tables(facts: &BlockFacts) {
    xtoxray::set_states(facts.states.iter().map(|s| &*s.name));
}
