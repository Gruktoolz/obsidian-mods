// Intercepts keyboard input while the HUD editor is active.
fn basic_hud_options_key(
    game: &mut crate::game::Game<'_>,
    key: winit::keyboard::KeyCode,
    text: Option<&str>,
) -> std::ops::ControlFlow<Result<()>> {
    use crate::game::{basic_hud as m, basic_hud_editor as editor};

    if !editor::active() {
        return std::ops::ControlFlow::Continue(());
    }
    if !game.native_on(m::ID) {
        editor::close(game);
        return std::ops::ControlFlow::Break(Ok(()));
    }
    if editor::key(game, key, text) {
        std::ops::ControlFlow::Break(Ok(()))
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
