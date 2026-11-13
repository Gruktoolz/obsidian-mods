// stations::Game::stack_tooltip HEAD: every item tooltip the game draws comes
// from here; an enchanted stack's gets a hint under each enchantment, anything
// else goes on to the game's own lines.
fn enchantmenthints_tooltip(
    game: &crate::game::Game<'_>,
    stack: &horizon_client::items::ItemStack,
) -> core::ops::ControlFlow<Vec<(horizon_client::text::Text, u32)>> {
    match game.enchantmenthints_tooltip(stack) {
        Some(lines) => core::ops::ControlFlow::Break(lines),
        None => core::ops::ControlFlow::Continue(()),
    }
}
