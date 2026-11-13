// While the key is held, the mouse turns the camera's yaw and pitch, not the
// player's: the game's own mouse look runs on the camera's angles and the
// player's are put back after it. Cancels the original; the nested call
// below runs it unchanged (m::begin is false then).
fn freelook_mouse(game: &mut crate::game::Game<'_>) -> std::ops::ControlFlow<()> {
    use crate::game::freelook as m;
    if !game.native_on(m::ID) || !game.native_window() || !m::begin() {
        return std::ops::ControlFlow::Continue(());
    }
    let (yaw, pitch) = (game.yaw, game.pitch);
    let (look_yaw, look_pitch) = m::look(yaw, pitch);
    game.yaw = look_yaw;
    game.pitch = look_pitch;
    game.apply_mouse_look();
    m::set_look(game.yaw, game.pitch);
    game.yaw = yaw;
    game.pitch = pitch;
    m::end();
    std::ops::ControlFlow::Break(())
}
