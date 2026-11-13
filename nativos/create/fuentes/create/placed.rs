fn create_placed(game: &mut crate::game::Game<'_>, stack: &crate::items::ItemStack, pos: [i32; 3]) {
    crate::game::create_port::placed(game, stack, pos);
    if game.native_on("create") && crate::game::create_port::node(game, pos).is_some() {
        crate::game::create_port_particles::queue_rotation(game, pos);
    }
}
