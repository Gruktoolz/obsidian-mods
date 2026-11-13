// The mod's settings page: its buttons.
fn plug_settings(game: &crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    crate::game::spider_man::settings(game, mod_id, buttons)
}
