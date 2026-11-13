// The chat's laid-out lines, moved right for the heads while the mod is on.
fn chatheads_shifted(value: Vec<crate::hud_layout::GuiDraw>) -> Vec<crate::hud_layout::GuiDraw> {
    crate::chat::native_chatheads_layout::shifted(value)
}
