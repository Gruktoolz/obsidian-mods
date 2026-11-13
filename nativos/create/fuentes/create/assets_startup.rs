fn create_assets_startup(root: &mut std::path::PathBuf) {
    *root = crate::game::create_port::asset_root(root.clone());
}
