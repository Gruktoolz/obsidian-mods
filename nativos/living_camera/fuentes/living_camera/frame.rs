// Each frame: the springs move toward what the movement and turning ask.
fn living_camera_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    use crate::game::living_camera as m;
    if !game.native_window() || !game.native_on(m::ID) {
        return;
    }
    {
        let game = &*game;
        m::settings(|key| game.native_setting(m::ID, key));
    }
    let view = game
        .native_player()
        .filter(|_| game.native_playing())
        .map(|p| [p.yaw, p.pitch]);
    m::frame(elapsed.as_secs_f32(), view);
}
