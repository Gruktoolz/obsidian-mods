// The slots and menu of ProjectE's devices and the alchemical chest.
fn pe_block_entity_container(block: &str, container: &mut Option<horizon_client::containers::ModContainer>) {
    crate::game::pe_glue::container(block, container);
}
