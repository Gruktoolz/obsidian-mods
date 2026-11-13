// Dragging a module in the layout editor.
fn official_hud_plus_screen_drag(
    game: &crate::game::Game<'_>,
    screen: &horizon_client::mod_api::NativeScreenView<'_>,
    canceled: &mut bool,
) {
    if crate::game::official_hud_plus_glue::screen_drag(game, screen) {
        *canceled = true;
    }
}
