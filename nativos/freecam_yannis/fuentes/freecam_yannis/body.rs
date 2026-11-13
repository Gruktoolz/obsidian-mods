fn freecam_body(game: &crate::game::Game<'_>) -> core::ops::ControlFlow<bool> {
    use crate::game::freecam_yannis as m;

    if game.native_on(m::ID) && m::active() {
        return core::ops::ControlFlow::Break(true);
    }
    core::ops::ControlFlow::Continue(())
}
