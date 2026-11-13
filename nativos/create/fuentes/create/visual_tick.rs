fn create_visual_tick(game: &mut crate::game::Game<'_>) {
    crate::game::create_port_burner_sync::tick(game);
    crate::game::create_port_nativeaudio::begin_tick(game);
    crate::game::create_port_fan::client_tick(game);
    crate::game::create_port_presentation::tick(game);
    crate::game::create_port_burner_presentation::client_tick(game);
    crate::game::create_port_millstone::client_tick(game);
    crate::game::create_port_basin::client_tick(game);
    crate::game::create_port_mixer::client_tick(game);
    crate::game::create_port_nativeaudio::end_tick(game);
}
