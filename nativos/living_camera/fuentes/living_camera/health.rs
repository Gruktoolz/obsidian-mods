// The health HUD, each frame it draws: the player's health (a drop shakes the
// camera) and whether they ride a living mount. It draws nothing of its own.
fn living_camera_health(
    game: &crate::game::Game<'_>,
    inputs: &horizon_client::hud_layout::HealthInputs,
) {
    use crate::game::living_camera as m;
    if game.native_on(m::ID) {
        m::health(inputs.health, inputs.vehicle_hearts > 0);
    }
}
