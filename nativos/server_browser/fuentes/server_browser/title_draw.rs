fn hook(
    title: &crate::title::Title,
    atlas: &crate::hud::HudAtlas,
    batch: &mut crate::hud::HudBatch,
    cursor: (f32, f32),
) -> std::ops::ControlFlow<()> {
    if crate::server_browser::native::active(title) {
        crate::server_browser::native::draw(atlas, batch, cursor);
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
