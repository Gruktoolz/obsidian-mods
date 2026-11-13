// The first-person hand lags behind the view and leans with the camera; the
// off hand mirrors the main one.
fn living_camera_hand(
    game: &crate::game::Game<'_>,
    hand: horizon_client::first_person::Hand,
    x: &mut f32,
    y: &mut f32,
    roll: &mut f32,
) {
    use crate::game::living_camera as m;
    if !game.native_on(m::ID) {
        return;
    }
    let pose = m::pose();
    let side = if hand == horizon_client::first_person::Hand::Main { 1.0 } else { -1.0 };
    *x += pose.hand[0];
    *y += pose.hand[1];
    *roll += pose.hand_roll * side;
}
