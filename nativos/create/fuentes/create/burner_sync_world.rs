fn create_burner_sync_world(game: &crate::game::Game<'_>, phase: crate::mod_api::WorldPhase) {
    if matches!(
        phase,
        crate::mod_api::WorldPhase::WorldOpenFailed
            | crate::mod_api::WorldPhase::WorldCloseFinal
            | crate::mod_api::WorldPhase::WorldCloseFailed
    ) {
        crate::game::create_port_burner_sync::clear(game);
    }
}
