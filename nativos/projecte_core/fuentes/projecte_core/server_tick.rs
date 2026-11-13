// The end of each server tick: open queued menus, run queued commands, save knowledge.
fn pe_server_tick(game: &mut crate::game::Game<'_>, phase: horizon_client::mod_api::ServerTickPhase) {
    if matches!(phase, horizon_client::mod_api::ServerTickPhase::ServerTickEnd { .. }) {
        crate::game::pe_glue::server_tick_end(game);
    }
}
