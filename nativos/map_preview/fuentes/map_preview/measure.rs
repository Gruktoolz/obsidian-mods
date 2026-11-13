// A map's tooltip asks for space below its lines for the picture, when the
// client has the map and the settings show it. Reads the game's own maps
// (`maps.data`, `maps::map_id`): Horizon has no hook point for a map's pixels.
fn map_preview_measure(
    game: &crate::game::Game<'_>,
    stack: &horizon_client::items::ItemStack,
    size: &mut Option<[f32; 2]>,
) {
    use crate::game::map_preview as m;
    if size.is_some() || !game.native_on(m::ID) {
        return;
    }
    let Some(id) = crate::game::maps::map_id(stack) else {
        return;
    };
    if !game.maps.data.contains_key(&id) {
        return;
    }
    let settings = m::settings(|key| game.native_setting(m::ID, key));
    let Some(side) = settings.tooltip_side(m::shift()) else {
        return;
    };
    let space = m::tooltip_space(side);
    *size = Some(space);
    m::claim(id, space);
}
