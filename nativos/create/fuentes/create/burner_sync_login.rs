fn create_burner_sync_login(game: &mut crate::game::Game<'_>, player: u128) {
    crate::game::create_port_burner_sync::logged_in(game, player);
}
