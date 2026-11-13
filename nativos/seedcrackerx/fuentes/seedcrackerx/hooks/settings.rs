// SeedCrackerX's page: the buttons of its screen.
fn scx_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    if mod_id == crate::game::scx_game::ID {
        buttons.extend(crate::game::scx_game::page_labels(game));
    }
}
