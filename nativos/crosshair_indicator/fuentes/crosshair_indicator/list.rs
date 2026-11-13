// Hook helper: native_mods
fn crosshair_indicator_list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    crate::game::mod_crosshair_indicator_glue::on_list(mods);
}
