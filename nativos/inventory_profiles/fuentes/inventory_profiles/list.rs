// Hook helper: native_mods
fn inventory_profiles_list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    crate::game::mod_inventory_profiles_glue::on_list(mods);
}
