// Hook helper: native_frame
fn toggle_sprint_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    crate::game::mod_toggle_sprint_glue::on_frame(game, elapsed);
}
