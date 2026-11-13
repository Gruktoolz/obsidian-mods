// From the free camera the player's own body is seen, as in third person.
fn freecam_body() -> core::ops::ControlFlow<bool> {
    if crate::game::fair_freecam::active() {
        core::ops::ControlFlow::Break(true)
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
