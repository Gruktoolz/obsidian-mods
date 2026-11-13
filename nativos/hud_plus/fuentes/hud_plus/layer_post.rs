// Draws HUD+ after a vanilla layer: the tab list fires even with no tab list
// up, so it is the frame's anchor.
fn official_hud_plus_layer_post(
    game: &mut crate::game::Game<'_>,
    layer: &str,
    batch: &mut horizon_client::hud::HudBatch,
) {
    crate::game::official_hud_plus_glue::layer_post(game, layer, batch);
}
