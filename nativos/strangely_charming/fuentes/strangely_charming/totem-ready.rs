fn hook(game: &mut crate::game::Game<'_>) -> core::ops::ControlFlow<()> {
    if strangely_charming_totem::fallback_slot(game).is_none() {
        return core::ops::ControlFlow::Continue(());
    }
    if let Some(p) = game.player.as_mut() {
        p.health.totem_ready = true;
    }
    core::ops::ControlFlow::Break(())
}
