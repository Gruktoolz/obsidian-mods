// Before the player's tick: while the camera flies free, the movement keys
// move the camera, not the player. They are taken out of the held keys for
// the tick (the player stands still and the server hears no input) and
// given back right after it, in `native_tick`.
fn freecam_tick(game: &mut crate::game::Game<'_>) {
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
    if !crate::game::fair_freecam::active() {
        return;
    }
    let mut taken = 0u8;
    for (i, k) in MOVE.iter().enumerate() {
        if game.keys.remove(k) {
            taken |= 1 << i;
        }
    }
    crate::game::fair_freecam::stash(taken);
}
