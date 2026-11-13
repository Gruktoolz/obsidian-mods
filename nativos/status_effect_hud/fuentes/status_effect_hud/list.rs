// Hook helper: native_mods
fn status_effect_hud_list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    crate::game::mod_status_effect_hud_glue::on_list(mods);
}
