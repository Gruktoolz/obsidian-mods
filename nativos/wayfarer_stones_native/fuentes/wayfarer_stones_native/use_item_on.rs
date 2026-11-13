// A right click with an item on a block, before any placement: a stone
// past the owner's limit is refused.
fn wayfarer_stones_use_item_on(
    game: &mut crate::game::Game<'_>,
    stack: &horizon_client::items::ItemStack,
    claimed: &mut bool,
) {
    crate::game::wayfarer_stones_glue::use_item_on(game, stack, claimed);
}
