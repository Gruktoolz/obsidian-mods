// HUD+'s row in Options > Mods.
fn official_hud_plus_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::official_hud_plus as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
