// A click on Fair Freecam's settings page: change the setting and keep it.
fn freecam_settings_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::fair_freecam as fc;
    if mod_id != fc::ID {
        return;
    }
    let mut settings = {
        let game = &*game;
        fc::settings(|key| game.native_setting(fc::ID, key))
    };
    settings.click(button);
    fc::set_settings(settings);
    for (key, value) in settings.stored() {
        game.native_set_setting(fc::ID, key, &value);
    }
}
