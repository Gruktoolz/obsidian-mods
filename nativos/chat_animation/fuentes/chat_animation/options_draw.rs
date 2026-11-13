// The Options screen, while Chat Animation Config is open: the config screen in its place.
fn chat_animation_options_draw(game: &mut crate::game::Game<'_>, batch: &mut HudBatch) -> ::core::ops::ControlFlow<()> {
    use horizon_client_render::native_chat_animation as ca;
    if !ca::menu_is_open() {
        return ::core::ops::ControlFlow::Continue(());
    }
    let in_world = game.ui.options.in_world;
    ca::menu_draw(batch, &game.gui.atlas, game.cursor, in_world);
    ::core::ops::ControlFlow::Break(())
}
