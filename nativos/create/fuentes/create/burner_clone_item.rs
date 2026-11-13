fn create_burner_clone_item(
    game: &crate::game::Game<'_>,
    pos: [i32; 3],
    stack: &mut Option<crate::items::ItemStack>,
) {
    if stack.is_none() && game.native_on("create") {
        *stack = crate::game::create_port_burner_interactions::clone_item(game, pos)
            .map(|id| crate::items::ItemStack::new(id, 1));
    }
}
