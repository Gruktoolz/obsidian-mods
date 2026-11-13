fn deepbore_quarry_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    crate::game::deepbore_quarry_glue::perform(game, command, ran);
}
