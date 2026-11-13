fn register_crop(blocks: &mut Vec<horizon_mods::content::Block>) {
    blocks.push(horizon_mods::content::Block {
        id: "inferium_lab:inferium_crop".into(),
        template: "minecraft:wheat".into(),
        ..Default::default()
    });
}
