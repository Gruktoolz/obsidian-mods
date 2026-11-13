// The mod's row in Options > Mods.
fn wayfarer_stones_listed(mods: &mut Vec<horizon_client::mod_api::NativeMod>) {
    crate::game::wayfarer_stones_glue::listed(mods);
}
