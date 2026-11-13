fn deepbore_quarry_block_entity_tick(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
    entity: &mut horizon_client::containers::ModBlockEntity,
) {
    crate::game::deepbore_quarry_glue::tick(game, pos, entity);
}
