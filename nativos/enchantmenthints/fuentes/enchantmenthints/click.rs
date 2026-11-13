// native_hooks::Game::native_settings_click (a hook point): a click on the
// settings page changes the setting, keeps it and applies it.
fn enchantmenthints_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    if mod_id == crate::game::enchantmenthints::ID {
        game.enchantmenthints_settings_click(button);
    }
}
