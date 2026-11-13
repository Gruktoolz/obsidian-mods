fn deepbore_quarry_logged_in(game: &mut crate::game::Game<'_>, player: u128, name: &str) {
    crate::game::deepbore_quarry_glue::logged_in(game, player, name);
}
