fn hook(
    title: &mut crate::title::Title,
    x: f32,
    y: f32,
    size: (f32, f32),
) -> std::ops::ControlFlow<(Action, bool)> {
    if crate::server_browser::native::active(title) {
        return std::ops::ControlFlow::Break(crate::server_browser::native::click(
            title, x, y, size,
        ));
    }
    if matches!(title.mode, crate::title::Mode::Main) {
        let multiplayer = title
            .buttons(size.0.ceil(), size.1.ceil())
            .into_iter()
            .any(|b| b.id == 4 && b.enabled && crate::ui::hovered((x, y), b.x, b.y, b.w, 20.0));
        if multiplayer && crate::server_browser::native::enter(title) {
            return std::ops::ControlFlow::Break((crate::title::Action::None, true));
        }
    }
    if crate::server_browser::native::classic_click(title, (x, y), size) {
        return std::ops::ControlFlow::Break((crate::title::Action::None, true));
    }
    std::ops::ControlFlow::Continue(())
}
