// While the key is held, the wheel moves the camera instead of the hotbar.
fn freelook_wheel(game: &crate::game::Game<'_>, steps: f32, claimed: &mut bool) {
    use crate::game::freelook as m;
    if game.native_on(m::ID) && m::wheel(steps) {
        *claimed = true;
    }
}
