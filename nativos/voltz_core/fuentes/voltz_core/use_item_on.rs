// A right click with an item on a block: the target designator.
fn voltz_use_item_on(
    game: &mut crate::game::Game<'_>,
    stack: &horizon_client::items::ItemStack,
    pos: [i32; 3],
    claimed: &mut bool,
) {
    let item = stack.item.to_string();
    crate::game::voltz_glue::use_item_on(game, &item, pos, claimed);
}
