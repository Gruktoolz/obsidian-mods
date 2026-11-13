// A player's command on the server: the mod's own, when it is one.
fn wayfarer_stones_perform_command(
    game: &mut crate::game::Game<'_>,
    player: &horizon_client::mod_api::NativeServerPlayer,
    command: &str,
    ran: &mut bool,
) {
    crate::game::wayfarer_stones_glue::perform(game, player, command, ran);
}
