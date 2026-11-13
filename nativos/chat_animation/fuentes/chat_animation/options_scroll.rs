// The wheel on the config screen: the list scrolls when it does not fit.
fn chat_animation_options_scroll(game: &mut crate::game::Game<'_>, amount: f32) -> ::core::ops::ControlFlow<Result<()>> {
    use horizon_client_render::native_chat_animation as ca;
    if !ca::menu_is_open() {
        return ::core::ops::ControlFlow::Continue(());
    }
    let (w, h) = game.gui_size();
    ca::menu_scroll(w.ceil(), h.ceil(), amount);
    ::core::ops::ControlFlow::Break(Ok(()))
}
