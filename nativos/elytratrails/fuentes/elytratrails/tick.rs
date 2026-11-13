fn elytratrails_tick(game: &mut crate::game::Game<'_>) {
    game.elytratrails_step();
    game.elytratrails_net_tick();
}
