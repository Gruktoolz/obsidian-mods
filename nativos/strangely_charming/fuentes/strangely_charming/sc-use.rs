fn hook(game: &mut crate::game::Game<'_>, stack: &ItemStack, claimed: &mut bool) {
    if !*claimed && game.remote.is_none() {
        *claimed = game.sc_use_item(stack);
    }
}
