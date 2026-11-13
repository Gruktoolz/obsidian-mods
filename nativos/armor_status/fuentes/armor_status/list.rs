// Register Armor Status in Options > Mods.
fn armor_status_list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    crate::game::mod_armor_status_glue::on_list(mods);
}
