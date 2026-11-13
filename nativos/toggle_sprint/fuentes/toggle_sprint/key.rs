// Hook helper: native_key
fn toggle_sprint_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    crate::game::mod_toggle_sprint_glue::on_key(game, key, pressed);
}
