use std::path::Path;

pub fn read(data: &Path) -> Vec<(String, Vec<u8>)> {
    let Ok(folder) = crate::json_folder::read(&data.join("strangely_charming/recipe")) else {
        return Vec::new();
    };
    folder
        .files
        .iter()
        .filter_map(|file| {
            Some((
                format!("strangely_charming:{}", file.name),
                file.bytes().ok()?.to_vec(),
            ))
        })
        .collect()
}

pub fn load(
    root: &std::path::Path,
    overlay: &horizon_world::loot::PackOverlay,
    original: impl FnOnce(
        &std::path::Path,
        &horizon_world::loot::PackOverlay
    ) -> crate::recipe_book::RecipeIndex,
) -> crate::recipe_book::RecipeIndex {
    use horizon_world::loot::PackOverlay;
    let mut files: Vec<_> = read(&root.join("data"))
        .into_iter()
        .filter(|(id, _)| !overlay.kind("recipe").any(|(key, _)| key == id))
        .map(|(id, bytes)| (format!("recipe/{id}"), vec![bytes]))
        .collect();
    if files.is_empty() {
        return original(root, overlay);
    }
    files.extend_from_slice(overlay.files);
    original(root, &PackOverlay { files: &files })
}
