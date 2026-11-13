// Hook helper: native_settings_click
fn light_overlay_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    crate::game::mod_light_overlay_glue::on_click(game, mod_id, button);
}
