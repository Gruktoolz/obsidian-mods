// The world closed: save every player's knowledge.
fn pe_world_lifecycle(phase: horizon_client::mod_api::WorldPhase) {
    if matches!(phase, horizon_client::mod_api::WorldPhase::WorldCloseFinal | horizon_client::mod_api::WorldPhase::WorldCloseFailed) {
        crate::game::pe_glue::world_closing();
    }
}
