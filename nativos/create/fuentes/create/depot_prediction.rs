// Create's SharedDepotBlockMethods.onUse returns SUCCESS on the client too.
// End the click after its normal UseItemOn packet, before item prediction or
// the other hand can send another exchange. Inventories remain server-owned.
fn create_depot_prediction(
    game: &crate::game::Game<'_>,
    hit: &crate::game::Hit,
    held: Option<&crate::items::ItemStack>,
) -> std::ops::ControlFlow<(InteractionResult, DryUse)> {
    if game.native_on("create")
        && held.is_some_and(|s| {
            matches!(
                s.item.as_ref(),
                "create:vertical_gearbox" | "create:large_water_wheel"
            )
        })
        || crate::game::create_port::depot_reacts(game, hit.pos, hit.face)
    {
        let bypass = game.keys.contains(&winit::keyboard::KeyCode::ShiftLeft)
            && (game
                .inventory
                .get(game.selected)
                .is_some_and(Option::is_some)
                || game.offhand.is_some());
        if game.native_block(hit.pos) == "create:hand_crank"
            && !held.is_some_and(|s| s.item.as_ref() == "create:wrench")
            && !bypass
        {
            crate::game::create_port_nativeaudio::predict_crank(game, hit.pos);
        }
        std::ops::ControlFlow::Break((InteractionResult::Success, DryUse::default()))
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
