// Toggle Fullbright when the configured key (G by default) is pressed while playing.
fn fullbright_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    crate::game::mod_fullbright_glue::on_key(game, key, pressed);
}
