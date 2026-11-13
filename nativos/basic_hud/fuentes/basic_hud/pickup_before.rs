fn basic_hud_pickup_before(game: &crate::game::Game<'_>, stack: &horizon_client::items::ItemStack) {
    crate::game::basic_hud_pickup_data::before_local(game, stack);
}
