fn hook(title: &mut crate::title::Title) -> std::ops::ControlFlow<bool> {
    if crate::server_browser::native::active(title) {
        return std::ops::ControlFlow::Break(crate::server_browser::native::typing());
    }
    std::ops::ControlFlow::Continue(())
}
