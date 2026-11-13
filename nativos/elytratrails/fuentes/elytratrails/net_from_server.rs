fn elytratrails_from_server(game: &mut crate::game::Game<'_>, channel: &str, data: &[u8]) {
    game.elytratrails_client_message(channel, data);
}
