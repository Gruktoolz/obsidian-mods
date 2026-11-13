// ArmorStatusHUD renderer: draws equipped armor pieces and held items.
fn armor_status_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    crate::game::mod_armor_status_glue::on_hud(game, batch, corners);
}
