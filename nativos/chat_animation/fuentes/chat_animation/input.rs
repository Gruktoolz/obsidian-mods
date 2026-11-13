// ChatScreen::draw: the input field (and its suggestions) slide up from the screen's bottom edge
// as the chat opens.
fn chat_animation_input(
    screen: &mut ChatScreen,
    m: &Measure,
    mouse: (i32, i32),
    blink_ms: u64,
    background: i32,
) -> ::core::ops::ControlFlow<Vec<Draw>> {
    use crate::native_chat_animation as ca;
    if !screen.layout.anchor_to_bottom || !ca::enter(2) {
        return ::core::ops::ControlFlow::Continue(());
    }
    let mut out = screen.draw(m, mouse, blink_ms, background);
    ca::leave(2);
    ca::shift(&mut out, ca::input_offset());
    ::core::ops::ControlFlow::Break(out)
}
