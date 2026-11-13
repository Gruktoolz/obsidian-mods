// Register AppleSkin in Options > Mods.
fn appleskin_list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    crate::game::mod_appleskin_glue::on_list(mods);
}
