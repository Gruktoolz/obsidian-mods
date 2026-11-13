fn hook(value: i32, stack: &ItemStack) -> i32 {
    if stack.item.as_ref() == "strangely_charming:raid_horn" {
        20 * 60 * 10
    } else {
        value
    }
}
