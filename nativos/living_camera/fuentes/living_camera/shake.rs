// The camera's lean, tilt, dip, bob and breath, added to the other mods'
// shakes.
fn living_camera_shake(
    game: &crate::game::Game<'_>,
    shake: &mut crate::game::native_hooks::NativeShake,
) {
    use crate::game::living_camera as m;
    if !game.native_on(m::ID) {
        return;
    }
    let pose = m::pose();
    shake.pitch += pose.pitch;
    shake.roll += pose.roll;
    shake.offset[1] += pose.rise;
}
