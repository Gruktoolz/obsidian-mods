// A mouse button over the mod's screen.
fn wayfarer_stones_screen_mouse(
    game: &crate::game::Game<'_>,
    screen: &horizon_client::mod_api::NativeScreenView<'_>,
    button: i32,
    canceled: &mut bool,
) {
    crate::game::wayfarer_stones_glue::mouse_pressed(game, screen, button, canceled);
}
