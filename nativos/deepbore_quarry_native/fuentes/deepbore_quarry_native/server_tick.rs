fn deepbore_quarry_server_tick(
    game: &mut crate::game::Game<'_>,
    phase: horizon_client::mod_api::ServerTickPhase,
) {
    crate::game::deepbore_quarry_glue::server_tick(game, phase);
}
