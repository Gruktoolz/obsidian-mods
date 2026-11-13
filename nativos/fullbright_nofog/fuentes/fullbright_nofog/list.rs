// The mod's row in Options > Mods, with its two toggle keys.
fn fullbright_nofog_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::fullbright_nofog as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("Fullbright & NoFog {}", m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![
            [m::KEY_FULLBRIGHT.into(), "Toggle Fullbright".into(), m::DEFAULT_KEY_FULLBRIGHT.into()],
            [m::KEY_NOFOG.into(), "Toggle NoFog".into(), m::DEFAULT_KEY_NOFOG.into()],
        ],
    });
}
