fn flair_draw(game: &crate::game::Game<'_>, batch: &mut horizon_client::hud::HudBatch) {
    crate::game::umbx_flair_screen::draw(game, batch);
}
