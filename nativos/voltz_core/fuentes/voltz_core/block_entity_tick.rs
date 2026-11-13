// One server tick of a Voltz machine.
fn voltz_block_entity_tick(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
    entity: &mut horizon_client::containers::ModBlockEntity,
) {
    crate::game::voltz_glue::tick(game, pos, entity);
}
