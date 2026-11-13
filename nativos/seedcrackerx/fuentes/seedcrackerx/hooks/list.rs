// SeedCrackerX's row in Options > Mods.
fn scx_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    let (id, title, description) = crate::game::scx_game::listed();
    mods.push(crate::game::native_hooks::NativeMod { id, title, description, keys: Vec::new() });
}
