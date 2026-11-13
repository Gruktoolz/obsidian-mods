// A map's slot (hotbar, inventory, containers): its picture over the plain
// map icon, when the client has the map.
fn map_preview_slot(
    game: &crate::game::Game<'_>,
    stack: &horizon_client::items::ItemStack,
    x: f32,
    y: f32,
    batch: &mut horizon_client::hud::HudBatch,
) {
    use crate::game::map_preview as m;
    if !game.native_on(m::ID) {
        return;
    }
    let Some(id) = crate::game::maps::map_id(stack) else {
        return;
    };
    let settings = m::settings(|key| game.native_setting(m::ID, key));
    if settings.get(m::o::SLOT) == 0 {
        return;
    }
    let Some(map) = game.maps.data.get(&id) else {
        return;
    };
    let edged = settings.get(m::o::SLOT_STYLE) == 1;
    let paper = settings.get(m::o::PAPER) == 1;
    let palette = &horizon_client::blocks::MAP_COLORS;
    for (rect, argb) in m::slot_rects(&map.colors, palette, [x, y], edged, paper) {
        game.native_fill(batch, rect, argb);
    }
}
