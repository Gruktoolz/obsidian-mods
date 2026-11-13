fn set_seed_crop(row: &mut J, rename: &Rename) {
    if rename.to == "inferium_lab:inferium_seeds"
        && let Some(block) = row.at("fields/block")
        && block.as_str() == Some("minecraft:wheat")
    {
        *block = J::Str("inferium_lab:inferium_crop".into());
    }
}
