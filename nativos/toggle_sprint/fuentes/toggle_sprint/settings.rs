// Hook helper: native_settings
fn toggle_sprint_settings(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    crate::game::mod_toggle_sprint_glue::on_settings(game, mod_id, buttons);
}
