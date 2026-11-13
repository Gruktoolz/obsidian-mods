fn movement_status_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::mod_movement_status as m;
    if mod_id != m::ID || button != 0 { return; }
    let now = m::background(|| game.native_setting(m::ID, "background"));
    let next = !now;
    m::set_background(next);
    game.native_set_setting(m::ID, "background", if next { "true" } else { "false" });
}
