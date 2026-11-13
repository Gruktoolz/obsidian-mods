// ProjectE's devices and the alchemical chest get their block entity.
fn pe_block_entity(block: &str, entity: &mut Option<(String, horizon_world::nbt::Compound)>) {
    crate::game::pe_glue::block_entity(block, entity);
}
