// Register Fullbright in Options > Mods with its configurable keybind.
fn fullbright_list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    crate::game::mod_fullbright_glue::on_list(mods);
}
