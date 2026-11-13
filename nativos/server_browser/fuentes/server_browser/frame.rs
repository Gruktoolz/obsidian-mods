fn hook(game: &mut crate::game::Game<'_>) {
    if !game.native_window() {
        return;
    }
    if let Some(title) = &mut game.title {
        let on = !game.settings.mods.disabled.contains("server_browser");
        horizon_client::server_browser::native::sync(title, on);
    } else {
        horizon_client::server_browser::native::close();
    }
    if let Some((address, lan)) = horizon_client::server_browser::native::take_connect()
        && let Err(error) =
            game.connect_server_with_options(&address, &game.options.player_name(), None, lan)
    {
        log::warn!("Server Browser: {error}");
    }
}
