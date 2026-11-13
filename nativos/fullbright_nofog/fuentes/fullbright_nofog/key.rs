// A toggle key went down while playing: Fullbright or NoFog on or off.
fn fullbright_nofog_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::fullbright_nofog as m;
    if !pressed || !game.native_on(m::ID) || !game.native_playing() {
        return;
    }
    let command = if key == game.native_mod_key(m::ID, m::KEY_FULLBRIGHT, m::DEFAULT_KEY_FULLBRIGHT) {
        "fullbright"
    } else if key == game.native_mod_key(m::ID, m::KEY_NOFOG, m::DEFAULT_KEY_NOFOG) {
        "nofog"
    } else {
        return;
    };
    let before = {
        let game = &*game;
        m::settings(|k| game.native_setting(m::ID, k))
    };
    if let Some(after) = m::run(before, command) {
        m::set_settings(after);
        for (k, value) in after.stored() {
            game.native_set_setting(m::ID, k, &value);
        }
    }
}
