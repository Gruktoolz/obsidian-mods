fn create_use_on(
    game: &mut crate::game::Game<'_>,
    stack: &crate::items::ItemStack,
    pos: [i32; 3],
    face: [i32; 3],
    claimed: &mut bool,
) {
    crate::game::create_port::use_on(game, stack, pos, face);
    if crate::game::create_port_placement::alias(game, stack, pos, face) {
        *claimed = true;
    }
}
