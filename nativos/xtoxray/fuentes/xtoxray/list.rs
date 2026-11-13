// The mod's row in Options > Mods, with its key.
fn xtoxray_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use horizon_client_render::mesher::xtoxray;
    mods.push(crate::game::native_hooks::NativeMod {
        id: xtoxray::ID.into(),
        title: format!("X To Xray {}", xtoxray::VERSION),
        description: xtoxray::DESCRIPTION.into(),
        keys: vec![[xtoxray::KEY.into(), "Toggle xray".into(), xtoxray::DEFAULT_KEY.into()]],
    });
}
