fn burner_entity(
    block: &str,
    properties: &crate::mod_api::BlockProperties,
    entity: &mut Option<(String, horizon_world::nbt::Compound)>,
) {
    crate::game::create_port_burner::entity(block, properties, entity);
}
