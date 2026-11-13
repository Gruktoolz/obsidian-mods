// Intercepts edit shortcuts (Ctrl+A/C/V/X) while the HUD editor is active.
fn basic_hud_options_shortcut(
    game: &mut crate::game::Game<'_>,
) -> std::ops::ControlFlow<bool> {
    use crate::game::{basic_hud as m, basic_hud_editor as editor};

    if !editor::active() {
        return std::ops::ControlFlow::Continue(());
    }
    if !game.native_on(m::ID) {
        editor::close(game);
        return std::ops::ControlFlow::Break(true);
    }
    // Block stock options clipboard operations while HUD editor is active
    std::ops::ControlFlow::Break(true)
}
