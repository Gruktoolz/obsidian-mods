fn rarity_borders_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::rarity_borders as m;
    if mod_id != m::ID {
        return;
    }
    match button {
        0 => {
            let next =
                !m::is_enabled(|| game.native_setting(m::ID, "enabled").map(|v| v != "false"));
            m::set_enabled(next);
            game.native_set_setting(m::ID, "enabled", if next { "true" } else { "false" });
        }
        1 => {
            let next =
                !m::is_show_common(|| game.native_setting(m::ID, "common").map(|v| v == "true"));
            m::set_show_common(next);
            game.native_set_setting(m::ID, "common", if next { "true" } else { "false" });
        }
        2 => {
            let next =
                !m::is_show_hotbar(|| game.native_setting(m::ID, "hotbar").map(|v| v == "true"));
            m::set_show_hotbar(next);
            game.native_set_setting(m::ID, "hotbar", if next { "true" } else { "false" });
        }
        _ => {}
    }
}
