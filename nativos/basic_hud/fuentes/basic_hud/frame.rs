// Dataframe hook: updates telemetry and smoothed horizontal BPS each frame.
fn basic_hud_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    crate::game::basic_hud_data::frame(game, elapsed);
    // Drop transient editor state when death, disconnect or another screen takes over.
    if game.native_window() && crate::game::basic_hud_editor::active()
        && (game.screen != crate::game::Screen::Options || !game.native_on(crate::game::basic_hud::ID))
    {
        crate::game::basic_hud_editor::close(game);
    }
}
