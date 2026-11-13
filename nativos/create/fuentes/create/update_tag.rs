fn create_on_update_tag(
    entity: &crate::containers::ModBlockEntity,
    tag: &mut Option<horizon_world::nbt::Compound>,
) {
    crate::game::create_port::update_tag(entity, tag);
    crate::game::create_port_fan::update_tag(entity, tag);
}
