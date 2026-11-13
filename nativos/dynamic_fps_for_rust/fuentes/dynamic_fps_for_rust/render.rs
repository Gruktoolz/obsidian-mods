fn render(game: &crate::game::Game<'_>) -> ::core::ops::ControlFlow<Result<bool>> {
    if crate::game::dynamic_fps_adapter::skip_render(game) {
        ::core::ops::ControlFlow::Break(Ok(false))
    } else {
        ::core::ops::ControlFlow::Continue(())
    }
}
