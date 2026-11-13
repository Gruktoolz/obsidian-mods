// The blocks the mod declares (API 145): the stone.
fn wayfarer_stones_blocks(blocks: &mut Vec<horizon_mods::content::Block>) {
    crate::game::wayfarer_stones_glue::blocks(blocks);
}
