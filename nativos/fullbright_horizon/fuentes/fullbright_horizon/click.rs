// Handle a button click on Fullbright's settings page in Options > Mods.
fn fullbright_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    crate::game::mod_fullbright_glue::on_click(game, mod_id, button);
}
