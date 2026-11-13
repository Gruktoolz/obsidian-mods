// A button on the mod's page.
fn livingmotion_settings_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    crate::game::livingmotion::settings_click(game, mod_id, button);
}
