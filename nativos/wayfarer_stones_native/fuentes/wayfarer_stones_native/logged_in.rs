fn wayfarer_stones_logged_in(game: &mut crate::game::Game<'_>, player: u128, name: &str) {
    crate::game::wayfarer_stones_glue::logged_in(game, player, name);
}
