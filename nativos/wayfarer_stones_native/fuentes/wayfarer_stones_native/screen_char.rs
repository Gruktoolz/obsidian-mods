// A character typed at the mod's screen.
fn wayfarer_stones_screen_char(
    game: &crate::game::Game<'_>,
    screen: &horizon_client::mod_api::NativeScreenView<'_>,
    c: char,
    canceled: &mut bool,
) {
    crate::game::wayfarer_stones_glue::char_typed(game, screen, c, canceled);
}
