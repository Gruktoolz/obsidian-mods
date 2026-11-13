// A button of the mod's page in Options > Mods: its setting changes and is saved.
fn chat_animation_settings_click(mod_id: &str, button: usize) {
    use horizon_client_render::native_chat_animation as ca;
    if mod_id != ca::ID {
        return;
    }
    if !ca::loaded() {
        ca::load(crate::game::Game::native_config_dir().ok());
    }
    ca::setting_click(button);
}
