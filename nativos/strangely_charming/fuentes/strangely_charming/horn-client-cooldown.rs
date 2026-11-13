fn hook(value: i32, held: &ItemStack) -> i32 {
    if held.item.as_ref() == "strangely_charming:raid_horn" {
        20 * 60 * 10
    } else {
        value
    }
}
