// A Voltz machine block gets its block entity.
fn voltz_block_entity(block: &str, entity: &mut Option<(String, horizon_world::nbt::Compound)>) {
    crate::game::voltz_glue::block_entity(block, entity);
}
