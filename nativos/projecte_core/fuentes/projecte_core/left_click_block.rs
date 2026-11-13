// The start of a dig: the matter tools remember the face for their wider digs.
fn pe_left_click_block(game: &mut crate::game::Game<'_>, face: [i32; 3]) {
    crate::game::pe_glue::left_click_block(game, face);
}
