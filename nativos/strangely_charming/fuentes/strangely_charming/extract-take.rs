fn hook(menu: &mut Menu, env: &Env, s: usize) -> core::ops::ControlFlow<()> {
    use crate::stations::evanders_grinding as grinding;
    if menu.slots[s].role == Role::GrindResult
        && let Some((i, item, book)) = grinding::extraction(
            menu.containers[MAIN][0].as_ref(),
            menu.containers[MAIN][1].as_ref(),
        )
    {
        let cost = grinding::cost(item, env.enchantments);
        let mut original = item.clone();
        original.set_crafting_enchantments(Vec::new());
        original.tidy();
        if original.item.as_ref() == "minecraft:enchanted_book" {
            original = original.transmute("minecraft:book", 1);
        }
        let mut left = book.clone();
        left.count -= 1;
        if !menu.creative {
            menu.level_change -= cost;
        }
        menu.put(env, i, Some(original));
        menu.put(env, 1 - i, (left.count > 0).then_some(left));
        return core::ops::ControlFlow::Break(());
    }
    core::ops::ControlFlow::Continue(())
}
