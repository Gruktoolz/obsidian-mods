// Draws the mod's screen.
fn wayfarer_stones_screen_render(
    game: &crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    screen: &horizon_client::mod_api::NativeScreenView<'_>,
) {
    crate::game::wayfarer_stones_glue::render(game, batch, screen);
}
