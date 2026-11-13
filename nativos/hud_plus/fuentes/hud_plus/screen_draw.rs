// The settings and layout screens, and the inventory beside a container.
fn official_hud_plus_screen_draw(
    game: &crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    screen: &horizon_client::mod_api::NativeScreenView<'_>,
) {
    crate::game::official_hud_plus_glue::draw_screen(game, batch, screen);
}
