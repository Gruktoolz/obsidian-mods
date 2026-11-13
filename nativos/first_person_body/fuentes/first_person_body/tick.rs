// Each tick on the window: the settings are read once, for the render hooks.
fn first_person_body_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::mobs::first_person_body as m;
    if game.native_window() {
        let game = &*game;
        m::settings(|key| game.native_setting(m::ID, key));
    }
}
