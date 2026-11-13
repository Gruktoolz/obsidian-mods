fn settings_click(mod_id: &str, button: usize) {
    if mod_id == "tectonic_horizon" {
        horizon_world::worldgen::overrides::tectonic_config_menu::click(button);
    }
}
