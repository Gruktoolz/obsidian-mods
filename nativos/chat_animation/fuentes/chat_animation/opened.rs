// ChatScreen::new: the chat opened, its input field starts its slide.
fn chat_animation_opened(width: i32) {
    let _ = width;
    crate::native_chat_animation::input_opened();
}
