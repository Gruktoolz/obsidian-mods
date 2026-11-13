fn hook(
    game: &mut crate::game::Game<'_>,
    out: &horizon_client::osk::Output,
) -> std::ops::ControlFlow<()> {
    if game.screen == crate::game::Screen::Playing
        && game
            .title
            .as_ref()
            .is_some_and(horizon_client::server_browser::native::active)
    {
        let mut action = horizon_client::title::Action::None;
        if let Some(title) = game.title.as_mut() {
            match out {
                horizon_client::osk::Output::Text(text) => title.type_text(text),
                horizon_client::osk::Output::Backspace => title.backspace(),
                horizon_client::osk::Output::Enter => {
                    action = horizon_client::server_browser::native::control_key(
                        title,
                        winit::keyboard::KeyCode::Enter,
                        "tap",
                        &mut game.clipboard,
                    )
                }
                horizon_client::osk::Output::Close => {
                    game.pad.menu.osk = None;
                    game.pad.menu.osk_dismissed = true;
                }
            }
        }
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
