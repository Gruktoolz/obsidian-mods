// ProjectE's devices tick.
fn pe_block_entity_ticks(block: &str, ticks: &mut bool) {
    crate::game::pe_glue::ticks(block, ticks);
}
