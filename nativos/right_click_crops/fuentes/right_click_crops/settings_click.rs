fn settings_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::right_click_crops as runtime;

    if mod_id != runtime::MOD_ID { return; }
    if !runtime::plant_settings_loaded() {
        let stored = game.native_setting(runtime::MOD_ID, runtime::CONFIG_FILE);
        runtime::load_plant_settings_once(stored.as_deref());
    }
    let Some(plant) = runtime::Plant::ALL.get(button).copied() else { return; };
    runtime::toggle_plant(plant);
    game.native_set_setting(runtime::MOD_ID, runtime::CONFIG_FILE, &runtime::plant_config());
}
