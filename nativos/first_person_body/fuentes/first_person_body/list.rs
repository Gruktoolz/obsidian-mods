// The mod's row in Options > Mods, with its toggle key.
fn first_person_body_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use horizon_client::mobs::first_person_body as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![[
            m::TOGGLE_KEY.into(),
            m::TOGGLE_KEY_LABEL.into(),
            m::TOGGLE_KEY_DEFAULT.into(),
        ]],
    });
}
