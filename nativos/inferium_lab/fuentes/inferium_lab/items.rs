fn register_items(items: &mut Vec<horizon_mods::content::Item>) {
    use horizon_mods::content::Item;
    use serde_json::json;

    for (id, template, translation, fallback) in [
        (
            "inferium_lab:inferium_seeds",
            "minecraft:wheat_seeds",
            "item.inferium_lab.inferium_seeds",
            "Inferium Seeds",
        ),
        (
            "inferium_lab:inferium_essence",
            "minecraft:redstone",
            "item.inferium_lab.inferium_essence",
            "Inferium Essence",
        ),
    ] {
        items.push(Item {
            id: id.into(),
            template: template.into(),
            components: [
                ("minecraft:item_model".into(), json!(template)),
                (
                    "minecraft:item_name".into(),
                    json!({"translate": translation, "fallback": fallback}),
                ),
            ]
            .into(),
            ..Default::default()
        });
    }
}
