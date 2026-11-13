// Hook helper: native_key
fn light_overlay_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    crate::game::mod_light_overlay_glue::on_key(game, key, pressed);
}
