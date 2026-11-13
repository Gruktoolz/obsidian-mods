use std::{collections::BTreeMap, fs, io, path::Path, sync::OnceLock};
const FEATURES: &[&str] = &[
    "rune_circles",
    "underground_ruins",
    "runebreaker",
    "moonstones",
    "enchanting_totems",
    "scrollkeepers",
    "coral_squids",
    "lumberjacks",
    "beekeepers",
    "ender_pearl_blocks",
    "kilns",
    "exploration_advancements",
    "extract_enchantments",
    "mansion_loot",
    "fortress_loot",
    "mineshaft_ores",
    "raid_horns",
    "fullbright",
    "poison_apple",
    "portable_crafting",
    "extra_recipes",
    "stackable_potions",
    "stackable_stews",
    "stackable_enchanted_books",
    "unlimited_anvil",
    "inventory_tidying",
    "protect_pets",
    "inventory_totem",
    "grindable_armor",
    "crying_obsidian_frames",
];
static CONFIG: OnceLock<BTreeMap<String, bool>> = OnceLock::new();

fn load() -> io::Result<BTreeMap<String, bool>> {
    let path = Path::new("config/strangely_charming.json");
    let old_path = Path::new("config/evanders_tweaks.json");
    let migrate = !path.exists() && old_path.exists();
    let mut values: BTreeMap<String, bool> = match fs::read(if migrate { old_path } else { path }) {
        Ok(bytes) => serde_json::from_slice(&bytes).map_err(io::Error::other)?,
        Err(e) if e.kind() == io::ErrorKind::NotFound => BTreeMap::new(),
        Err(e) => return Err(e),
    };
    let mut changed = migrate | values.remove("bundle_recipe").is_some();
    changed |= values.remove("mooblooms").is_some();
    for &key in FEATURES {
        if !values.contains_key(key) {
            values.insert(key.to_owned(), true);
            changed = true;
        }
    }
    if changed {
        fs::create_dir_all(path.parent().unwrap())?;
        let temp = path.with_extension(format!(
            "{}-{}.tmp",
            std::process::id(),
            env!("CARGO_PKG_NAME")
        ));
        fs::write(&temp, serde_json::to_string_pretty(&values)? + "\n")?;
        fs::rename(temp, path)?;
    }
    Ok(values)
}
pub fn enabled(key: &str) -> bool {
    CONFIG.get_or_init(|| load().unwrap_or_else(|err| {
        eprintln!("Strangely Charming could not read its config: {err}. Using defaults for this session.");
        BTreeMap::new()
    })).get(key).copied().unwrap_or(true)
}
