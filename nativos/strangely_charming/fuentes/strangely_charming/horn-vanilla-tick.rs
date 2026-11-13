fn hook(value: bool, stack: &ItemStack) -> bool {
    value && stack.item.as_ref() != "strangely_charming:raid_horn"
}
