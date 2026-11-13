// A click on the settings page's button: the next preset, and keep it.
fn fly_speed_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::fly_speed as m;
    if mod_id != m::ID || button != 0 {
        return;
    }
    let current = m::multiplier(|| game.native_setting(m::ID, "multiplier"));
    let next = m::next_preset(current);
    m::set_multiplier(next);
    game.native_set_setting(m::ID, "multiplier", &m::stored(next));
}
