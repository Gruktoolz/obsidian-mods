fn rider_hud_settings(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    use crate::game::rider_hud as m;
    if mod_id == m::ID {
        let enabled = m::is_enabled(|| game.native_setting(m::ID, "enabled"));
        buttons.push(m::enabled_label(enabled));

        let hunger = m::is_show_hunger(|| game.native_setting(m::ID, "hunger"));
        buttons.push(m::hunger_label(hunger));

        let exp = m::is_show_exp(|| game.native_setting(m::ID, "exp"));
        buttons.push(m::exp_label(exp));
    }
}

