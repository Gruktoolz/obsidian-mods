// A piston won't part a stone's two blocks.
fn wayfarer_stones_piston_move(
    level: &crate::game::GameLevel<'_>,
    push: &[horizon_core::position::BlockPos],
    destroy: &[horizon_core::position::BlockPos],
    canceled: &mut bool,
) {
    crate::game::wayfarer_stones_glue::piston_move(level, push, destroy, canceled);
}
