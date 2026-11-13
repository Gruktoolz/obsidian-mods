// Hook helper: native_settings_click
fn toggle_sprint_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    crate::game::mod_toggle_sprint_glue::on_click(game, mod_id, button);
}
