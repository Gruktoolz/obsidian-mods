// Shift going down and up, for "Hold Shift" (keys come on any screen).
fn map_preview_key(key: &str, pressed: bool) {
    crate::game::map_preview::key(key, pressed);
}
