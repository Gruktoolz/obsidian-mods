fn hook(root: &Path) -> crate::Result<MobTypes> {
    evanders_models::load(root, true, None)
}
