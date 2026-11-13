fn hook(item: &str, components: &mut Map<String, Value>) {
    let (feature, size) = match item {
        "minecraft:potion" | "minecraft:splash_potion" | "minecraft:lingering_potion" => {
            ("stackable_potions", 16)
        }
        "minecraft:enchanted_book" => ("stackable_enchanted_books", 16),
        "minecraft:mushroom_stew" | "minecraft:rabbit_stew" | "minecraft:beetroot_soup" => {
            ("stackable_stews", 16)
        }
        _ => return,
    };
    if crate::evanders_config::enabled(feature) {
        components.insert("minecraft:max_stack_size".into(), Value::from(size));
    }
}
