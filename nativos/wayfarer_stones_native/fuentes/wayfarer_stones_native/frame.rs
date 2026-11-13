// Once a frame: what the screen's read-only hooks asked for.
fn wayfarer_stones_frame(game: &mut crate::game::Game<'_>) {
    crate::game::wayfarer_stones_glue::frame(game);
}
