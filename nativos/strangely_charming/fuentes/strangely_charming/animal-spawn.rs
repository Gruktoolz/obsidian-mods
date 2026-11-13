fn hook(game: &mut crate::game::Game<'_>, from: usize, natural: bool) {
    use horizon_core::random::RandomSource;
    if !natural {
        return;
    }
    for i in from..game.mobs.len() {
        let m = &game.mobs[i];
        let pos = m.position;
        let id = match m.kind.id.as_str() {
            "minecraft:squid" | "minecraft:tropical_fish"
                if horizon_mods::evanders_config::enabled("coral_squids")
                    && game.biome_name_at(pos).as_deref() == Some("minecraft:warm_ocean")
                    && (-3..=3).any(|x| {
                        (-3..=3).any(|z| {
                            (-4..=1).any(|y| {
                                let block = game.block_name([
                                    pos.x.floor() as i32 + x,
                                    pos.y.floor() as i32 + y,
                                    pos.z.floor() as i32 + z,
                                ]);
                                block.contains("_coral") && !block.contains("dead_")
                            })
                        })
                    }) =>
            {
                "strangely_charming:coral_squid"
            }
            _ => continue,
        };
        if game.world.random.next_float() < 0.35
            && let Some(kind) = game.res.mob_types().get(id).cloned()
        {
            let old = &game.mobs[i];
            let mut replacement =
                horizon_client::mobs::Mob::new(old.id, kind, old.position, old.yaw);
            replacement.persistent = old.persistent;
            replacement.extra.age = old.extra.age;
            game.mobs[i] = replacement;
        }
    }
}
