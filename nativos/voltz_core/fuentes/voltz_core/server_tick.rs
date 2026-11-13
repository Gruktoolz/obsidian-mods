// The end of each server tick: run what Voltz explosives queued.
fn voltz_server_tick(
    game: &mut crate::game::Game<'_>,
    phase: crate::game::native_lifecycle::ServerTickPhase,
) {
    if matches!(phase, crate::game::native_lifecycle::ServerTickPhase::ServerTickEnd { .. }) {
        crate::game::voltz_glue::server_tick_end(game);
    }
}
