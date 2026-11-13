fn freecam_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    use crate::game::freecam_yannis as m;

    if !game.native_window() {
        return;
    }
    if !game.native_on(m::ID) {
        m::STATE.with(|s| s.borrow_mut().reset());
        return;
    }
    let player = game.native_player();
    m::STATE.with(|s| s.borrow_mut().world(player.as_ref().map(|p| p.dimension)));
    let Some(p) = player else {
        return;
    };
    if !game.native_playing() {
        m::STATE.with(|s| {
            let mut s = s.borrow_mut();
            s.free_key = false;
        });
        return;
    }
    if !m::active() {
        return;
    }
    let mut down = |name| game.native_mapping(name).is_some_and(|(held, _)| held);
    let forward = f64::from(down("key.forward")) - f64::from(down("key.back"));
    let side = f64::from(down("key.right")) - f64::from(down("key.left"));
    let up = f64::from(down("key.jump")) - f64::from(down("key.sneak"));
    let sprint = down("key.sprint");
    m::STATE.with(|s| {
        s.borrow_mut().advance(
            m::Pose {
                position: [p.x, p.y + 1.62, p.z],
                yaw: p.yaw,
                pitch: p.pitch,
            },
            elapsed.as_secs_f64(),
            [forward, side, up],
            sprint,
        )
    });
}
