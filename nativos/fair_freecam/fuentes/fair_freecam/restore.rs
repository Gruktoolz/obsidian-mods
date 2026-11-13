// After the player's tick: the movement keys taken for it are held again.
fn freecam_restore(game: &mut crate::game::Game<'_>) {
    use winit::keyboard::KeyCode as K;
    const MOVE: [K; 7] = [
        K::KeyW,
        K::KeyS,
        K::KeyA,
        K::KeyD,
        K::Space,
        K::ShiftLeft,
        K::ControlLeft,
    ];
    // The server thread's own game ticks too; the camera is the window's.
    if !game.native_window() {
        return;
    }
    let taken = crate::game::fair_freecam::take_stash();
    for (i, k) in MOVE.iter().enumerate() {
        if taken & (1 << i) != 0 {
            game.keys.insert(*k);
        }
    }
}
