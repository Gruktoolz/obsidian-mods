// Hook helper: native_mods
fn toggle_sprint_list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    crate::game::mod_toggle_sprint_glue::on_list(mods);
}
