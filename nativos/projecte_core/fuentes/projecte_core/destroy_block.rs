// A block broken: the matter tools break the rest of their area.
fn pe_destroy_block(game: &mut crate::game::Game<'_>, pos: [i32; 3]) {
    crate::game::pe_glue::destroy_block(game, pos);
}
