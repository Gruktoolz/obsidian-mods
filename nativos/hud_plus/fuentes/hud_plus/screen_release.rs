// The end of a drag in the layout editor.
fn official_hud_plus_screen_release(
    game: &crate::game::Game<'_>,
    canceled: &mut bool,
) {
    if crate::game::official_hud_plus_glue::screen_release(game) {
        *canceled = true;
    }
}
