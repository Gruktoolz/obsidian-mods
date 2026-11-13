// A click on a button of SeedCrackerX's page.
fn scx_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    if mod_id == crate::game::scx_game::ID {
        crate::game::scx_game::page_click(game, button);
    }
}
