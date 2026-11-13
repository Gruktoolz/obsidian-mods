// Once a frame: every player's springs.
fn livingmotion_frame(game: &mut crate::game::Game<'_>) {
    crate::game::livingmotion::frame(game);
}
