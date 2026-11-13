fn hook(id: &str, table: &mut Value) {
    let bonus: Option<(&str, f32, &[&str])> = match id {
        "minecraft:chests/end_city_treasure" => {
            Some(("enchanting_totems", 0.125, &["totem_of_enchanting"]))
        }
        "minecraft:chests/simple_dungeon" | "minecraft:chests/village/village_cartographer" => {
            Some((
                "scrollkeepers",
                0.20,
                &[
                    "scroll_wheat",
                    "scroll_bread",
                    "scroll_coal",
                    "scroll_zombies",
                ],
            ))
        }
        _ => None,
    };
    if let Some((flag, chance, items)) = bonus
        && crate::evanders_config::enabled(flag)
        && let Some(pools) = table["pools"].as_array_mut()
    {
        let entries: Vec<_>=items.iter().map(|item|serde_json::json!({"type":"minecraft:item","name":format!("strangely_charming:{item}")})).collect();
        pools.push(serde_json::json!({"rolls":1,"condition":{"type":"minecraft:random_chance","chance":chance},"entries":entries}));
    }
    let (key, items): (&str, &[(&str, i32)]) = match id {
        "minecraft:chests/woodland_mansion" => (
            "mansion_loot",
            &[
                ("totem_of_undying", 1),
                ("diamond_axe", 3),
                ("diamond_chestplate", 2),
                ("enchanted_book", 4),
            ],
        ),
        "minecraft:chests/nether_bridge" => (
            "fortress_loot",
            &[
                ("wither_skeleton_skull", 1),
                ("diamond", 5),
                ("blaze_powder", 5),
                ("golden_sword", 3),
                ("enchanted_book", 3),
            ],
        ),
        _ => return,
    };
    if !crate::evanders_config::enabled(key) {
        return;
    }
    if let Some(pools) = table["pools"].as_array_mut() {
        let entries: Vec<_> = items.iter().map(|&(item,weight)| {
            let mut entry = serde_json::json!({"type":"minecraft:item","name":format!("minecraft:{item}"),"weight":weight});
            if item=="enchanted_book" {
                entry["name"]=serde_json::json!("minecraft:book");
                entry["modifier"]=serde_json::json!([{"type":"minecraft:enchant_randomly","options":"#minecraft:treasure"}]);
            } else if matches!(item,"diamond_axe" | "diamond_chestplate" | "golden_sword") {
                entry["modifier"] = serde_json::json!([{"type":"minecraft:enchant_randomly"}]);
            }
            entry
        }).collect();
        pools.push(serde_json::json!({"rolls":1,"entries":entries}));
    }
}
