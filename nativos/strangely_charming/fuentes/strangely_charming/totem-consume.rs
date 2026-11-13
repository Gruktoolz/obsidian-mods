fn hook(game: &mut crate::game::Game<'_>) -> core::ops::ControlFlow<Option<Body>> {
    if !game.player.as_ref().is_some_and(|p| p.health.totem_used) {
        return core::ops::ControlFlow::Continue(());
    }
    let Some(slot) = strangely_charming_totem::fallback_slot(game) else {
        return core::ops::ControlFlow::Continue(());
    };
    // The occupied offhand disables this fallback during the native call. Horizon
    // consumes the real stack, applies its components and constructs its animation.
    std::mem::swap(&mut game.offhand, &mut game.inventory[slot]);
    let body = game.apply_totem();
    std::mem::swap(&mut game.offhand, &mut game.inventory[slot]);
    core::ops::ControlFlow::Break(body)
}
