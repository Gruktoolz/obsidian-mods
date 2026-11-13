// The freecam key went down: leave the body, or snap back to it.
fn freecam_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::fair_freecam as fc;
    // Turned off in Options > Mods: nothing until the next build leaves it out.
    if !pressed || !game.native_on(fc::ID) || !game.native_playing() {
        return;
    }
    if key != game.native_mod_key(fc::ID, fc::KEY, fc::DEFAULT_KEY) {
        return;
    }
    if fc::active() {
        fc::stop();
        return;
    }
    if game.player.is_none() {
        return;
    }
    let settings = {
        let game = &*game;
        fc::settings(|name| game.native_setting(fc::ID, name))
    };
    // Your own world (single player, or the LAN world you host) always allows it.
    if game.integrated.is_none() && !settings.servers {
        game.native_chat(fc::SERVER_OFF);
        return;
    }
    // Start where the view was last drawn from.
    fc::start(game.view.position, game.view.y_rot, game.view.x_rot);
}
