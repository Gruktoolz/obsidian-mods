// /seedcracker, before the server and Horizon's commands see it.
fn scx_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    if !*ran && game.native_on(crate::game::scx_game::ID) {
        crate::game::scx_game::command(game, command, ran);
    }
}
