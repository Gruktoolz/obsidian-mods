fn hitbox_plus_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use horizon_client::official_hitbox_plus as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![[m::KEY.into(), "Toggle Hitbox+".into(), m::DEFAULT_KEY.into()]],
    });
}
