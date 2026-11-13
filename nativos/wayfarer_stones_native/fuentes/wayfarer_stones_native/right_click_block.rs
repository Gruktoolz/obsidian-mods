// A player right clicks a block: a stone activates and opens its menu.
fn wayfarer_stones_right_click_block(game: &mut crate::game::Game<'_>, pos: [i32; 3], canceled: &mut bool) {
    crate::game::wayfarer_stones_glue::right_click(game, pos, canceled);
}
