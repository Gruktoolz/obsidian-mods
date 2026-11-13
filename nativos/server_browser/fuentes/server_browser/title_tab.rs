fn hook(title: &mut crate::title::Title) -> std::ops::ControlFlow<()> {
    if crate::server_browser::native::active(title) {
        crate::server_browser::native::key(title, winit::keyboard::KeyCode::Tab, false, false);
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
