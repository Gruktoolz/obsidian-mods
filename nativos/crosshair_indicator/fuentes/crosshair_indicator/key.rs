// Hook helper: native_key
fn crosshair_indicator_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    crate::game::mod_crosshair_indicator_glue::on_key(game, key, pressed);
}
