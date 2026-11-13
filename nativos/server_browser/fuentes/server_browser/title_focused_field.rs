fn hook(
    title: &mut crate::title::Title,
) -> Option<(crate::clipboard::FieldId, &mut String, usize)> {
    if crate::server_browser::native::active(title) || title.rules.get_mut().is_some() {
        return None;
    }
    match &mut title.mode {
        crate::title::Mode::Multiplayer(screen) => screen.focused_text(),
        mode => caret_field(mode, title.search_focused, &mut title.search),
    }
}
