// Once a tick: the local player's flight speed is the chosen multiplier of the
// game's own. Only the window's game does it (in single player the server
// thread ticks too, and the client decides how fast it moves).
fn fly_speed_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::fly_speed as m;
    if !game.native_on(m::ID) || !game.native_window() {
        return;
    }
    let multiplier = m::multiplier(|| game.native_setting(m::ID, "multiplier"));
    let Some(player) = game.player.as_mut() else {
        return;
    };
    // Spectators keep the game's own speed: the scroll wheel adjusts it.
    if !player.flying || player.spectator {
        return;
    }
    let target = m::flying_speed(multiplier);
    if (player.flying_speed - target).abs() > 1e-6 {
        player.flying_speed = target;
    }
}
