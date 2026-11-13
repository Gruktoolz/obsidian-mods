// The settings page: one button per setting, each showing its value.
fn connected_textures_settings(mod_id: &str, buttons: &mut Vec<String>) {
    use horizon_client_render::native_connected_textures as ct;
    if mod_id != ct::ID {
        return;
    }
    if !ct::loaded() {
        ct::load(crate::game::Game::native_config_dir().ok());
    }
    buttons.extend(ct::setting_labels());
}
