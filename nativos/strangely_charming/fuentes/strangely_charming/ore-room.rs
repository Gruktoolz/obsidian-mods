fn hook(piece: &MineshaftPiece, c: &mut PieceContext<'_>) {
    if !crate::evanders_config::enabled("mineshaft_ores") {
        return;
    }
    let b = piece.frame.bounding_box;
    let y = b.min[1];
    for x in b.min[0].max(c.chunk_box.min[0])..=b.max[0].min(c.chunk_box.max[0]) {
        for z in b.min[2].max(c.chunk_box.min[2])..=b.max[2].min(c.chunk_box.max[2]) {
            let pos = [x, y, z];
            if !c.chunk_box.is_inside(pos) {
                continue;
            }
            let Ok(old) = c.world.block(pos) else {
                continue;
            };
            let deep = match c.name(old) {
                "minecraft:stone" => false,
                "minecraft:deepslate" => true,
                _ => continue,
            };
            let draw = c.random.double();
            let ore = if draw < 0.007 && y < 16 {
                "diamond_ore"
            } else if draw < 0.04 {
                "iron_ore"
            } else if draw < 0.09 {
                "coal_ore"
            } else {
                continue;
            };
            let block = if deep {
                format!("deepslate_{ore}")
            } else {
                ore.to_owned()
            };
            let new = c.state(&block);
            let _ = c.world.set(pos, new, 2);
        }
    }
}
