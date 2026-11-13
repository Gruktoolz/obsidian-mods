// With chat open, Horizon skips the chat and tab list layers, so the HUD
// would vanish: draw it from the HUD hook instead, which still runs.
fn official_hud_plus_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
) {
    crate::game::official_hud_plus_glue::hud_fallback(game, batch);
}
