fn freecam_turn(game: &crate::game::Game<'_>, xo: f64, yo: f64) -> core::ops::ControlFlow<()> {
    use crate::game::freecam_yannis as m;

    if game.native_on(m::ID) && game.native_playing() && m::detached() {
        m::STATE.with(|s| s.borrow_mut().turn(xo, yo));
        return core::ops::ControlFlow::Break(());
    }
    core::ops::ControlFlow::Continue(())
}
