// Handle button clicks in Options > Mods > AppleSkin.
fn appleskin_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    crate::game::mod_appleskin_glue::on_click(game, mod_id, button);
}
