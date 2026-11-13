// AppleSkin settings page buttons in Options > Mods.
fn appleskin_settings(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    crate::game::mod_appleskin_glue::on_settings(game, mod_id, buttons);
}
