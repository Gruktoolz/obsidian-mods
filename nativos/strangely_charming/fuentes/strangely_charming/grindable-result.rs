fn hook(
    input: Option<&ItemStack>,
    additional: Option<&ItemStack>,
) -> core::ops::ControlFlow<Option<ItemStack>> {
    if let Some((_, item, _)) = evanders_grinding::extraction(input, additional) {
        let mut book = ItemStack::new("minecraft:enchanted_book", 1);
        book.set_crafting_enchantments(item.crafting_enchantments().to_vec());
        return core::ops::ControlFlow::Break(Some(book));
    }
    if let (Some(s), None) | (None, Some(s)) = (input, additional) {
        if let Some(id) = evanders_grinding::material(s) {
            return core::ops::ControlFlow::Break(Some(ItemStack::new(id, 1)));
        }
    }
    core::ops::ControlFlow::Continue(())
}
