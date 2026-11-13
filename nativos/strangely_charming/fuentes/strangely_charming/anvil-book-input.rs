fn hook(menu: &Menu, slot: usize, stack: &ItemStack) -> core::ops::ControlFlow<u32> {
    if menu.kind == Kind::Anvil && slot == 0 && stack.item.as_ref() == "minecraft:enchanted_book" {
        core::ops::ControlFlow::Break(1)
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
