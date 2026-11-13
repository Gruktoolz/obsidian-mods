// The wheel over the mod's screen.
fn wayfarer_stones_screen_wheel(
    game: &crate::game::Game<'_>,
    screen: &horizon_client::mod_api::NativeScreenView<'_>,
    scroll: [f64; 2],
    canceled: &mut bool,
) {
    crate::game::wayfarer_stones_glue::mouse_scrolled(game, screen, scroll, canceled);
}
