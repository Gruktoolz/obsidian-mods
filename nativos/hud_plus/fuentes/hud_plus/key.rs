// The layout and settings hotkeys.
fn official_hud_plus_key(
    game: &mut crate::game::Game<'_>,
    key: &str,
    pressed: bool,
) {
    crate::game::official_hud_plus_glue::key(game, key, pressed);
}
