// Keys on the config screen: Escape goes back to Options; the others do nothing there.
fn chat_animation_options_key(
    game: &mut crate::game::Game<'_>,
    key: KeyCode,
    text: Option<&str>,
) -> ::core::ops::ControlFlow<Result<()>> {
    use horizon_client_render::native_chat_animation as ca;
    let _ = (game, text);
    if !ca::menu_is_open() {
        return ::core::ops::ControlFlow::Continue(());
    }
    if key == KeyCode::Escape {
        ca::menu_close();
    }
    ::core::ops::ControlFlow::Break(Ok(()))
}
