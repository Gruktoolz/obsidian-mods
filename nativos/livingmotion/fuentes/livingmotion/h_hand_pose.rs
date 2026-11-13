// Your hands in first person: the rig's sway, bob, swings, chewing and inspect.
fn livingmotion_hand_pose(
    game: &crate::game::Game<'_>,
    hand: horizon_client::first_person::Hand,
    stack: Option<&horizon_client::items::ItemStack>,
    partial: f32,
    swing: f32,
    x: &mut f32,
    y: &mut f32,
    z: &mut f32,
    pitch: &mut f32,
    yaw: &mut f32,
    roll: &mut f32,
) {
    crate::game::livingmotion::hand_pose(game, hand, stack.is_some(), partial, swing, x, y, z, pitch, yaw, roll);
}
