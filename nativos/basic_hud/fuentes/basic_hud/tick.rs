// A count ends with its player's life, and all of them with the world.
fn basic_hud_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::{basic_hud as m, basic_hud_data as data, basic_hud_players as players};
    if !game.native_on(m::ID) || !game.native_window() {
        data::clear_all();
        return;
    }
    let Some(own) = game.player.as_ref().map(|p| p.health.dead_or_dying()) else {
        m::pops(|p| p.clear());
        data::clear_all();
        return;
    };
    if own {
        let name = game.own_profile_name();
        m::pops(|p| p.forget(&name));
    }
    for other in players::others(game).into_iter().filter(|p| !p.alive) {
        m::pops(|p| p.forget(&other.name));
        data::remove_entity(other.id);
    }
    data::cleanup_stale_entities(game);
}
