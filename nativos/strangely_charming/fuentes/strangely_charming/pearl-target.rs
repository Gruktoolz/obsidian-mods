fn hook(game: &mut crate::game::Game<'_>, avoid: &str) -> core::ops::ControlFlow<bool> {
    if !horizon_mods::evanders_config::enabled("ender_pearl_blocks") {
        return core::ops::ControlFlow::Continue(());
    }
    let Some(at) = game.player.as_ref().map(|p| p.position) else {
        return core::ops::ControlFlow::Continue(());
    };
    let base = [
        at.x.floor() as i32,
        at.y.floor() as i32,
        at.z.floor() as i32,
    ];
    let mut best = None;
    let mut distance = 17 * 17;
    for x in -16..=16 {
        for z in -16..=16 {
            let gx = base[0] + x;
            let gz = base[2] + z;
            if !game.world.loaded(gx, gz) {
                continue;
            }
            for y in -16..=16 {
                let d = x * x + y * y + z * z;
                if d > 256 || d >= distance {
                    continue;
                }
                let p = [gx, base[1] + y, gz];
                if game.block_name(p).as_ref() == "strangely_charming:ender_pearl_block"
                    && (1..=2).all(|dy| {
                        matches!(
                            game.block_name([gx, p[1] + dy, gz]).as_ref(),
                            "minecraft:air" | "minecraft:cave_air" | "minecraft:void_air"
                        )
                    })
                {
                    best = Some(Vec3::new(
                        f64::from(gx) + 0.5,
                        f64::from(p[1] + 1),
                        f64::from(gz) + 0.5,
                    ));
                    distance = d;
                }
            }
        }
    }
    if let Some(at) = best {
        let ground = game
            .world
            .block(at.x.floor() as i32, at.y as i32 - 1, at.z.floor() as i32);
        if game.check_position_and_teleport(at, ground, avoid) {
            return core::ops::ControlFlow::Break(true);
        }
    }
    core::ops::ControlFlow::Continue(())
}
