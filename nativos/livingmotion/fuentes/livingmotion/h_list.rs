// The mod in Options > Mods, with its key.
fn livingmotion_list(mods: &mut Vec<horizon_client::mod_api::NativeMod>) {
    crate::game::livingmotion::listed(mods);
}
