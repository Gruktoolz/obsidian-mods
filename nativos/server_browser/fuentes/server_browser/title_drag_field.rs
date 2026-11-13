fn hook(
    title: &mut crate::title::Title,
    atlas: &crate::hud::HudAtlas,
    x: f32,
) -> std::ops::ControlFlow<()> {
    if crate::server_browser::native::active(title) {
        crate::server_browser::native::drag_field(atlas, x);
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
