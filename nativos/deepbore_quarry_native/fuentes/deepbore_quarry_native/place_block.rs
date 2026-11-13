fn deepbore_quarry_place_block(
    game: &mut crate::game::Game<'_>,
    stack: &horizon_client::items::ItemStack,
    pos: [i32; 3],
) {
    crate::game::deepbore_quarry_glue::placed(game, stack, pos);
}
