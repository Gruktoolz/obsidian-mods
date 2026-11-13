// The mouse on the Mods screen and on native mod settings pages.
fn mod_menu_options_mouse(
    game: &mut crate::game::Game<'_>,
    pressed: bool,
    right: bool,
) -> ::core::ops::ControlFlow<Result<()>> {
    use horizon_client::options_ui::Page;
    
    // If the Mods screen is open, handle clicks there.
    if horizon_client::title::native_mod_menu_title::is_open() {
        return ::core::ops::ControlFlow::Break(crate::game::native_mod_menu_ui::mouse(game, pressed, right));
    }
    
    // If we're on a mod's settings page and the user clicks the Done button,
    // return to the Mods screen instead of closing Options entirely.
    if pressed && !right {
        if let Page::NativeMod(_) = game.ui.options.page() {
            let (w, h) = game.gui_size();
            let (x, y) = game.cursor;
            let cx = w / 2.0;
            // Done button on NativeMod page: cx + 4.0, h - 27.0, width 150.0, height 20.0
            let done_x = cx + 4.0;
            let done_y = h - 27.0;
            let done_w = 150.0;
            let done_h = 20.0;
            
            if x >= done_x && x <= done_x + done_w && y >= done_y && y <= done_y + done_h {
                horizon_client::title::native_mod_menu_title::open();
                return ::core::ops::ControlFlow::Break(Ok(()));
            }
        }
    }
    
    ::core::ops::ControlFlow::Continue(())
}
