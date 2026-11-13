// A block is about to be placed: a stone needs room for its upper block.
fn wayfarer_stones_state_for_placement(
    level: &crate::game::GameLevel<'_>,
    state: horizon_world::registry::BlockStateId,
    context: &horizon_world::level::PlacementContext,
    answer: &mut Option<Option<horizon_world::registry::BlockStateId>>,
) {
    crate::game::wayfarer_stones_glue::state_for_placement(level, state, context, answer);
}
