// Each frame: read the player's position, yaw, pitch, and held weapon from Game
// and store it in HitBox+'s globals so entity_box can do crosshair raycast targeting.
fn hitbox_plus_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    use horizon_client::official_hitbox_plus as m;
    let _ = elapsed;
    if !m::is_enabled() {
        return;
    }
    if let Some(p) = game.native_player() {
        let is_spear = game.main_hand_key().contains("spear");
        let is_creative = game.creative;
        let reach = if is_spear {
            if is_creative { 6.5 } else { 4.5 }
        } else {
            if is_creative { 5.0 } else { 3.0 }
        };
        m::update_player_aim(p.x, p.y + 1.62, p.z, p.yaw, p.pitch, reach);
    }
}
