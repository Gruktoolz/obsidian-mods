// While the camera flies free, the wheel changes its speed instead of the
// hotbar slot.
fn freecam_wheel(game: &crate::game::Game<'_>, steps: f32, claimed: &mut bool) {
    use crate::game::fair_freecam as fc;
    if !*claimed && game.native_on(fc::ID) && fc::wheel(steps) {
        *claimed = true;
    }
}
