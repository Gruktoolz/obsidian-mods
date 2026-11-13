// A click on Fullbright's settings page: change the setting and keep it.
fn fullbright_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::fullbright_gamma as fb;
    if mod_id != fb::ID {
        return;
    }
    let mut settings = {
        let game = &*game;
        fb::settings(|key| game.native_setting(fb::ID, key))
    };
    settings.click(button);
    fb::set_settings(settings);
    for (key, value) in settings.stored() {
        game.native_set_setting(fb::ID, key, &value);
    }
}
