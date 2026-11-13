// Hide only the crosshair while following. Never override another mod's cancellation.
fn freecam_gui_layer_pre(game: &crate::game::Game<'_>, layer: &str, canceled: &mut bool) {
    use crate::game::freecam_yannis as m;
    if layer == "minecraft:crosshair"
        && game.native_on(m::ID)
        && game.native_player().is_some()
        && m::STATE.with(|s| s.borrow().mode == m::Mode::Follow)
    {
        *canceled = true;
    }
}
