// Records where the layer's vertices start, then hides vanilla's effect icons
// when HUD+ draws them itself.
fn official_hud_plus_layer_pre(
    game: &crate::game::Game<'_>,
    layer: &str,
    batch: &mut horizon_client::hud::HudBatch,
    canceled: &mut bool,
) {
    crate::game::official_hud_plus_glue::layer_pre(game, layer, batch, canceled);
}
