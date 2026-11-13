// Once a tick: the chunks that arrived, the seed the server sent, what the cracker has to say, the outlines.
fn scx_tick(game: &mut crate::game::Game<'_>) {
    if game.native_on(crate::game::scx_game::ID) {
        crate::game::scx_game::tick(game);
    }
}
