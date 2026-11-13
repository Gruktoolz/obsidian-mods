// The server tick's phases: travels and saves at its end.
fn wayfarer_stones_server_tick(
    game: &mut crate::game::Game<'_>,
    context: &horizon_client::mod_api::NativeLifecycleContext,
    phase: horizon_client::mod_api::ServerTickPhase,
) {
    crate::game::wayfarer_stones_glue::tick(game, context, phase);
}
