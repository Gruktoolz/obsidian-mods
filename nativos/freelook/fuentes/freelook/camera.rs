// While the key is held, the view is worked out from the camera's angles
// (third person included) instead of the player's, which are put back right
// after. The eye and the camera
// behind the player are worked out by the game, and the camera is put on that
// line at the wheel's distance (a wall behind the player still pulls it in).
// Cancels the original; the nested calls run it unchanged.
fn freelook_camera(
    game: &mut crate::game::Game<'_>,
    partial: f32,
) -> std::ops::ControlFlow<Option<(glam::DVec3, f32, f32)>> {
    use crate::game::freelook as m;
    use crate::game::perspective::CameraType;
    if !game.native_on(m::ID) || !game.native_window() || !m::begin() {
        return std::ops::ControlFlow::Continue(());
    }
    let (yaw, pitch) = (game.yaw, game.pitch);
    let kind = game.camera_type;
    let (look_yaw, look_pitch) = m::look(yaw, pitch);
    game.yaw = look_yaw;
    game.pitch = look_pitch;
    let mut view = game.camera_view(partial);
    if let Some(wanted) = m::distance() {
        game.camera_type = CameraType::FirstPerson;
        let eye = game.camera_view(partial);
        game.camera_type = CameraType::ThirdPersonBack;
        let back = game.camera_view(partial);
        if let (Some((eye, _, _)), Some((back, y, p))) = (eye, back) {
            let line = back - eye;
            let clipped = line.length() as f32;
            if clipped > 1e-4 {
                let far = m::length(wanted, clipped);
                view = Some((eye + line / f64::from(clipped) * f64::from(far), y, p));
            }
        }
    }
    game.camera_type = kind;
    game.yaw = yaw;
    game.pitch = pitch;
    m::end();
    std::ops::ControlFlow::Break(view)
}
