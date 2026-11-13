fn rider_hud_click(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    button: usize,
) {
    use crate::game::rider_hud as m;
    if mod_id == m::ID {
        match button {
            0 => {
                let current = m::is_enabled(|| game.native_setting(m::ID, "enabled"));
                let next = !current;
                m::set_enabled(next);
                game.native_set_setting(m::ID, "enabled", if next { "true" } else { "false" });
            }
            1 => {
                let current = m::is_show_hunger(|| game.native_setting(m::ID, "hunger"));
                let next = !current;
                m::set_show_hunger(next);
                game.native_set_setting(m::ID, "hunger", if next { "true" } else { "false" });
            }
            2 => {
                let current = m::is_show_exp(|| game.native_setting(m::ID, "exp"));
                let next = !current;
                m::set_show_exp(next);
                game.native_set_setting(m::ID, "exp", if next { "true" } else { "false" });
            }
            _ => {}
        }
    }
}

