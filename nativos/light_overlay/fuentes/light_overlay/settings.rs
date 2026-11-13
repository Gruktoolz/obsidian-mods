// Hook helper: native_settings
fn light_overlay_settings(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    crate::game::mod_light_overlay_glue::on_settings(game, mod_id, buttons);
}
