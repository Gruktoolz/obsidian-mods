fn deepbore_quarry_quick_move(
    game: &crate::game::Game<'_>,
    entity: &mut horizon_client::containers::ModBlockEntity,
    container_slots: usize,
    slot: usize,
    stack: &mut horizon_client::items::ItemStack,
    handled: &mut bool,
) {
    crate::game::deepbore_quarry_glue::quick_move(
        game,
        entity,
        container_slots,
        slot,
        stack,
        handled,
    );
}
