// Before the chat is drawn: whether the heads (and the room for them) are on.
fn chat_heads_rust_mark(game: &mut crate::game::Game<'_>) {
    crate::game::chat::native_chat_heads_rust_glue::mark(game);
}
