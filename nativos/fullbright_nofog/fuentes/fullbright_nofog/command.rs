// `/fullbright [on|off]` and `/nofog [on|off|fluids]`; every other line goes
// on to the game and to other mods.
fn fullbright_nofog_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::fullbright_nofog as m;
    if *ran || !game.native_on(m::ID) {
        return;
    }
    let before = {
        let game = &*game;
        m::settings(|key| game.native_setting(m::ID, key))
    };
    let Some(after) = m::run(before, command) else {
        return;
    };
    *ran = true;
    m::set_settings(after);
    for (key, value) in after.stored() {
        game.native_set_setting(m::ID, key, &value);
    }
}
