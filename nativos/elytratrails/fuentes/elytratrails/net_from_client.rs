fn elytratrails_from_client(
    game: &mut crate::game::Game<'_>,
    player: u128,
    channel: &str,
    data: &[u8],
) {
    game.elytratrails_server_message(player, channel, data);
}
