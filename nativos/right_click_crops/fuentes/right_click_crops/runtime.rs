// Shared by the standalone library and the injected game module.
use std::sync::{OnceLock, atomic::{AtomicBool, AtomicU32, Ordering}};

pub const MOD_ID: &str = "right_click_crops";
pub const MOD_NAME: &str = "Right-Click Crops";
pub const MOD_VERSION: &str = "1.0.1";
pub const MOD_DESCRIPTION: &str = "Survival harvesting with individual settings for sixteen plant families.";
pub const STARTUP_MESSAGE: &str = "[right_click_crops] Plant settings apply immediately; the global mod toggle requires a full game restart.";
pub const CONFIG_FILE: &str = "plants.conf";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Plant {
    Bamboo,
    Beetroot,
    Cactus,
    Carrots,
    Cocoa,
    Kelp,
    Melon,
    NetherWart,
    PitcherCrop,
    Potatoes,
    Pumpkin,
    SugarCane,
    Torchflower,
    TwistingVines,
    WeepingVines,
    Wheat,
}

impl Plant {
    pub const ALL: [Self; 16] = [
        Self::Bamboo,
        Self::Beetroot,
        Self::Cactus,
        Self::Carrots,
        Self::Cocoa,
        Self::Kelp,
        Self::Melon,
        Self::NetherWart,
        Self::PitcherCrop,
        Self::Potatoes,
        Self::Pumpkin,
        Self::SugarCane,
        Self::Torchflower,
        Self::TwistingVines,
        Self::WeepingVines,
        Self::Wheat,
    ];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Bamboo => "bamboo",
            Self::Beetroot => "beetroot",
            Self::Cactus => "cactus",
            Self::Carrots => "carrots",
            Self::Cocoa => "cocoa",
            Self::Kelp => "kelp",
            Self::Melon => "melon",
            Self::NetherWart => "nether_wart",
            Self::PitcherCrop => "pitcher_crop",
            Self::Potatoes => "potatoes",
            Self::Pumpkin => "pumpkin",
            Self::SugarCane => "sugar_cane",
            Self::Torchflower => "torchflower",
            Self::TwistingVines => "twisting_vines",
            Self::WeepingVines => "weeping_vines",
            Self::Wheat => "wheat",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Bamboo => "Bamboo",
            Self::Beetroot => "Beetroot",
            Self::Cactus => "Cactus",
            Self::Carrots => "Carrots",
            Self::Cocoa => "Cocoa",
            Self::Kelp => "Kelp",
            Self::Melon => "Melon",
            Self::NetherWart => "Nether wart",
            Self::PitcherCrop => "Pitcher crop",
            Self::Potatoes => "Potatoes",
            Self::Pumpkin => "Pumpkin",
            Self::SugarCane => "Sugar cane",
            Self::Torchflower => "Torchflower",
            Self::TwistingVines => "Twisting vines",
            Self::WeepingVines => "Weeping vines",
            Self::Wheat => "Wheat",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|plant| plant.key() == key)
    }

    pub fn from_block(block: &str) -> Option<Self> {
        match block {
            "minecraft:bamboo" => Some(Self::Bamboo),
            "minecraft:beetroots" => Some(Self::Beetroot),
            "minecraft:cactus" => Some(Self::Cactus),
            "minecraft:carrots" => Some(Self::Carrots),
            "minecraft:cocoa" => Some(Self::Cocoa),
            "minecraft:kelp" | "minecraft:kelp_plant" => Some(Self::Kelp),
            "minecraft:melon" => Some(Self::Melon),
            "minecraft:nether_wart" => Some(Self::NetherWart),
            "minecraft:pitcher_crop" => Some(Self::PitcherCrop),
            "minecraft:potatoes" => Some(Self::Potatoes),
            "minecraft:pumpkin" => Some(Self::Pumpkin),
            "minecraft:sugar_cane" => Some(Self::SugarCane),
            "minecraft:torchflower" => Some(Self::Torchflower),
            "minecraft:twisting_vines" | "minecraft:twisting_vines_plant" => Some(Self::TwistingVines),
            "minecraft:weeping_vines" | "minecraft:weeping_vines_plant" => Some(Self::WeepingVines),
            "minecraft:wheat" => Some(Self::Wheat),
            _ => None,
        }
    }

    const fn bit(self) -> u32 { 1 << self as u8 }
}

const ALL_PLANTS_MASK: u32 = (1 << Plant::ALL.len()) - 1;
static PLANT_SETTINGS_LOADED: OnceLock<()> = OnceLock::new();
static PLANT_MASK: AtomicU32 = AtomicU32::new(ALL_PLANTS_MASK);

pub fn parse_plant_config(contents: Option<&str>) -> u32 {
    let mut mask = ALL_PLANTS_MASK;
    let Some(contents) = contents else { return mask; };
    for line in contents.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') { continue; }
        let Some((key, value)) = line.split_once('=') else { continue; };
        let Some(plant) = Plant::from_key(key.trim()) else { continue; };
        match value.trim() {
            "true" => mask |= plant.bit(),
            "false" => mask &= !plant.bit(),
            _ => {}
        }
    }
    mask
}

pub fn serialize_plant_config(mask: u32) -> String {
    let mut out = String::from(
        "# Right-Click Crops plant settings.\n# true enables the mod's action; false leaves ordinary Horizon behavior.\n",
    );
    for plant in Plant::ALL {
        out.push_str(plant.key());
        out.push('=');
        out.push_str(if mask & plant.bit() != 0 { "true" } else { "false" });
        out.push('\n');
    }
    out
}

pub fn plant_settings_loaded() -> bool { PLANT_SETTINGS_LOADED.get().is_some() }

pub fn load_plant_settings_once(contents: Option<&str>) {
    PLANT_SETTINGS_LOADED.get_or_init(|| {
        PLANT_MASK.store(parse_plant_config(contents), Ordering::Release);
    });
}

pub fn plant_enabled(plant: Plant) -> bool {
    PLANT_MASK.load(Ordering::Acquire) & plant.bit() != 0
}

pub fn toggle_plant(plant: Plant) {
    PLANT_MASK.fetch_xor(plant.bit(), Ordering::AcqRel);
}

pub fn plant_config() -> String {
    serialize_plant_config(PLANT_MASK.load(Ordering::Acquire))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Crop {
    pub mature_age: Option<&'static str>,
    pub planting_item: &'static str,
    // Cocoa uses the game's direction-sensitive survival check instead.
    pub support_below: Option<&'static str>,
    pub initial_block: Option<&'static str>,
    pub two_blocks: bool,
}

// Explicit profiles, not every block with an age property.
pub fn crop_profile(block: &str) -> Option<Crop> {
    let (mature_age, planting_item, support_below, initial_block, two_blocks) = match block {
        "minecraft:wheat" => (Some("7"), "minecraft:wheat_seeds", Some("minecraft:farmland"), None, false),
        "minecraft:carrots" => (Some("7"), "minecraft:carrot", Some("minecraft:farmland"), None, false),
        "minecraft:potatoes" => (Some("7"), "minecraft:potato", Some("minecraft:farmland"), None, false),
        "minecraft:beetroots" => (Some("3"), "minecraft:beetroot_seeds", Some("minecraft:farmland"), None, false),
        "minecraft:nether_wart" => (Some("3"), "minecraft:nether_wart", Some("minecraft:soul_sand"), None, false),
        "minecraft:cocoa" => (Some("2"), "minecraft:cocoa_beans", None, None, false),
        "minecraft:torchflower" => (None, "minecraft:torchflower_seeds", Some("minecraft:farmland"), Some("minecraft:torchflower_crop"), false),
        "minecraft:pitcher_crop" => (Some("4"), "minecraft:pitcher_pod", Some("minecraft:farmland"), None, true),
        _ => return None,
    };
    Some(Crop { mature_age, planting_item, support_below, initial_block, two_blocks })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VerticalPlant {
    pub head: &'static str,
    pub body: &'static str,
    pub growth_y: i32,
}

pub fn vertical_profile(block: &str) -> Option<VerticalPlant> {
    let profile = match block {
        "minecraft:sugar_cane" => VerticalPlant { head: "minecraft:sugar_cane", body: "minecraft:sugar_cane", growth_y: 1 },
        "minecraft:cactus" => VerticalPlant { head: "minecraft:cactus", body: "minecraft:cactus", growth_y: 1 },
        "minecraft:bamboo" => VerticalPlant { head: "minecraft:bamboo", body: "minecraft:bamboo", growth_y: 1 },
        "minecraft:kelp" | "minecraft:kelp_plant" => VerticalPlant { head: "minecraft:kelp", body: "minecraft:kelp_plant", growth_y: 1 },
        "minecraft:twisting_vines" | "minecraft:twisting_vines_plant" => VerticalPlant { head: "minecraft:twisting_vines", body: "minecraft:twisting_vines_plant", growth_y: 1 },
        "minecraft:weeping_vines" | "minecraft:weeping_vines_plant" => VerticalPlant { head: "minecraft:weeping_vines", body: "minecraft:weeping_vines_plant", growth_y: -1 },
        _ => return None,
    };
    Some(profile)
}

pub fn same_vertical_plant(block: &str, profile: VerticalPlant) -> bool {
    block == profile.head || block == profile.body
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StemFruit {
    pub fruit: &'static str,
    pub attached_stem: &'static str,
    pub stem: &'static str,
}

pub fn stem_fruit_profile(block: &str) -> Option<StemFruit> {
    match block {
        "minecraft:pumpkin" => Some(StemFruit {
            fruit: "minecraft:pumpkin",
            attached_stem: "minecraft:attached_pumpkin_stem",
            stem: "minecraft:pumpkin_stem",
        }),
        "minecraft:melon" => Some(StemFruit {
            fruit: "minecraft:melon",
            attached_stem: "minecraft:attached_melon_stem",
            stem: "minecraft:melon_stem",
        }),
        _ => None,
    }
}

static ENABLED: OnceLock<bool> = OnceLock::new();
static STARTUP_REPORTED: AtomicBool = AtomicBool::new(false);

pub fn enabled(requested: bool) -> bool {
    *ENABLED.get_or_init(|| requested)
}

pub fn report_startup_once() {
    if !STARTUP_REPORTED.swap(true, Ordering::Relaxed) {
        eprintln!("{STARTUP_MESSAGE}");
    }
}

pub trait SeedStack {
    fn item(&self) -> &str;
    fn count(&self) -> u32;
    fn count_mut(&mut self) -> &mut u32;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Payment {
    Loot(usize),
    Inventory(usize),
    None,
}

pub fn is_seed(stack: &impl SeedStack, planting_item: &str) -> bool {
    stack.item() == planting_item && stack.count() > 0
}

pub fn select_payment<'a, S: SeedStack + 'a>(
    planting_item: &str,
    loot: &[S],
    inventory: impl IntoIterator<Item = (usize, &'a S)>,
) -> Payment {
    if let Some(index) = loot.iter().position(|s| is_seed(s, planting_item)) {
        Payment::Loot(index)
    } else if let Some((slot, _)) = inventory.into_iter().find(|(_, stack)| is_seed(*stack, planting_item)) {
        Payment::Inventory(slot)
    } else {
        Payment::None
    }
}

// Called only after the world transition succeeds. Selection guarantees a seed.
pub fn spend_one<S: SeedStack>(slot: &mut Option<S>, planting_item: &str) {
    if let Some(stack) = slot.as_mut().filter(|s| is_seed(*s, planting_item)) {
        *stack.count_mut() -= 1;
        if stack.count() == 0 {
            *slot = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SEED: &str = "minecraft:wheat_seeds";

    #[derive(Clone, Debug, PartialEq)]
    struct Stack(&'static str, u32);
    impl SeedStack for Stack {
        fn item(&self) -> &str { self.0 }
        fn count(&self) -> u32 { self.1 }
        fn count_mut(&mut self) -> &mut u32 { &mut self.1 }
    }

    fn seeds(count: u32) -> Stack { Stack(SEED, count) }

    #[test]
    fn loot_precedes_inventory_and_charges_exactly_one() {
        let loot = vec![Stack("minecraft:wheat", 1), seeds(3), seeds(2)];
        let inventory = [seeds(10)];
        assert_eq!(select_payment(SEED, &loot, inventory.iter().enumerate()), Payment::Loot(1));
        let mut paid = Some(loot[1].clone());
        spend_one(&mut paid, SEED);
        assert_eq!(paid, Some(seeds(2)));
        assert_eq!(inventory[0], seeds(10));
    }

    #[test]
    fn one_loot_seed_leaves_no_ground_seed() {
        let mut slot = Some(seeds(1));
        spend_one(&mut slot, SEED);
        assert_eq!(slot, None);
    }

    #[test]
    fn inventory_fallback_keeps_all_loot() {
        let loot = [Stack("minecraft:wheat", 1)];
        let mut inventory = [Some(seeds(2))];
        assert_eq!(select_payment(SEED, &loot, inventory.iter().enumerate().filter_map(|(i,s)| s.as_ref().map(|s| (i,s)))), Payment::Inventory(0));
        spend_one(&mut inventory[0], SEED);
        assert_eq!(inventory, [Some(seeds(1))]);
        assert_eq!(loot, [Stack("minecraft:wheat", 1)]);
    }

    #[test]
    fn empty_loot_can_use_the_last_offhand_seed() {
        let mut offhand = Some(seeds(1));
        assert_eq!(select_payment(SEED, &[], [(40, offhand.as_ref().unwrap())]), Payment::Inventory(40));
        spend_one(&mut offhand, SEED);
        assert_eq!(offhand, None);
    }

    #[test]
    fn wrong_items_and_zero_counts_do_not_pay() {
        let loot = [Stack("minecraft:beetroot_seeds", 8), seeds(0)];
        let inventory = [Stack("minecraft:wheat", 1), seeds(0)];
        assert_eq!(select_payment(SEED, &loot, inventory.iter().enumerate()), Payment::None);
        assert_eq!(select_payment::<Stack>(SEED, &[], []), Payment::None);
    }

    #[test]
    fn selection_and_cancelled_commit_have_no_item_effects() {
        let loot = [Stack("minecraft:wheat", 1)];
        let inventory = [seeds(1)];
        let before = (loot.clone(), inventory.clone());
        let _planned = select_payment(SEED, &loot, inventory.iter().enumerate());
        // An adapter that cannot commit must not call spend_one or spawn loot.
        assert_eq!((loot, inventory), before);
    }

    #[test]
    fn deterministic_first_positive_seed_stack() {
        let inventory = [(8, seeds(0)), (12, seeds(2)), (40, seeds(5))];
        assert_eq!(select_payment(SEED, &[], inventory.iter().map(|(i,s)| (*i,s))), Payment::Inventory(12));
    }

    #[test]
    fn spending_never_underflows_or_charges_wrong_items() {
        for stack in [seeds(0), Stack("minecraft:beetroot_seeds", 1)] {
            let mut slot = Some(stack.clone());
            spend_one(&mut slot, SEED);
            assert_eq!(slot, Some(stack));
        }
    }

    #[test]
    fn enablement_latch_keeps_both_sides_consistent_until_restart() {
        for first in [false, true] {
            let setting = OnceLock::new();
            assert_eq!(*setting.get_or_init(|| first), first);
            assert_eq!(*setting.get_or_init(|| !first), first);
        }
    }

    #[test]
    fn field_profiles_have_exact_maturity_and_planting_items() {
        for (block, age, item, support, initial, tall) in [
            ("minecraft:wheat", Some("7"), SEED, Some("minecraft:farmland"), None, false),
            ("minecraft:carrots", Some("7"), "minecraft:carrot", Some("minecraft:farmland"), None, false),
            ("minecraft:potatoes", Some("7"), "minecraft:potato", Some("minecraft:farmland"), None, false),
            ("minecraft:beetroots", Some("3"), "minecraft:beetroot_seeds", Some("minecraft:farmland"), None, false),
            ("minecraft:nether_wart", Some("3"), "minecraft:nether_wart", Some("minecraft:soul_sand"), None, false),
            ("minecraft:cocoa", Some("2"), "minecraft:cocoa_beans", None, None, false),
            ("minecraft:torchflower", None, "minecraft:torchflower_seeds", Some("minecraft:farmland"), Some("minecraft:torchflower_crop"), false),
            ("minecraft:pitcher_crop", Some("4"), "minecraft:pitcher_pod", Some("minecraft:farmland"), None, true),
        ] {
            assert_eq!(crop_profile(block), Some(Crop { mature_age: age, planting_item: item, support_below: support, initial_block: initial, two_blocks: tall }));
        }
        for block in ["minecraft:torchflower_crop", "minecraft:pitcher_plant", "minecraft:sweet_berry_bush", "minecraft:air"] {
            assert_eq!(crop_profile(block), None);
        }
    }

    #[test]
    fn every_profile_pays_only_its_own_item_and_prefers_loot() {
        let items = [SEED, "minecraft:carrot", "minecraft:potato", "minecraft:beetroot_seeds", "minecraft:nether_wart", "minecraft:cocoa_beans", "minecraft:torchflower_seeds", "minecraft:pitcher_pod"];
        for item in items {
            let loot = [Stack(item, 2)];
            let inventory = [Stack(item, 4)];
            assert_eq!(select_payment(item, &loot, inventory.iter().enumerate()), Payment::Loot(0));
            assert_eq!(select_payment(item, &[], inventory.iter().enumerate()), Payment::Inventory(0));
            let mut last = Some(Stack(item, 1));
            spend_one(&mut last, item);
            assert_eq!(last, None);
            for wrong in items.into_iter().chain(["minecraft:poisonous_potato", "minecraft:beetroot"]) {
                if wrong == item { continue; }
                let wrong_loot = [Stack(wrong, 3), Stack(item, 0)];
                assert_eq!(select_payment(item, &wrong_loot, [(40, &wrong_loot[0])]), Payment::None);
                let mut slot = Some(Stack(wrong, 3));
                spend_one(&mut slot, item);
                assert_eq!(slot, Some(Stack(wrong, 3)));
            }
        }
    }

    #[test]
    fn vertical_profiles_are_explicit_directed_and_need_no_planting_payment() {
        for name in ["minecraft:sugar_cane", "minecraft:cactus", "minecraft:bamboo"] {
            let profile = vertical_profile(name).unwrap();
            assert_eq!(profile, VerticalPlant { head: name, body: name, growth_y: 1 });
            assert_eq!(crop_profile(name), None);
        }
        for (head, body, growth_y) in [
            ("minecraft:kelp", "minecraft:kelp_plant", 1),
            ("minecraft:twisting_vines", "minecraft:twisting_vines_plant", 1),
            ("minecraft:weeping_vines", "minecraft:weeping_vines_plant", -1),
        ] {
            let expected = VerticalPlant { head, body, growth_y };
            assert_eq!(vertical_profile(head), Some(expected));
            assert_eq!(vertical_profile(body), Some(expected));
            assert!(same_vertical_plant(head, expected));
            assert!(same_vertical_plant(body, expected));
        }
        for name in ["minecraft:bamboo_sapling", "minecraft:cave_vines", "minecraft:vine"] {
            assert_eq!(vertical_profile(name), None);
        }
    }

    #[test]
    fn stem_fruits_are_explicit_and_decorations_are_excluded() {
        assert_eq!(stem_fruit_profile("minecraft:pumpkin"), Some(StemFruit {
            fruit: "minecraft:pumpkin",
            attached_stem: "minecraft:attached_pumpkin_stem",
            stem: "minecraft:pumpkin_stem",
        }));
        assert_eq!(stem_fruit_profile("minecraft:melon"), Some(StemFruit {
            fruit: "minecraft:melon",
            attached_stem: "minecraft:attached_melon_stem",
            stem: "minecraft:melon_stem",
        }));
        for name in ["minecraft:carved_pumpkin", "minecraft:jack_o_lantern", "minecraft:pumpkin_stem"] {
            assert_eq!(stem_fruit_profile(name), None);
        }
    }

    #[test]
    fn plant_settings_are_alphabetical_and_cover_only_mod_targets() {
        assert_eq!(Plant::ALL.map(Plant::label), [
            "Bamboo", "Beetroot", "Cactus", "Carrots", "Cocoa", "Kelp", "Melon",
            "Nether wart", "Pitcher crop", "Potatoes", "Pumpkin", "Sugar cane",
            "Torchflower", "Twisting vines", "Weeping vines", "Wheat",
        ]);
        assert_eq!(Plant::from_block("minecraft:kelp_plant"), Some(Plant::Kelp));
        assert_eq!(Plant::from_block("minecraft:weeping_vines_plant"), Some(Plant::WeepingVines));
        for ordinary in ["minecraft:sweet_berry_bush", "minecraft:cave_vines", "minecraft:cave_vines_plant"] {
            assert_eq!(Plant::from_block(ordinary), None);
        }
    }

    #[test]
    fn plant_config_defaults_to_enabled_and_accepts_exact_boolean_overrides() {
        assert_eq!(parse_plant_config(None), ALL_PLANTS_MASK);
        let mask = parse_plant_config(Some(
            "# player choices\nkelp=false\nwheat=false\nunknown=false\ncactus=invalid\n",
        ));
        assert_eq!(mask & Plant::Kelp.bit(), 0);
        assert_eq!(mask & Plant::Wheat.bit(), 0);
        assert_ne!(mask & Plant::Cactus.bit(), 0);
        assert_ne!(mask & Plant::Bamboo.bit(), 0);
    }

    #[test]
    fn serialized_plant_config_is_stable_and_round_trips() {
        let mask = ALL_PLANTS_MASK & !Plant::Bamboo.bit() & !Plant::PitcherCrop.bit();
        let text = serialize_plant_config(mask);
        assert!(text.contains("bamboo=false\n"));
        assert!(text.contains("pitcher_crop=false\n"));
        assert!(!text.contains("berry"));
        assert_eq!(parse_plant_config(Some(&text)), mask);
    }
}
