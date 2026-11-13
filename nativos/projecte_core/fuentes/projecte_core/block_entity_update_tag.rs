// What clients get of a ProjectE device: its EMC.
fn pe_block_entity_update_tag(
    entity: &horizon_client::containers::ModBlockEntity,
    tag: &mut Option<horizon_world::nbt::Compound>,
) {
    crate::game::pe_glue::update_tag(entity, tag);
}
