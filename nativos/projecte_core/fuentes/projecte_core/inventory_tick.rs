// Each carried stack's tick: keys, charge, modes, active rings and stones, gem armor.
fn pe_inventory_tick(game: &mut crate::game::Game<'_>, slot: i32, selected: bool) {
    crate::game::pe_glue::inventory_tick(game, slot, selected);
}
