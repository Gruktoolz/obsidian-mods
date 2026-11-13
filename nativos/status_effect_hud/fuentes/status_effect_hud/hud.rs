// Hook helper: native_hud
fn status_effect_hud_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    crate::game::mod_status_effect_hud_glue::on_hud(game, batch, corners);
}
