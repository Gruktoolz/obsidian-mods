// A player breaks a block: a stone leaves the registry, or stays for its owner.
fn wayfarer_stones_destroy_block(game: &mut crate::game::Game<'_>, pos: [i32; 3], canceled: &mut bool) {
    crate::game::wayfarer_stones_glue::destroyed(game, pos, canceled);
}
