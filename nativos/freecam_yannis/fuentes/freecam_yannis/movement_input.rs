fn freecam_movement_input(
    game: &crate::game::Game<'_>,
    forward: &mut f32,
    strafe: &mut f32,
    jump: &mut bool,
    sneak: &mut bool,
) {
    use crate::game::freecam_yannis as m;

    if game.native_on(m::ID) && m::detached() {
        *forward = 0.0;
        *strafe = 0.0;
        *jump = false;
        *sneak = false;
    }
}
