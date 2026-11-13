// The mod's settings page: one button, stepping through the presets.
fn fly_speed_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::fly_speed as m;
    if mod_id != m::ID {
        return;
    }
    let current = m::multiplier(|| game.native_setting(m::ID, "multiplier"));
    buttons.push(format!("Fly speed: {}", m::label(current)));
}
