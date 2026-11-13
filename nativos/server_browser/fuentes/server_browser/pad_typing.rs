fn hook(game: &crate::game::Game<'_>) -> std::ops::ControlFlow<bool> {
    if game.screen == crate::game::Screen::Playing
        && game
            .title
            .as_ref()
            .is_some_and(horizon_client::server_browser::native::active)
    {
        return std::ops::ControlFlow::Break(horizon_client::server_browser::native::typing());
    }
    std::ops::ControlFlow::Continue(())
}
