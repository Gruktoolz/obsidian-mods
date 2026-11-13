// Chop! (c) 2026 Mayonesu0. All rights reserved, except the grants in LICENSE: https://github.com/Mayonesu0/chop/blob/main/LICENSE

// The server stops: what still falls drops at once.
fn chop_server_stopping(game: &mut crate::game::Game<'_>) {
    crate::game::chop_server::stopping(game);
}
