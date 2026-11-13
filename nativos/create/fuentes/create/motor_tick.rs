fn motor_tick(game: &mut crate::game::Game<'_>) {
    crate::game::create_port_motor_value_world::tick(game);
    crate::game::create_port_motor_value_ui::tick(game);
}
