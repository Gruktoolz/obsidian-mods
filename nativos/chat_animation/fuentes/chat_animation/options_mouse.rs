// The mouse on the Options screen: "Chat Animation Settings..." opens the config screen, and
// while it is open every press is its own.
fn chat_animation_options_mouse(
    game: &mut crate::game::Game<'_>,
    pressed: bool,
    right: bool,
) -> ::core::ops::ControlFlow<Result<()>> {
    use horizon_client_render::native_chat_animation as ca;
    let (x, y) = game.cursor;
    if !ca::menu_is_open() {
        if pressed && !right && game.ui.options.page() == Page::Main && ca::button_at(x, y) {
            ca::menu_open();
            return ::core::ops::ControlFlow::Break(game.handle_option_events(vec![Event::ButtonClick]));
        }
        return ::core::ops::ControlFlow::Continue(());
    }
    if right {
        return ::core::ops::ControlFlow::Break(Ok(()));
    }
    game.dragging = pressed;
    let (w, h) = game.gui_size();
    let out = match ca::menu_mouse(w.ceil(), h.ceil(), x, y, pressed) {
        ca::MenuOut::None => Ok(()),
        ca::MenuOut::Click | ca::MenuOut::Close => game.handle_option_events(vec![Event::ButtonClick]),
    };
    ::core::ops::ControlFlow::Break(out)
}
