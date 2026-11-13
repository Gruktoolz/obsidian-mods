// No first-person hand while the camera flies free.
fn freecam_hand() -> core::ops::ControlFlow<Vec<Vertex>> {
    if crate::game::fair_freecam::active() {
        core::ops::ControlFlow::Break(Vec::new())
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
