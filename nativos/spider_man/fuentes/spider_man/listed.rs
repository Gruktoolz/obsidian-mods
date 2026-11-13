// The mod's row in Options > Mods, with its keys.
fn plug_listed(game: &crate::game::Game<'_>, mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    crate::game::spider_man::listed(game, mods)
}
