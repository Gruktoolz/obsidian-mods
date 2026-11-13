// Game::inventory_mouse_release HEAD (pinned): the button went up, the drag
// ends. The release itself is the game's.
fn mousetricks_on_release(game: &mut crate::game::Game<'_>) {
    game.mousetricks_release();
}
