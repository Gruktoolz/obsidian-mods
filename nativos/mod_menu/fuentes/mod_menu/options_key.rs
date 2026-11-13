// Keys on the Mods screen and on native mod settings pages.
fn mod_menu_options_key(
    game: &mut crate::game::Game<'_>,
    key: KeyCode,
    text: Option<&str>,
) -> ::core::ops::ControlFlow<Result<()>> {
    use horizon_client::options_ui::Page;
    
    // If the Mods screen is open, handle keys there.
    if horizon_client::title::native_mod_menu_title::is_open() {
        return ::core::ops::ControlFlow::Break(crate::game::native_mod_menu_ui::key(game, key, text));
    }
    
    // If we're on a mod's settings page and the user presses Escape or Enter,
    // return to the Mods screen instead of closing Options entirely.
    if matches!(key, KeyCode::Escape | KeyCode::Enter | KeyCode::NumpadEnter) {
        if let Page::NativeMod(_) = game.ui.options.page() {
            horizon_client::title::native_mod_menu_title::open();
            return ::core::ops::ControlFlow::Break(Ok(()));
        }
    }
    
    ::core::ops::ControlFlow::Continue(())
}
