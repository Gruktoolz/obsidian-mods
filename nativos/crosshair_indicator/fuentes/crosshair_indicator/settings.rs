// Hook helper: native_settings
fn crosshair_indicator_settings(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    crate::game::mod_crosshair_indicator_glue::on_settings(game, mod_id, buttons);
}
