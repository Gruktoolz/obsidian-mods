fn rarity_borders_decorations(
    game: &crate::game::Game<'_>,
    stack: &ItemStack,
    x: f32,
    y: f32,
    batch: &mut crate::hud::HudBatch,
) {
    use crate::game::rarity_borders as m;

    let enabled = m::is_enabled(|| game.native_setting(m::ID, "enabled").map(|v| v != "false"));
    if !enabled {
        return;
    }

    // Skip empty stacks or air
    if stack.count == 0 || &*stack.item == "minecraft:air" {
        return;
    }

    let is_playing_hud = game.screen == crate::game::Screen::Playing;
    let show_hotbar =
        m::is_show_hotbar(|| game.native_setting(m::ID, "hotbar").map(|v| v == "true"));
    if is_playing_hud && !show_hotbar {
        return;
    }

    let is_carried = game.carried.is_some()
        && (x - (game.cursor.0 - 8.0)).abs() < 0.5
        && (y - (game.cursor.1 - 8.0)).abs() < 0.5;
    if is_carried {
        return;
    }

    let default_rarity = game.item_rarity(&stack.item);
    let rarity_str = crate::item_tooltip::rarity(stack, &default_rarity);
    let show_common =
        m::is_show_common(|| game.native_setting(m::ID, "common").map(|v| v == "true"));

    m::render_slot_decoration(
        x,
        y,
        enabled,
        stack.count,
        &stack.item,
        is_playing_hud,
        show_hotbar,
        is_carried,
        rarity_str,
        show_common,
        |rect, argb| {
            game.native_fill(batch, rect, argb);
        },
    );
}
