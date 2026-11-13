// The client's keys: charge (V), mode (G), extra function (C), fire projectile (R) go to the server.
fn pe_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    crate::game::pe_glue::key(game, key, pressed);
}
