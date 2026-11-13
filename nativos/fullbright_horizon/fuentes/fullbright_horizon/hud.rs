// Draw the temporary `Fullbright: ON` / `Fullbright: OFF` HUD overlay above the hotbar.
fn fullbright_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
) {
    crate::game::mod_fullbright_glue::on_hud(game, batch);
}
