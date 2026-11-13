// After the chat is drawn: the heads, over its own background.
fn chat_heads_rust_heads(
    value: (),
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
) {
    let _ = value;
    crate::game::chat::native_chat_heads_rust_glue::heads(game, batch);
}
