// Horizon's cosmetics (hats, wings...): not on the own first-person model,
// a hat would sit over the camera.
fn first_person_body_skip_cosmetics() -> core::ops::ControlFlow<Option<(glam::Vec3, glam::Vec3)>> {
    if horizon_client::mobs::first_person_body::active().is_some() {
        return core::ops::ControlFlow::Break(None);
    }
    core::ops::ControlFlow::Continue(())
}
