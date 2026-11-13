// The world's phases: the files are written when it closes.
fn wayfarer_stones_world_lifecycle(phase: horizon_client::mod_api::WorldPhase) {
    crate::game::wayfarer_stones_glue::lifecycle(phase);
}
