// After vanilla posed a player: the rig's arms, legs and head.
fn livingmotion_pose(value: (), p: &mut crate::mobs::anim::Poses) {
    crate::mobs::livingmotion::pose(p);
    value
}
