// Voltz machines have a furnace's three slots and menu.
fn voltz_block_entity_container(block: &str, container: &mut Option<horizon_client::containers::ModContainer>) {
    crate::game::voltz_glue::container(block, container);
}
