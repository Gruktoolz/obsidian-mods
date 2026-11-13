fn hook(game: &crate::game::Game<'_>, batch: &mut HudBatch, hidden: bool) {
    if !hidden && game.title.is_none() {
        game.sc_draw_alert(batch);
    }
}
