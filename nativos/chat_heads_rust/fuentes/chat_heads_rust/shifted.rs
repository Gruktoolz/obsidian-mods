// The chat's laid-out lines, moved right for the heads while the mod is on.
fn chat_heads_rust_shifted(value: Vec<crate::hud_layout::GuiDraw>) -> Vec<crate::hud_layout::GuiDraw> {
    crate::chat::native_chat_heads_rust_layout::shifted(value)
}
