// Once a frame: the settings are read at the start, the mod follows its switch in Options > Mods,
// and after a change every loaded section is meshed again (as the game does for smooth lighting).
fn connected_textures_frame(game: &mut crate::game::Game<'_>) {
    use horizon_client_render::native_connected_textures as ct;
    if !ct::loaded() {
        ct::load(crate::game::Game::native_config_dir().ok());
    }
    ct::set_active(game.native_on(ct::ID));
    if ct::take_remesh() {
        game.all_sections_changed();
    }
}
