// The mod's row in Options > Mods; its settings are on the row's page too.
fn chat_animation_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use horizon_client_render::native_chat_animation as ca;
    mods.push(crate::game::native_hooks::NativeMod {
        id: ca::ID.into(),
        title: format!("{} {}", ca::NAME, ca::VERSION),
        description: ca::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
