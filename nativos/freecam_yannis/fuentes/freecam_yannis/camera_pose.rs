fn freecam_camera_pose(
    game: &crate::game::Game<'_>,
    position: &mut [f64; 3],
    yaw: &mut f32,
    pitch: &mut f32,
) {
    use crate::game::freecam_yannis as m;

    if !game.native_on(m::ID) || game.native_player().is_none() {
        return;
    }
    m::STATE.with(|s| {
        let s = s.borrow();
        if let Some(p) = s.pose {
            *position = p.position;
            *yaw = p.yaw;
            *pitch = p.pitch;
        }
    });
}
