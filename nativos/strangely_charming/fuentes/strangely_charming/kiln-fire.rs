fn hook(game: &mut crate::game::Game<'_>, pos: Pos) -> core::ops::ControlFlow<Result<()>> {
    if game.block_name([pos.0, pos.1, pos.2]).as_ref() == "strangely_charming:kiln" {
        if !horizon_mods::evanders_config::enabled("kilns") {
            return core::ops::ControlFlow::Break(Ok(()));
        }
        if let Some(BlockEntity::Furnace(f)) = game.world.block_entities.get_mut(&pos) {
            if let Some(s) = &f.items[0] {
                let id = s.item.strip_prefix("minecraft:").unwrap_or("");
                if !matches!(
                    id,
                    "clay_ball"
                        | "clay"
                        | "sand"
                        | "red_sand"
                        | "cobblestone"
                        | "stone"
                        | "sandstone"
                        | "red_sandstone"
                        | "quartz_block"
                        | "stone_bricks"
                        | "nether_bricks"
                        | "netherrack"
                ) && !id.ends_with("_terracotta")
                {
                    // Let the normal empty furnace tick handle fuel, lighting and cooldown.
                    let invalid = f.items[0].take();
                    let result = game.tick_furnace(pos);
                    if let Some(BlockEntity::Furnace(f)) = game.world.block_entities.get_mut(&pos) {
                        f.items[0] = invalid;
                    }
                    return core::ops::ControlFlow::Break(result);
                }
                if f.lit_remaining > 0
                    && f.cooking_timer > 0
                    && f.cooking_timer + 1 < f.cooking_total
                {
                    f.cooking_timer += 1;
                }
            }
        }
    }
    core::ops::ControlFlow::Continue(())
}
