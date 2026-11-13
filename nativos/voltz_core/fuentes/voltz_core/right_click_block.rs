// A right click on a block: ME terminals open their menu.
fn voltz_right_click_block(game: &mut crate::game::Game<'_>, pos: [i32; 3], canceled: &mut bool) {
    crate::game::voltz_glue::right_click_block(game, pos, canceled);
}
