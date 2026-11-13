// native_hooks::Game::native_settings (a hook point): the settings page.
fn enchantmenthints_page(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    if mod_id == crate::game::enchantmenthints::ID {
        buttons.extend(game.enchantmenthints_settings());
    }
}
