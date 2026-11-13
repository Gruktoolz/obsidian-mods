// An item used on a block: the philosopher's stone, divining rods, catalysts, amulets, hoes.
fn pe_use_item_on(
    game: &mut crate::game::Game<'_>,
    stack: &horizon_client::items::ItemStack,
    pos: [i32; 3],
    face: [i32; 3],
    claimed: &mut bool,
) {
    crate::game::pe_glue::use_item_on(game, stack, pos, face, claimed);
}
