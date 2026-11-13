// Original MIT HUD implementation. No Java mod code is incorporated.
fn held_item_info(
    game: &crate::game::Game<'_>,
    layer: &str,
    batch: &mut horizon_client::hud::HudBatch,
) {
    if layer != "minecraft:selected_item_name" || game.player.is_none() {
        return;
    }
    let Some(stack) = game.held().filter(|s| s.count > 0) else {
        return;
    };
    let maximum = game.res.items().get(&stack.item).map_or(0, |i| i.max_damage);
    let mut details = Vec::new();
    if maximum > 0 {
        details.push(format!("{} / {}", maximum.saturating_sub(stack.damage), maximum));
    }
    if stack.count > 1 {
        details.push(format!("×{}", stack.count));
    }
    for (id, level) in stack.enchantments().iter().take(6) {
        let key = format!("enchantment.{}", id.replace(':', "."));
        let name = horizon_client::lang::get(&key);
        details.push(format!("{name} {level}"));
    }
    if details.is_empty() {
        return;
    }
    let mut text = details.join(" · ");
    let available = (batch.width - 8.0).max(0.0);
    if available < game.native_text_width("…") {
        return;
    }
    if game.native_text_width(&text) > available {
        while !text.is_empty() && game.native_text_width(&format!("{text}…")) > available {
            text.pop();
        }
        text.push('…');
    }
    let width = game.native_text_width(&text);
    // Keep text above the item-name/health rows. Clamp the horizontal origin for small GUIs.
    let x = ((batch.width - width) * 0.5).max(2.0);
    game.native_text(batch, &text, [x, (batch.height - 76.0).max(2.0)], 0xE0E0E0, true);
}
