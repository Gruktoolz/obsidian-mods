fn register(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::right_click_crops as runtime;

    if mods.iter().any(|row| row.id == runtime::MOD_ID) {
        return;
    }
    mods.push(crate::game::native_hooks::NativeMod {
        id: runtime::MOD_ID.to_owned(),
        title: format!("{} {}", runtime::MOD_NAME, runtime::MOD_VERSION),
        description: runtime::MOD_DESCRIPTION.to_owned(),
        keys: Vec::new(),
    });
}
