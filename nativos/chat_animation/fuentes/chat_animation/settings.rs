// The mod's page in Options > Mods: one button per setting, each showing its value.
fn chat_animation_settings(mod_id: &str, buttons: &mut Vec<String>) {
    use horizon_client_render::native_chat_animation as ca;
    if mod_id != ca::ID {
        return;
    }
    if !ca::loaded() {
        ca::load(crate::game::Game::native_config_dir().ok());
    }
    buttons.extend(ca::setting_labels());
}
