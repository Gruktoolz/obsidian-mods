// Freelook's row in Options > Mods, with its key.
fn freelook_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::freelook as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: "Freelook".into(),
        description: m::DESCRIPTION.into(),
        keys: vec![[m::KEY.into(), "Freelook key".into(), m::DEFAULT_KEY.into()]],
    });
}
