// A ProjectE key press from a player's client.
fn pe_net_from_client(player: u128, channel: &str, data: &[u8]) {
    crate::game::pe_glue::net_from_client(player, channel, data);
}
