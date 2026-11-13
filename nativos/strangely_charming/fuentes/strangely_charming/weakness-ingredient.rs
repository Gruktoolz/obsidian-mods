fn hook(kind: &str, items: &mut Vec<String>) {
 if kind == "strangely_charming:weakness_potion" { items.extend(["minecraft:potion", "minecraft:splash_potion", "minecraft:lingering_potion"].map(str::to_owned)); }
}
