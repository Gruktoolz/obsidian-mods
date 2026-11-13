fn hook(
    menu: &mut crate::game::startup_menu::StartupMenu,
    event: &winit::event::KeyEvent,
) -> std::ops::ControlFlow<Action> {
    let action = if let Some(title) = &mut menu.title {
        horizon_client::server_browser::native::key_event(
            title,
            event,
            menu.modifiers.control_key(),
            menu.modifiers.shift_key(),
            menu.modifiers.alt_key(),
            &mut menu.clipboard,
        )
    } else {
        None
    };
    if let Some(action) = action {
        return std::ops::ControlFlow::Break(action);
    }
    std::ops::ControlFlow::Continue(())
}
