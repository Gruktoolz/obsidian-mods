// An item used in the air: the tablet, bags, tome, rings' on/off, lenses.
fn pe_use_item(game: &mut crate::game::Game<'_>, stack: &horizon_client::items::ItemStack, claimed: &mut bool) {
    crate::game::pe_glue::use_item(game, stack, claimed);
}
