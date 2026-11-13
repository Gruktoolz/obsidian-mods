fn hook(root: &Path, document: &EntityModelDocument) -> crate::Result<MobTypes> {
    evanders_models::load(root, true, Some(document))
}
