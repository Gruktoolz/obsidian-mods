fn rarity_borders_settings(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    use crate::game::rarity_borders as m;
    if mod_id == m::ID {
        let is_ru = game.settings.language.starts_with("ru");
        let enabled = m::is_enabled(|| game.native_setting(m::ID, "enabled").map(|v| v != "false"));
        let common =
            m::is_show_common(|| game.native_setting(m::ID, "common").map(|v| v == "true"));
        let hotbar =
            m::is_show_hotbar(|| game.native_setting(m::ID, "hotbar").map(|v| v == "true"));

        buttons.push(m::label_master(enabled, is_ru));
        buttons.push(m::label_common(common, is_ru));
        buttons.push(m::label_hotbar(hotbar, is_ru));
    }
}
