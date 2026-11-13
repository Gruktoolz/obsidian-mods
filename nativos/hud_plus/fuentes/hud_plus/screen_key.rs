// The keys of HUD+'s own screen: Tab, the arrows, R, Space and Escape.
fn official_hud_plus_screen_key(
    game: &crate::game::Game<'_>,
    screen: &horizon_client::mod_api::NativeScreenView<'_>,
    key: &str,
    repeat: bool,
    canceled: &mut bool,
) {
    if crate::game::official_hud_plus_glue::screen_key(game, screen, key, repeat) {
        *canceled = true;
    }
}
