// A click on the mod's settings page: the next value, kept.
fn chat_heads_rust_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::chat::native_chat_heads_rust as m;
    if mod_id != m::ID {
        return;
    }
    crate::game::chat::native_chat_heads_rust_glue::click(game, button);
}
