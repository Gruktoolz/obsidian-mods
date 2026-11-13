fn hook(game: &mut crate::game::Game<'_>, mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    let _ = game;
    mods.push(crate::game::native_hooks::NativeMod {
        id: "server_browser".into(),
        title: "Server Atlas 2.2.1".into(),
        description: "Native server browser: search, filters, favorites, LAN and public discovery."
            .into(),
        keys: Vec::new(),
    });
}
