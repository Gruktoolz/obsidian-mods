fn settings_list(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    use crate::game::right_click_crops as runtime;

    if mod_id != runtime::MOD_ID { return; }
    if !runtime::plant_settings_loaded() {
        let stored = game.native_setting(runtime::MOD_ID, runtime::CONFIG_FILE);
        runtime::load_plant_settings_once(stored.as_deref());
    }
    buttons.extend(runtime::Plant::ALL.map(|plant| {
        let state = if runtime::plant_enabled(plant) { "ON" } else { "OFF" };
        format!("{}: {state}", plant.label())
    }));
}
