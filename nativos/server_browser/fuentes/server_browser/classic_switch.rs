fn hook(
    screen: &crate::multiplayer::Screen,
    atlas: &crate::hud::HudAtlas,
    batch: &mut crate::hud::HudBatch,
    cursor: (f32, f32),
) {
    crate::server_browser::native::draw_classic_switch(screen, atlas, batch, cursor);
}
