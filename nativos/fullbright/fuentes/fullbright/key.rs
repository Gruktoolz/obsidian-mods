// The toggle key went down while playing: fullbright on or off.
fn fullbright_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::fullbright_gamma as fb;
    if !pressed || !game.native_on(fb::ID) || !game.native_playing() {
        return;
    }
    if key != game.native_mod_key(fb::ID, fb::KEY, fb::DEFAULT_KEY) {
        return;
    }
    let slider = game.settings.gamma;
    let before = {
        let game = &*game;
        fb::settings(|k| game.native_setting(fb::ID, k))
    };
    if let Some((after, message)) = fb::run(before, "fullbright", slider) {
        fb::set_settings(after);
        for (k, value) in after.stored() {
            game.native_set_setting(fb::ID, k, &value);
        }
        game.native_chat(&message);
    }
}
