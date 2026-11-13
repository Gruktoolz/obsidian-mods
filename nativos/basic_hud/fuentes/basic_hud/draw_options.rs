// Injects into Game::draw_options to render the HUD editor over the game.
fn basic_hud_draw_options(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
) -> std::ops::ControlFlow<()> {
    use crate::game::{basic_hud as m, basic_hud_editor as editor};

    // Label collection may run during registration; only an actual visit
    // to this mod's settings may open its first-use guide.
    let hud_page = match game.ui.options.page() {
        horizon_client::options_ui::Page::NativeMod(i) => game.settings.mods.installed.get(i)
            .is_some_and(|installed| installed.id == m::ID),
        _ => false,
    };
    if !editor::active() && hud_page && game.native_on(m::ID) && editor::guide_pending(game) {
        editor::open(game);
    }
    if !editor::active() {
        return std::ops::ControlFlow::Continue(());
    }
    if !game.native_on(m::ID) {
        editor::close(game);
        return std::ops::ControlFlow::Continue(());
    }
    if editor::draw(game, batch) {
        std::ops::ControlFlow::Break(())
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
