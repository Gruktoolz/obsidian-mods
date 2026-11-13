// Once a frame: the settings are read at the start, and the mod follows its switch in
// Options > Mods.
fn chat_animation_frame(game: &mut crate::game::Game<'_>) {
    use horizon_client_render::native_chat_animation as ca;
    if !ca::loaded() {
        ca::load(crate::game::Game::native_config_dir().ok());
    }
    ca::set_active(game.native_on(ca::ID));
}
