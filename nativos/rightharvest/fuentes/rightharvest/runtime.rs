// Right-click harvest: which blocks are ripe crops and what sows them back.
// This file is both the standalone library and the module injected into
// `Game`; the game half lives in `harvest.rs`.

// The mod's row in Options > Mods.
pub const ID: &str = "rightharvest";
pub const NAME: &str = "RightClickHarvest";
pub const VERSION: &str = "0.1.1";
pub const DESCRIPTION: &str = "Right-click a ripe crop: it is picked and sown again.";

/// A crop the mod sows again: its highest `age`, and the item that plants it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Crop {
    pub max_age: u8,
    pub seed: &'static str,
}

/// The crop `name`, or None for anything else (a chest, a bush, air).
pub fn crop(name: &str) -> Option<Crop> {
    Some(match name {
        "minecraft:wheat" => Crop {
            max_age: 7,
            seed: "minecraft:wheat_seeds",
        },
        "minecraft:carrots" => Crop {
            max_age: 7,
            seed: "minecraft:carrot",
        },
        "minecraft:potatoes" => Crop {
            max_age: 7,
            seed: "minecraft:potato",
        },
        "minecraft:beetroots" => Crop {
            max_age: 7,
            seed: "minecraft:beetroot_seeds",
        },
        "minecraft:nether_wart" => Crop {
            max_age: 3,
            seed: "minecraft:nether_wart",
        },
        _ => return None,
    })
}

/// The seed that sows `name` back, when its `age` property is the crop's
/// highest (a lower age is still growing; no property is not a crop).
pub fn ripe_seed(name: &str, age: Option<&str>) -> Option<&'static str> {
    let Crop { max_age, seed } = crop(name)?;
    (age?.parse::<u8>().ok()? == max_age).then_some(seed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_crop_is_ripe_at_its_own_highest_age() {
        for (name, max_age, seed) in [
            ("minecraft:wheat", 7, "minecraft:wheat_seeds"),
            ("minecraft:carrots", 7, "minecraft:carrot"),
            ("minecraft:potatoes", 7, "minecraft:potato"),
            ("minecraft:beetroots", 7, "minecraft:beetroot_seeds"),
            ("minecraft:nether_wart", 3, "minecraft:nether_wart"),
        ] {
            let age = max_age.to_string();
            assert_eq!(ripe_seed(name, Some(&age)), Some(seed), "{name} at {age}");
            assert_eq!(crop(name), Some(Crop { max_age, seed }), "{name}");
        }
    }

    #[test]
    fn growing_crops_are_left_alone() {
        for name in [
            "minecraft:wheat",
            "minecraft:carrots",
            "minecraft:potatoes",
            "minecraft:beetroots",
        ] {
            for age in ["0", "1", "3", "6"] {
                assert_eq!(ripe_seed(name, Some(age)), None, "{name} at {age}");
            }
        }
        for age in ["0", "1", "2"] {
            assert_eq!(ripe_seed("minecraft:nether_wart", Some(age)), None);
        }
    }

    #[test]
    fn an_age_past_the_highest_is_not_ripe_either() {
        // 8 never occurs on wheat: it is not "at least ripe", it is a mismatch.
        assert_eq!(ripe_seed("minecraft:wheat", Some("8")), None);
        assert_eq!(ripe_seed("minecraft:nether_wart", Some("4")), None);
    }

    #[test]
    fn other_blocks_are_not_crops() {
        for name in [
            "minecraft:chest",
            "minecraft:air",
            "minecraft:farmland",
            "minecraft:sweet_berry_bush",
            "minecraft:torchflower_crop",
            "minecraft:carrot",
        ] {
            assert_eq!(crop(name), None, "{name}");
            assert_eq!(ripe_seed(name, Some("7")), None, "{name}");
        }
    }

    #[test]
    fn a_crop_without_a_readable_age_is_not_ripe() {
        assert_eq!(ripe_seed("minecraft:wheat", None), None);
        assert_eq!(ripe_seed("minecraft:wheat", Some("")), None);
        assert_eq!(ripe_seed("minecraft:wheat", Some("ripe")), None);
        assert_eq!(ripe_seed("minecraft:wheat", Some("-1")), None);
    }
}
