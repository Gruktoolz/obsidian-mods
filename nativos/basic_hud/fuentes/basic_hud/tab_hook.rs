// Injects into Game::draw_tab_list to render the enhanced Tab list.
fn basic_hud_draw_tab_list(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
) -> std::ops::ControlFlow<()> {
    use crate::game::{basic_hud as m, basic_hud_tab as tab};

    if !game.native_on(m::ID) {
        return std::ops::ControlFlow::Continue(());
    }
    let key = game.screen == crate::game::Screen::Playing
        && game.keys.contains(&winit::keyboard::KeyCode::Tab);
    if !key {
        return std::ops::ControlFlow::Continue(());
    }
    let config = m::config(|| game.native_setting(m::ID, "config"), m::Config::clone);
    if !config.tab.enabled {
        return std::ops::ControlFlow::Continue(());
    }

    if tab::draw(game, batch, &config, false) {
        std::ops::ControlFlow::Break(())
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
