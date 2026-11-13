fn hook(game: &mut crate::game::Game<'_>, stack: &ItemStack) {
    if stack.item.as_ref() == "minecraft:bread" {
        game.sc_event(crate::game::sc_quests::Kind::Bread, stack.count);
    }
}
