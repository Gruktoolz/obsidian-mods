// Once a tick: follow the open screen, cache the sneak key and apply a
// screen change a settings row asked for.
fn official_hud_plus_tick(game: &mut crate::game::Game<'_>) {
    crate::game::official_hud_plus_glue::tick(game);
}
