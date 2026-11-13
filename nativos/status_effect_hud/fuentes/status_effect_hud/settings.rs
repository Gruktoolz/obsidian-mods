// Hook helper: native_settings
fn status_effect_hud_settings(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    crate::game::mod_status_effect_hud_glue::on_settings(game, mod_id, buttons);
}
