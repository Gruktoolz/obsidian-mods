fn hook(
    title: &mut crate::title::Title,
    cursor: (f32, f32),
    size: (f32, f32),
) -> std::ops::ControlFlow<()> {
    if crate::server_browser::native::active(title) {
        crate::server_browser::native::drag(cursor, size);
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
