// The wheel in HUD+'s own screen: scaling a module, or scrolling settings.
fn official_hud_plus_screen_scroll(
    game: &crate::game::Game<'_>,
    screen: &horizon_client::mod_api::NativeScreenView<'_>,
    scroll: [f64; 2],
    canceled: &mut bool,
) {
    if crate::game::official_hud_plus_glue::screen_scroll(game, screen, scroll) {
        *canceled = true;
    }
}
