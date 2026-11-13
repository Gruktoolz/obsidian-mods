// A block changed, whoever changed it: a stone's entry and its other block.
fn wayfarer_stones_block_changed(
    level: &crate::game::GameLevel<'_>,
    dimension: crate::game::LevelDimension,
    pos: horizon_core::position::BlockPos,
    old: horizon_world::registry::BlockStateId,
    new: horizon_world::registry::BlockStateId,
) {
    crate::game::wayfarer_stones_glue::block_changed(level, dimension, pos, old, new);
}
