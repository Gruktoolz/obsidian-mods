// The mouse turns the free camera, not the player.
fn freecam_turn(xo: f64, yo: f64) -> core::ops::ControlFlow<()> {
    if crate::game::fair_freecam::turn(xo, yo) {
        core::ops::ControlFlow::Break(())
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
