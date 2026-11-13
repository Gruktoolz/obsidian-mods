fn hook(name: &str) -> Option<FurnaceKind> {
    match name {
        "strangely_charming:kiln" | "minecraft:furnace" => Some(FurnaceKind::Furnace),
        "minecraft:blast_furnace" => Some(FurnaceKind::BlastFurnace),
        "minecraft:smoker" => Some(FurnaceKind::Smoker),
        _ => None,
    }
}
