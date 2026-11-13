fn hook(root: &Path) -> crate::Result<MobTypes> {
    evanders_models::load(root, false, None)
}
