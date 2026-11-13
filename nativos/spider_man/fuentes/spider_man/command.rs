// The mod's chat commands (`/swingmode …`), before the server and Horizon's
// own commands see them.
fn plug_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    crate::game::spider_man::command(game, command, ran)
}
