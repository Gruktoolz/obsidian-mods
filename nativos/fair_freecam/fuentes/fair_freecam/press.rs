// A movement key pressed while the camera flies free doesn't reach the
// player (no jump, no double-tap to fly, no sneak toggle).
fn freecam_press(key: winit::keyboard::KeyCode) -> core::ops::ControlFlow<Result<()>> {
    use winit::keyboard::KeyCode as K;
    let movement = matches!(
        key,
        K::KeyW | K::KeyS | K::KeyA | K::KeyD | K::Space | K::ShiftLeft | K::ControlLeft
    );
    if movement && crate::game::fair_freecam::active() {
        core::ops::ControlFlow::Break(Ok(()))
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
