fn deepbore_quarry_submit(
    game: &crate::game::Game<'_>,
    pos: [i32; 3],
    entity: &horizon_client::containers::ModBlockEntity,
    draw: &mut crate::game::native_block_entity_draw::BlockEntityDraw,
) {
    crate::game::deepbore_quarry_glue::submit(game, pos, entity, draw);
}
