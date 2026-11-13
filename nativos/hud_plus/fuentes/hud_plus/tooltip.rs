// Food values and sneak details in an item's tooltip.
fn official_hud_plus_tooltip(
    game: &crate::game::Game<'_>,
    stack: &horizon_client::items::ItemStack,
    lines: &mut Vec<(horizon_client::text::Text, u32)>,
) {
    crate::game::official_hud_plus_glue::tooltip(game, stack, lines);
}
