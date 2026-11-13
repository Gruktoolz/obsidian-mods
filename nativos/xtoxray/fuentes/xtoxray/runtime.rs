// X To Xray: while on, the mesher reads tables where every block outside
// `VISIBLE` is air and the rest are drawn at full brightness. This file is both the mod's Cargo library (its tests) and the
// module the bundle adds to the mesher, where the game's hooks reach it.
use std::sync::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};

pub const ID: &str = "xtoxray";
pub const VERSION: &str = "2026.29.0";
pub const DESCRIPTION: &str = "Press X to see ores through the world. Single player only.";
pub const KEY: &str = "toggle";
pub const DEFAULT_KEY: &str = "key.keyboard.x";

const VISIBLE: [&str; 24] = [
    "coal_ore",
    "deepslate_coal_ore",
    "iron_ore",
    "deepslate_iron_ore",
    "copper_ore",
    "deepslate_copper_ore",
    "gold_ore",
    "deepslate_gold_ore",
    "redstone_ore",
    "deepslate_redstone_ore",
    "emerald_ore",
    "deepslate_emerald_ore",
    "lapis_ore",
    "deepslate_lapis_ore",
    "diamond_ore",
    "deepslate_diamond_ore",
    "nether_gold_ore",
    "nether_quartz_ore",
    "ancient_debris",
    "chest",
    "trapped_chest",
    "spawner",
    "water",
    "lava",
];

static ACTIVE: AtomicBool = AtomicBool::new(false);
/// Per block state: whether it stays visible.
static MASK: RwLock<Vec<bool>> = RwLock::new(Vec::new());

pub fn active() -> bool {
    ACTIVE.load(Ordering::Relaxed)
}

pub fn set_active(on: bool) {
    ACTIVE.store(on, Ordering::Relaxed);
}

/// Whether a registry name (`minecraft:diamond_ore`) stays visible.
pub fn visible(name: &str) -> bool {
    VISIBLE.contains(&name.strip_prefix("minecraft:").unwrap_or(name))
}

pub fn set_states<'a>(names: impl Iterator<Item = &'a str>) {
    *MASK.write().unwrap() = names.map(visible).collect();
}

/// Whether a block state stays visible. A state outside the table stays: never
/// hide what we can't name.
pub fn state_visible(state: usize) -> bool {
    MASK.read().unwrap().get(state).copied().unwrap_or(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_listed_blocks_are_visible() {
        assert!(visible("minecraft:diamond_ore"));
        assert!(visible("water"));
        assert!(!visible("minecraft:stone"));
    }
}
