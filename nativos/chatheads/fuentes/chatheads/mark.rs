// Before the chat is drawn: whether the heads (and the room for them) are on.
fn chatheads_mark(game: &mut crate::game::Game<'_>) {
    crate::game::chat::native_chatheads_glue::mark(game);
}
