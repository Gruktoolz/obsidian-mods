fn hook(
    game: &mut crate::game::Game<'_>,
    line: &str,
) -> std::ops::ControlFlow<Result<Option<Value>>> {
    if game.title.is_some() && game.screen == crate::game::Screen::Playing {
        let words: Vec<_> = line.split_whitespace().collect();
        if words.first() == Some(&"key")
            && let Some(code) = words.get(1).and_then(|s| key_code(s))
        {
            let action = {
                let Some(title) = game.title.as_mut() else {
                    return std::ops::ControlFlow::Break(Err("title closed".into()));
                };
                horizon_client::server_browser::native::control_key(
                    title,
                    code,
                    words.get(2).copied().unwrap_or("tap"),
                    &mut game.clipboard,
                )
            };
            if let horizon_client::title::Action::Connect { address, lan } = action
                && let Err(error) = game.connect_server_with_options(
                    &address,
                    &game.options.player_name(),
                    None,
                    lan,
                )
            {
                return std::ops::ControlFlow::Break(Err(error));
            }
            return std::ops::ControlFlow::Break(Ok(Some(serde_json::json!({"ok":true}))));
        }
        if let Some(text) = line.strip_prefix("type ") {
            if let Some(title) = game.title.as_mut() {
                title.type_text(text);
            }
            return std::ops::ControlFlow::Break(Ok(Some(serde_json::json!({"ok":true}))));
        }
    }
    std::ops::ControlFlow::Continue(())
}
