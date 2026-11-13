fn hook(title: &mut crate::title::Title, text: &str) -> std::ops::ControlFlow<()> {
    if crate::server_browser::native::active(title) {
        crate::server_browser::native::type_text(text);
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
