fn create_on_entity(block: &str, entity: &mut Option<(String, horizon_world::nbt::Compound)>) {
    crate::game::create_port::entity(block, entity);
}
