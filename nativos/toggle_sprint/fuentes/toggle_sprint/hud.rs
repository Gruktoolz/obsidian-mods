// Hook helper: native_hud
fn toggle_sprint_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    crate::game::mod_toggle_sprint_glue::on_hud(game, batch, corners);
}
