// Entity event 35, in the window's game only: the single player server thread
// passes here too, for the same totem.
fn basic_hud_totem(game: &mut crate::game::Game<'_>, body: &Body, own: bool) {
    use crate::game::{basic_hud as m, basic_hud_players as players};
    if !game.native_on(m::ID) || !game.native_window() {
        return;
    }
    let name = if own { Some(game.own_profile_name()) } else { players::at(game, body.at) };
    if let Some(name) = name {
        m::pops(|p| p.add(&name));
    }
}
