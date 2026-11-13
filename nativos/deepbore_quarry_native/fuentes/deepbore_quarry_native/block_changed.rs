fn deepbore_quarry_block_changed(
    level: &crate::game::GameLevel<'_>,
    dimension: crate::game::LevelDimension,
    pos: horizon_core::position::BlockPos,
    old: horizon_world::registry::BlockStateId,
    new: horizon_world::registry::BlockStateId,
) {
    crate::game::deepbore_quarry_glue::block_changed(level, dimension, pos, old, new);
}
