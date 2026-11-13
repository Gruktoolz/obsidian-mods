// Hook helper: native_mods
fn light_overlay_list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    crate::game::mod_light_overlay_glue::on_list(mods);
}
