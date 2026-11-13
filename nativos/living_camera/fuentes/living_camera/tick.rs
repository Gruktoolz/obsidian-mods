// Each tick on the window: how the player moved, and whether their eyes are
// in water.
fn living_camera_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::living_camera as m;
    if !game.native_window() || !game.native_on(m::ID) {
        return;
    }
    let Some(p) = game.native_player() else {
        m::tick(None, 0.0, false);
        return;
    };
    let eyes = [p.x.floor() as i32, (p.y + m::EYE_HEIGHT).floor() as i32, p.z.floor() as i32];
    let swimming = m::is_water(&game.native_block(eyes));
    m::tick(Some([p.x, p.y, p.z]), p.yaw, swimming);
}
