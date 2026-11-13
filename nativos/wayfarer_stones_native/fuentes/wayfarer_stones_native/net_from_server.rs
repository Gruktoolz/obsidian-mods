// The server's list of stones: opens or refreshes the screen.
fn wayfarer_stones_net_from_server(game: &mut crate::game::Game<'_>, channel: &str, data: &[u8]) {
    crate::game::wayfarer_stones_glue::net_from_server(game, channel, data);
}
