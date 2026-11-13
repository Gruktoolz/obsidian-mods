fn hook(
    title: &mut crate::title::Title,
    delta: f32,
    cursor: (f32, f32),
    size: (f32, f32),
) -> std::ops::ControlFlow<bool> {
    if crate::server_browser::native::active(title) {
        return std::ops::ControlFlow::Break({
            crate::server_browser::native::wheel(delta, cursor, size);
            false
        });
    }
    std::ops::ControlFlow::Continue(())
}
