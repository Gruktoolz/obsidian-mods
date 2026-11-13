// native_hooks::Game::native_mods (a hook point): the row in Options > Mods.
fn enchantmenthints_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::enchantmenthints as eh;
    mods.push(crate::game::native_hooks::NativeMod {
        id: eh::ID.into(),
        title: format!("{} {}", eh::NAME, eh::VERSION),
        description: eh::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
