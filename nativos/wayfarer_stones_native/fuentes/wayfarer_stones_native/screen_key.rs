// A key over the mod's screen.
fn wayfarer_stones_screen_key(
    game: &crate::game::Game<'_>,
    screen: &horizon_client::mod_api::NativeScreenView<'_>,
    key: &str,
    canceled: &mut bool,
) {
    crate::game::wayfarer_stones_glue::key_pressed(game, screen, key, canceled);
}
