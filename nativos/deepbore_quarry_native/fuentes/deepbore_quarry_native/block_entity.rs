fn deepbore_quarry_block_entity(
    block: &str,
    entity: &mut Option<(String, horizon_world::nbt::Compound)>,
) {
    crate::game::deepbore_quarry_glue::block_entity(block, entity);
}
