// Populate Fullbright's settings buttons in Options > Mods > Fullbright.
fn fullbright_settings(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    crate::game::mod_fullbright_glue::on_settings(game, mod_id, buttons);
}
