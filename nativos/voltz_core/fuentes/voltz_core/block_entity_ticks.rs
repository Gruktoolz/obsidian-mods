// Voltz machines tick.
fn voltz_block_entity_ticks(block: &str, ticks: &mut bool) {
    crate::game::voltz_glue::ticks(block, ticks);
}
