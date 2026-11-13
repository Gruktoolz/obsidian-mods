// A player placed a block from an item: a stone registers itself.
fn wayfarer_stones_place_block(
    game: &mut crate::game::Game<'_>,
    stack: &horizon_client::items::ItemStack,
    pos: [i32; 3],
) {
    crate::game::wayfarer_stones_glue::placed(game, stack, pos);
}
