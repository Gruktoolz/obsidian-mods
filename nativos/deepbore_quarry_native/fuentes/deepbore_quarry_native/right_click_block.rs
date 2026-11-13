fn deepbore_quarry_right_click_block(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
    canceled: &mut bool,
) {
    crate::game::deepbore_quarry_glue::right_click(game, pos, canceled);
}
