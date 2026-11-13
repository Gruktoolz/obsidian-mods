// A right click on a block: the transmutation table opens its menu; devices tell their EMC.
fn pe_right_click_block(game: &mut crate::game::Game<'_>, pos: [i32; 3], canceled: &mut bool) {
    crate::game::pe_glue::right_click_block(game, pos, canceled);
}
