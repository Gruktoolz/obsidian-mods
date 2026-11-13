// Chop! (c) 2026 Mayonesu0. All rights reserved, except the grants in LICENSE: https://github.com/Mayonesu0/chop/blob/main/LICENSE

// The end of a player's tick: a player who stopped digging lets their log be.
fn chop_player_tick(game: &mut crate::game::Game<'_>) {
    crate::game::chop_server::player_tick(game);
}
