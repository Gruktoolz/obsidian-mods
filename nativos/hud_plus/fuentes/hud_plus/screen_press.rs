// A click on HUD+'s own screen.
fn official_hud_plus_screen_press(
    game: &crate::game::Game<'_>,
    screen: &horizon_client::mod_api::NativeScreenView<'_>,
    button: i32,
    canceled: &mut bool,
) {
    if crate::game::official_hud_plus_glue::screen_press(game, screen, button) {
        *canceled = true;
    }
}
