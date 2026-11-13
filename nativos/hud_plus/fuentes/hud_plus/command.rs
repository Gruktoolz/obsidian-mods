// Runs `/hudplus`.
fn official_hud_plus_command(
    game: &mut crate::game::Game<'_>,
    command: &str,
    ran: &mut bool,
) {
    crate::game::official_hud_plus_glue::command(game, command, ran);
}
