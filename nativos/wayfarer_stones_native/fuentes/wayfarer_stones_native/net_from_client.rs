// A request from a player's client.
fn wayfarer_stones_net_from_client(game: &mut crate::game::Game<'_>, player: u128, channel: &str, data: &[u8]) {
    crate::game::wayfarer_stones_glue::net_from_client(game, player, channel, data);
}
