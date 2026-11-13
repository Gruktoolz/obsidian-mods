fn deepbore_quarry_destroy_block(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
    canceled: &mut bool,
) {
    crate::game::deepbore_quarry_glue::destroyed(game, pos, canceled);
}
