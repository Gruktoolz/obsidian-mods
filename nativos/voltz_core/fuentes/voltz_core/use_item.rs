// A right click with an item in the air: guns.
fn voltz_use_item(game: &mut crate::game::Game<'_>, stack: &horizon_client::items::ItemStack, claimed: &mut bool) {
    let item = stack.item.to_string();
    crate::game::voltz_glue::use_item(game, &item, claimed);
}
