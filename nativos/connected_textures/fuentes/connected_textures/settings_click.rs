// A settings button: the setting flips, is saved, and the world is meshed again.
fn connected_textures_settings_click(mod_id: &str, button: usize) {
    use horizon_client_render::native_connected_textures as ct;
    if mod_id != ct::ID {
        return;
    }
    if !ct::loaded() {
        ct::load(crate::game::Game::native_config_dir().ok());
    }
    ct::setting_click(button);
}
