// After the chat is drawn: the heads, over its own background.
fn chatheads_heads(
    value: (),
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
) {
    let _ = value;
    crate::game::chat::native_chatheads_glue::heads(game, batch);
}
