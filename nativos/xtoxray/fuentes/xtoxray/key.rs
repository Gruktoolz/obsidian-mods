// The key toggles the xray, in single player only.
fn xtoxray_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use horizon_client_render::mesher::xtoxray;
    if !pressed
        || !game.native_window()
        || !game.native_on(xtoxray::ID)
        || !game.native_playing()
        || key != game.native_mod_key(xtoxray::ID, xtoxray::KEY, xtoxray::DEFAULT_KEY)
    {
        return;
    }
    if game.integrated.is_none() || game.published_port().is_some() {
        game.native_chat("X To Xray only works in single player!");
        return;
    }
    xtoxray::set_active(!xtoxray::active());
    game.all_sections_changed();
}
