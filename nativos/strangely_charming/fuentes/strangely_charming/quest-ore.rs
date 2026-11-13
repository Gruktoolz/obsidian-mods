fn hook(game: &mut crate::game::Game<'_>, ty: &str, key: &str, amount: i32) {
    if ty == "mined"
        && matches!(key, "minecraft:coal_ore" | "minecraft:deepslate_coal_ore")
        && amount > 0
    {
        game.sc_event(crate::game::sc_quests::Kind::Coal, amount as u32);
    }
}
