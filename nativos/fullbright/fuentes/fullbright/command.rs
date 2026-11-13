// `/gamma [value|reset|off]` and `/fullbright`; every other line goes on to
// the game and to other mods.
fn fullbright_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::fullbright_gamma as fb;
    if *ran || !game.native_on(fb::ID) {
        return;
    }
    let slider = game.settings.gamma;
    let before = {
        let game = &*game;
        fb::settings(|key| game.native_setting(fb::ID, key))
    };
    let Some((after, message)) = fb::run(before, command, slider) else {
        return;
    };
    *ran = true;
    if after != before {
        fb::set_settings(after);
        for (key, value) in after.stored() {
            game.native_set_setting(fb::ID, key, &value);
        }
    }
    game.native_chat(&message);
}
