// Hook helper: native_settings_click
fn crosshair_indicator_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    crate::game::mod_crosshair_indicator_glue::on_click(game, mod_id, button);
}
