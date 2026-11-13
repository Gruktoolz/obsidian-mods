fn freecam_client_level_unload(game: &crate::game::Game<'_>) {
    use crate::game::freecam_yannis as m;
    if game.native_window() {
        m::STATE.with(|s| s.borrow_mut().reset());
    }
}
