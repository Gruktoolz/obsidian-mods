fn activate(overrides: &WorldgenOverrides) {
    #[allow(unused_imports)]
    use horizon_world::worldgen::overrides::tectonic_selection::TectonicWorldSelection as _;
    use horizon_world::worldgen::overrides::tectonic_horizon;
    let previous = tectonic_horizon::active();
    let menu = crate::mod_list::ModList::load(&crate::logging::game_home());
    tectonic_horizon::set_active(!menu.disabled.contains("tectonic_horizon"));
    let enabled = overrides.mod_enabled("tectonic_horizon");
    tectonic_horizon::set_active(enabled);
    // Stock upstream's cache key has no mod flags. Invalidate it when its live switch changes.
    if previous != enabled {
        forget_shared_data();
    }
}
