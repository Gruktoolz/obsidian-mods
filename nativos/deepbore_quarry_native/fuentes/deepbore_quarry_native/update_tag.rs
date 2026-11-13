fn deepbore_quarry_update_tag(
    pos: [i32; 3],
    entity: &horizon_client::containers::ModBlockEntity,
    tag: &mut Option<horizon_world::nbt::Compound>,
) {
    crate::game::deepbore_quarry_glue::update_tag(pos, entity, tag);
}
