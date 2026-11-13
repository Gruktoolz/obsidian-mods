// The pointer moving with the button down on the config screen: a slider follows it.
fn chat_animation_options_drag(game: &mut crate::game::Game<'_>) -> ::core::ops::ControlFlow<Result<()>> {
    use horizon_client_render::native_chat_animation as ca;
    if !ca::menu_is_open() {
        return ::core::ops::ControlFlow::Continue(());
    }
    let (w, h) = game.gui_size();
    ca::menu_drag(w.ceil(), h.ceil(), game.cursor.0);
    ::core::ops::ControlFlow::Break(Ok(()))
}
