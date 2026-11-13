fn context_tools_mouse(
    game: &mut crate::game::Game<'_>,
    button: winit::event::MouseButton,
    pressed: bool,
) -> core::ops::ControlFlow<Result<()>> {
    if button != winit::event::MouseButton::Middle {
        return core::ops::ControlFlow::Continue(());
    }
    match game.context_tools_pick(pressed) {
        Some(true) => core::ops::ControlFlow::Break(Ok(())),
        _ => core::ops::ControlFlow::Continue(()),
    }
}
