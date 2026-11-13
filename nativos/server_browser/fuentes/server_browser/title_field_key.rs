fn hook(
    title: &mut crate::title::Title,
    key: winit::keyboard::KeyCode,
    ctrl: bool,
    shift: bool,
) -> std::ops::ControlFlow<bool> {
    if crate::server_browser::native::active(title) {
        return std::ops::ControlFlow::Break(crate::server_browser::native::field_key(
            key, ctrl, shift,
        ));
    }
    std::ops::ControlFlow::Continue(())
}
