// The FOV hook knows whether the player flies in creative and whether they
// draw a bow or look through a spyglass: keep both. Speed FOV widens the view
// past walking speed.
fn living_camera_fov(
    game: &crate::game::Game<'_>,
    player: &horizon_client::camera::FovPlayer,
    modifier: &mut f32,
) {
    use crate::game::living_camera as m;
    use horizon_client::camera::UseItem;
    if !game.native_on(m::ID) {
        return;
    }
    let aiming = matches!(player.using, Some((UseItem::Bow | UseItem::Spyglass, _)));
    m::set_player(player.flying, aiming);
    *modifier *= m::pose().fov;
}
