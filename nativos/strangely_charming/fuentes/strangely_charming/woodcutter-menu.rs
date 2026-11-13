fn hook(name: &str) -> Option<Kind> {
    Some(match name {
        "minecraft:enchanting_table" => Kind::Enchanting,
        "minecraft:anvil" | "minecraft:chipped_anvil" | "minecraft:damaged_anvil" => Kind::Anvil,
        "minecraft:grindstone" => Kind::Grindstone,
        "minecraft:smithing_table" => Kind::Smithing,
        "strangely_charming:woodcutter" if horizon_mods::evanders_config::enabled("lumberjacks") => {
            Kind::Stonecutter
        }
        "minecraft:stonecutter" => Kind::Stonecutter,
        "minecraft:beacon" => Kind::Beacon,
        "minecraft:cartography_table" => Kind::Cartography,
        "minecraft:loom" => Kind::Loom,
        _ => return None,
    })
}
