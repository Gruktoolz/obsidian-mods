// The box: effects, then what the player carries, then what is wearing out.
fn basic_hud_draw(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    let _ = corners;
    use crate::game::{basic_hud as m, basic_hud_editor as editor, basic_hud_render as render};

    if !game.native_on(m::ID) || game.player.is_none() || editor::active() {
        return;
    }
    let config = m::config(|| game.native_setting(m::ID, "config"), m::Config::clone);
    render::draw(game, batch, &config, false);
}
