fn hook(
    title: &mut crate::title::Title,
    shortcut: crate::clipboard::EditShortcut,
    board: &mut dyn crate::clipboard::Clipboard,
) -> std::ops::ControlFlow<()> {
    if crate::server_browser::native::active(title) {
        crate::server_browser::native::shortcut(shortcut, board);
        return std::ops::ControlFlow::Break(());
    }
    std::ops::ControlFlow::Continue(())
}
