// Chop! (c) 2026 Mayonesu0. All rights reserved, except the grants in LICENSE: https://github.com/Mayonesu0/chop/blob/main/LICENSE

// A world opened or closed: its Chop! data loads or is saved.
fn chop_world_lifecycle(
    game: &mut crate::game::Game<'_>,
    phase: crate::game::native_lifecycle::WorldPhase,
) {
    crate::game::chop_server::lifecycle(game, phase);
}
