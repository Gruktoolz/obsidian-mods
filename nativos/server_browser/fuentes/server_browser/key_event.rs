fn hook(
    game: &mut crate::game::Game<'_>,
    event: &winit::event::WindowEvent,
    window_id: winit::window::WindowId,
) -> std::ops::ControlFlow<()> {
    if game
        .window
        .as_ref()
        .is_none_or(|window| window.id() != window_id)
    {
        return std::ops::ControlFlow::Continue(());
    }
    if game.screen != crate::game::Screen::Playing {
        return std::ops::ControlFlow::Continue(());
    }
    let action = if let Some(title) = &mut game.title {
        horizon_client::server_browser::native::event(
            title,
            event,
            game.modifiers.control_key(),
            game.modifiers.shift_key(),
            game.modifiers.alt_key(),
            &mut game.clipboard,
        )
    } else {
        None
    };
    if let Some(action) = action {
        if let horizon_client::title::Action::Connect { address, lan } = action
            && let Err(error) =
                game.connect_server_with_options(&address, &game.options.player_name(), None, lan)
        {
            log::warn!("Server Browser: {error}");
        }
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
