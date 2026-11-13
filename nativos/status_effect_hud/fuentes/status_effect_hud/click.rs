// Hook helper: native_settings_click
fn status_effect_hud_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    crate::game::mod_status_effect_hud_glue::on_click(game, mod_id, button);
}
