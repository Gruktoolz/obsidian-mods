// Fair Freecam's row in Options > Mods, with its key.
fn freecam_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::fair_freecam as fc;
    mods.push(crate::game::native_hooks::NativeMod {
        id: fc::ID.into(),
        title: format!("Fair Freecam {}", fc::VERSION),
        description: fc::DESCRIPTION.into(),
        keys: vec![[fc::KEY.into(), "Toggle freecam".into(), fc::DEFAULT_KEY.into()]],
    });
}
