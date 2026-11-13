fn hook(value: bool, stack: &ItemStack) -> bool {
    value
        || (stack.item.as_ref() == "minecraft:book"
            && horizon_mods::evanders_config::enabled("extract_enchantments"))
        || evanders_grinding::material(stack).is_some()
}
