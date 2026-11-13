// Chop! (c) 2026 Mayonesu0. All rights reserved, except the grants in LICENSE: https://github.com/Mayonesu0/chop/blob/main/LICENSE

// Before a level's tick, with that level in place: its trees move, its cuts are looked at.
fn chop_level_tick_pre(game: &mut crate::game::Game<'_>, dimension: &str) {
    crate::game::chop_server::level_tick(game, dimension);
}
