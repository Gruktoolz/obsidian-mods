fn settings(mod_id: &str, buttons: &mut Vec<String>) {
    if mod_id == "tectonic_horizon" {
        buttons.extend(horizon_world::worldgen::overrides::tectonic_config_menu::buttons());
    }
}
