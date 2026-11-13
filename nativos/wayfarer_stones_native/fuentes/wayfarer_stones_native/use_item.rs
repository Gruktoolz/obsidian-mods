// A player uses an item: a wayfarer's scroll opens the menu.
fn wayfarer_stones_use_item(
    game: &mut crate::game::Game<'_>,
    stack: &horizon_client::items::ItemStack,
    claimed: &mut bool,
) {
    crate::game::wayfarer_stones_glue::use_item(game, stack, claimed);
}
