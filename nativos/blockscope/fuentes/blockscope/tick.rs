// The server thread's tick: the container the window asked about.
fn blockscope_tick(game: &mut crate::game::Game<'_>) {
    crate::game::native_blockscope_glue::tick(game);
}
