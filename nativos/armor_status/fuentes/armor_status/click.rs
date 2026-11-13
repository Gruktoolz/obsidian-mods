// Handle button clicks in Options > Mods > Armor Status.
fn armor_status_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    crate::game::mod_armor_status_glue::on_click(game, mod_id, button);
}
