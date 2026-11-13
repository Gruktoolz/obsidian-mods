// The mod's page in Options > Mods.
fn livingmotion_settings(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    crate::game::livingmotion::settings_page(game, mod_id, buttons);
}
