fn freecam_block_highlight(game: &crate::game::Game<'_>, canceled: &mut bool) {
    use crate::game::freecam_yannis as m;

    if game.native_on(m::ID) && m::detached() {
        *canceled = true;
    }
}
