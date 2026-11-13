fn freecam_render_hand(game: &crate::game::Game<'_>, canceled: &mut bool) {
    use crate::game::freecam_yannis as m;

    if game.native_on(m::ID) && m::active() {
        *canceled = true;
    }
}
