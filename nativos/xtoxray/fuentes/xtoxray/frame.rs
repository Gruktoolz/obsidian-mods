// Off as soon as this game is no longer a closed single player world.
fn xtoxray_frame(game: &mut crate::game::Game<'_>) {
    use horizon_client_render::mesher::xtoxray;
    if xtoxray::active()
        && game.native_window()
        && (game.integrated.is_none() || game.published_port().is_some())
    {
        xtoxray::set_active(false);
        game.all_sections_changed();
    }
}
