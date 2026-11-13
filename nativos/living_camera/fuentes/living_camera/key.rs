// The toggle key, while playing: turn the whole mod on or off, and keep it.
fn living_camera_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::living_camera as m;
    if !pressed || !game.native_on(m::ID) || !game.native_playing() {
        return;
    }
    if game.native_mod_key(m::ID, m::TOGGLE_KEY, m::TOGGLE_KEY_DEFAULT) != key {
        return;
    }
    let mut settings = {
        let game = &*game;
        m::settings(|key| game.native_setting(m::ID, key))
    };
    let (stored, value) = settings.toggle();
    m::set_settings(settings);
    game.native_set_setting(m::ID, stored, &value);
    game.native_chat(&m::toggled_line(settings.enabled()));
}
