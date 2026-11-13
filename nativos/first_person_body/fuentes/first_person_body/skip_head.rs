// Layers at the head (a pumpkin or skull worn, deadmau5's ears): not drawn
// while the own first-person model is, they would sit over the camera.
fn first_person_body_skip_head() -> core::ops::ControlFlow<()> {
    if horizon_client::mobs::first_person_body::active().is_some() {
        return core::ops::ControlFlow::Break(());
    }
    core::ops::ControlFlow::Continue(())
}
