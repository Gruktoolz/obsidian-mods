// Clear health cache for entity id before spawning in the client window.
fn basic_hud_entity_spawn(game: &mut crate::game::Game<'_>, id: i32) {
    use crate::game::basic_hud as m;
    if game.native_window() && game.native_on(m::ID) {
        crate::game::basic_hud_data::remove_entity(id as u32);
    }
}
