// The tooltip's picture, in the space `measure.rs` asked for (not in another
// mod's).
fn map_preview_draw(
    game: &crate::game::Game<'_>,
    stack: &horizon_client::items::ItemStack,
    batch: &mut horizon_client::hud::HudBatch,
    origin: [f32; 2],
    size: [f32; 2],
) {
    use crate::game::map_preview as m;
    if !game.native_on(m::ID) {
        return;
    }
    let Some(id) = crate::game::maps::map_id(stack) else {
        return;
    };
    if !m::claimed(id, size) {
        return;
    }
    let Some(map) = game.maps.data.get(&id) else {
        return;
    };
    let settings = m::settings(|key| game.native_setting(m::ID, key));
    let paper = settings.get(m::o::PAPER) == 1;
    let palette = &horizon_client::blocks::MAP_COLORS;
    for (rect, argb) in m::tooltip_rects(&map.colors, palette, origin, size[0], paper) {
        game.native_fill(batch, rect, argb);
    }
}
