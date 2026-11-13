fn startup(game: &crate::game::Game<'_>) {
    use crate::game::right_click_crops as runtime;

    if !runtime::plant_settings_loaded() {
        let stored = game.native_setting(runtime::MOD_ID, runtime::CONFIG_FILE);
        runtime::load_plant_settings_once(stored.as_deref());
    }
    if runtime::enabled(game.native_on(runtime::MOD_ID)) {
        runtime::report_startup_once();
    }
}
