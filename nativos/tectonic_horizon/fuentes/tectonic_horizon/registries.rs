fn registries(overrides:&WorldgenOverrides,bundle:&str,base:&mut BTreeMap<String,String>) {
    #[allow(unused_imports)]
    use tectonic_selection::TectonicWorldSelection as _;

    let enabled=overrides.mod_enabled("tectonic_horizon");
    tectonic_horizon::set_active(enabled);
    if enabled {tectonic_horizon::populate(bundle,base,tectonic_horizon::config());}
}
