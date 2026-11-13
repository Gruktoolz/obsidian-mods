// Chat::layout: the chat as Horizon lays it out, then a new message rises into place and fades
// in, and the indicator bar is left out.
fn chat_animation_layout(chat: &Chat, height: i32, ticks: f32, focused: bool) -> ::core::ops::ControlFlow<Vec<GuiDraw>> {
    use crate::native_chat_animation as ca;
    if !ca::enter(1) {
        return ::core::ops::ControlFlow::Continue(());
    }
    let mut draws = chat.layout(height, ticks, focused);
    ca::leave(1);
    let newest = chat.lines.first().map_or(0, |l| l.added);
    let fresh = chat.lines.iter().take_while(|l| l.added == newest).count();
    ca::messages(&mut draws, chat.received, newest, fresh, chat.options.line_height(), chat.scroll > 0);
    ::core::ops::ControlFlow::Break(draws)
}
