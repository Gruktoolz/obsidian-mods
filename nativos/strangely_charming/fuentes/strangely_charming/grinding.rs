use super::ItemStack;
pub fn material(s: &ItemStack) -> Option<&'static str> {
    if s.has_any_enchantments()
        || s.damage != 0
        || s.count != 1
        || !horizon_mods::evanders_config::enabled("grindable_armor")
    {
        return None;
    }
    let (material, slot) = s.item.strip_prefix("minecraft:")?.split_once('_')?;
    if !matches!(
        slot,
        "helmet" | "chestplate" | "leggings" | "boots" | "horse_armor"
    ) {
        return None;
    }
    match material {
        "leather" => Some("minecraft:leather"),
        "iron" | "chainmail" => Some("minecraft:iron_ingot"),
        "golden" => Some("minecraft:gold_ingot"),
        "netherite" => Some("minecraft:netherite_ingot"),
        "diamond" => Some("minecraft:diamond"),
        "copper" => Some("minecraft:copper_ingot"),
        _ => None,
    }
}

pub fn extraction<'a>(
    a: Option<&'a ItemStack>,
    b: Option<&'a ItemStack>,
) -> Option<(usize, &'a ItemStack, &'a ItemStack)> {
    if !horizon_mods::evanders_config::enabled("extract_enchantments") {
        return None;
    }
    let (a, b) = (a?, b?);
    let (i, item, book) = if a.item.as_ref() == "minecraft:book" {
        (1, b, a)
    } else {
        (0, a, b)
    };
    (book.item.as_ref() == "minecraft:book"
        && item.count == 1
        && !item.crafting_enchantments().is_empty())
    .then_some((i, item, book))
}
pub fn cost(item: &ItemStack, e: &super::Enchantments) -> i32 {
    let treasure = item
        .crafting_enchantments()
        .iter()
        .any(|(id, _)| e.in_tag("minecraft:treasure", id));
    (5u32
        .saturating_add(if treasure { 5 } else { 0 })
        .saturating_add(item.repair_cost()))
    .min(i32::MAX as u32) as i32
}
