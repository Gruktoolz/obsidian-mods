// EMC, stored EMC, charge, mode and on/off in item tooltips.
fn pe_tooltip(
    game: &crate::game::Game<'_>,
    stack: &horizon_client::items::ItemStack,
    lines: &mut Vec<(horizon_client::text::Text, u32)>,
) {
    crate::game::pe_glue::tooltip(game, stack, lines);
}
