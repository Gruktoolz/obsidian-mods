fn hook(game: &mut crate::game::Game<'_>, start: bool) -> core::ops::ControlFlow<()> {
    if game.creative && game.pick().is_some_and(|hit| game.sc_rune_at(hit.pos)) {
        // Take the normal progressive digging path for this click only.
        game.creative = false;
        game.remote_mining(start);
        game.creative = true;
        return core::ops::ControlFlow::Break(());
    }
    core::ops::ControlFlow::Continue(())
}
