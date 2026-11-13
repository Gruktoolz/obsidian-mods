// The mod's row in Options > Mods; its settings are on the row's page.
fn connected_textures_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use horizon_client_render::native_connected_textures as ct;
    mods.push(crate::game::native_hooks::NativeMod {
        id: ct::ID.into(),
        title: format!("{} {}", ct::NAME, ct::VERSION),
        description: ct::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
