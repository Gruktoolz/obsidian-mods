fn hook(
    root: &std::path::Path,
    overlay: &horizon_world::loot::PackOverlay,
    original: impl FnOnce(
        &std::path::Path,
        &horizon_world::loot::PackOverlay
    ) -> crate::recipe_book::RecipeIndex,
) -> crate::recipe_book::RecipeIndex {
    crate::recipe_book::evanders_recipe_files::load(root, overlay, original)
}
