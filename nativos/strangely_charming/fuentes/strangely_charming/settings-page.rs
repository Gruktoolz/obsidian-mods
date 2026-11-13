fn hook(mods: &mut Vec<NativeMod>) {
    let keys = [
        ("scrollkeepers","Open Quest Journal","key.keyboard.j"),
        ("fullbright", "Toggle Fullbright", "key.keyboard.g"),
        ("portable_crafting", "Portable Crafting", "key.keyboard.v"),
        ("inventory_tidying", "Tidy Backpack", "key.keyboard.y"),
    ]
    .into_iter()
    .filter(|(key, _, _)| horizon_mods::evanders_config::enabled(key))
    .map(|(id, label, key)| [id.into(), label.into(), key.into()])
    .collect();
    mods.push(NativeMod {
        id: "strangely_charming".into(),
        title: "Strangely Charming 0.2.11".into(),
        description: "Gameplay tweaks".into(),
        keys,
    });
}
