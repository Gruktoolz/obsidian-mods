// One server tick of a ProjectE device.
fn pe_block_entity_tick(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
    entity: &mut horizon_client::containers::ModBlockEntity,
) {
    crate::game::pe_glue::tick(game, pos, entity);
}
