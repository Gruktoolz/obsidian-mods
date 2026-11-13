// The mod's settings page: heads, own head, default skins.
fn chat_heads_rust_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::chat::native_chat_heads_rust as m;
    if mod_id != m::ID {
        return;
    }
    buttons.extend(crate::game::chat::native_chat_heads_rust_glue::settings(game).labels());
}
