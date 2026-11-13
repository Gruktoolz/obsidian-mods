fn freecam_interaction_key(
    game: &crate::game::Game<'_>,
    swing_hand: &mut bool,
    canceled: &mut bool,
) {
    use crate::game::freecam_yannis as m;

    if game.native_on(m::ID) && m::detached() {
        *canceled = true;
        *swing_hand = false;
    }
}
