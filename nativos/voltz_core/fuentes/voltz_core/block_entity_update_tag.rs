// What clients get of a Voltz machine.
fn voltz_block_entity_update_tag(
    entity: &horizon_client::containers::ModBlockEntity,
    tag: &mut Option<horizon_world::nbt::Compound>,
) {
    crate::game::voltz_glue::update_tag(entity, tag);
}
