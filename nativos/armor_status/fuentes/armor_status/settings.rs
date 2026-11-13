// Armor Status settings page buttons in Options > Mods.
fn armor_status_settings(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    crate::game::mod_armor_status_glue::on_settings(game, mod_id, buttons);
}
