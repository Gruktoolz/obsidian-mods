// Fullbright's row in Options > Mods, with its toggle key.
fn fullbright_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::fullbright_gamma as fb;
    mods.push(crate::game::native_hooks::NativeMod {
        id: fb::ID.into(),
        title: format!("Fullbright {}", fb::VERSION),
        description: fb::DESCRIPTION.into(),
        keys: vec![[fb::KEY.into(), "Toggle fullbright".into(), fb::DEFAULT_KEY.into()]],
    });
}
