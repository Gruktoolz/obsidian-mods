fn hook(stack: &ItemStack) -> core::ops::ControlFlow<Option<String>> {
    if stack.item.as_ref() == "strangely_charming:raid_horn" {
        core::ops::ControlFlow::Break(Some("minecraft:dream_goat_horn".into()))
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
