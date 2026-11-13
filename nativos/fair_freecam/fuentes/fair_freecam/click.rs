// No attacking, mining or using while the camera flies free: the player
// would act where it looks, not where the camera is. Releases still go
// through, so a click held from before ends.
fn freecam_click(pressed: bool) -> core::ops::ControlFlow<Result<()>> {
    if pressed && crate::game::fair_freecam::active() {
        core::ops::ControlFlow::Break(Ok(()))
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
