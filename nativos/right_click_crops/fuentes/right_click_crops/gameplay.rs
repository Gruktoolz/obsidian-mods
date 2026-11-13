use super::{Game, right_click_crops as core};
use horizon_client::{command::args::GameMode, first_person::Hand, items::ItemStack};
use horizon_core::position::BlockPos;
use horizon_world::{level::flags::{UPDATE_ALL, UPDATE_CLIENTS, UPDATE_KNOWN_SHAPE, UPDATE_SUPPRESS_DROPS}, registry::BlockStateId};
use std::{collections::HashMap, sync::{Mutex, OnceLock}};
use winit::keyboard::KeyCode;

impl core::SeedStack for ItemStack {
    fn item(&self) -> &str { &self.item }
    fn count(&self) -> u32 { self.count }
    fn count_mut(&mut self) -> &mut u32 { &mut self.count }
}

fn enabled(game: &Game<'_>) -> bool {
    if !core::plant_settings_loaded() {
        let stored = game.native_setting(core::MOD_ID, core::CONFIG_FILE);
        core::load_plant_settings_once(stored.as_deref());
    }
    core::enabled(game.native_on(core::MOD_ID))
}

struct CropTarget {
    pos: [i32; 3],
    state: BlockStateId,
    crop: core::Crop,
    upper: Option<([i32; 3], BlockStateId)>,
}

struct ColumnTarget {
    blocks: Vec<([i32; 3], BlockStateId)>,
}

struct FruitTarget {
    pos: [i32; 3],
    state: BlockStateId,
}

enum Target {
    Crop(CropTarget),
    Column(ColumnTarget),
    Fruit(FruitTarget),
}

#[derive(Clone, Copy)]
struct HandledClick {
    pos: [i32; 3],
    tick: u64,
}

static HANDLED_CLICKS: OnceLock<Mutex<HashMap<u128, HandledClick>>> = OnceLock::new();

fn actor_id(game: &Game<'_>) -> Option<u128> {
    game.lan_acting_uuid.or_else(|| game.serving.as_ref().map(|server| server.owner))
}

fn handled(game: &Game<'_>, pos: [i32; 3]) -> horizon_client::Result<bool> {
    if let Some(actor) = actor_id(game) {
        let clicks = HANDLED_CLICKS.get_or_init(|| Mutex::new(HashMap::new()));
        clicks.lock().unwrap_or_else(|error| error.into_inner())
            .insert(actor, HandledClick { pos, tick: game.ticks });
    }
    Ok(true)
}

pub(super) fn claim_offhand_followup(game: &Game<'_>, pos: Option<[i32; 3]>) -> bool {
    if game.hand != Hand::Off {
        return false;
    }
    let Some(actor) = actor_id(game) else { return false; };
    let clicks = HANDLED_CLICKS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut clicks = clicks.lock().unwrap_or_else(|error| error.into_inner());
    let Some(click) = clicks.get(&actor).copied() else {
        return false;
    };
    let matches = pos.is_none_or(|pos| click.pos == pos)
        && game.ticks.checked_sub(click.tick).is_some_and(|elapsed| elapsed <= 2);
    // UseItemOn and UseItem may both arrive for the same offhand click.
    if pos.is_none() || !matches { clicks.remove(&actor); }
    matches
}

fn target(game: &Game<'_>, hit: [i32; 3]) -> Option<Target> {
    let bone_meal = |s: &ItemStack| s.count > 0 && &*s.item == "minecraft:bone_meal";
    let main = game.stack_in(game.selected);
    if game.mode != GameMode::Survival
        || main.is_some_and(bone_meal)
        || game.offhand.as_ref().is_some_and(bone_meal)
        || (game.keys.contains(&KeyCode::ShiftLeft) && (main.is_some() || game.offhand.is_some()))
        || !game.world.loaded(hit[0], hit[2])
    {
        return None;
    }
    let clicked_state = game.world.block(hit[0], hit[1], hit[2]);
    let clicked_name = game.res.facts().get(clicked_state).name.clone();
    let plant = core::Plant::from_block(&clicked_name)?;
    if !core::plant_enabled(plant) { return None; }
    if let Some(profile) = core::vertical_profile(&clicked_name) {
        let mut anchor_y = hit[1];
        for _ in 0..32 {
            let support_y = anchor_y - profile.growth_y;
            let support = game.world.block(hit[0], support_y, hit[2]);
            if !core::same_vertical_plant(&game.res.facts().get(support).name, profile) { break; }
            anchor_y = support_y;
        }
        let mut blocks = Vec::new();
        for distance in 1..=32 {
            let y = anchor_y + profile.growth_y * distance;
            let state = game.world.block(hit[0], y, hit[2]);
            if !core::same_vertical_plant(&game.res.facts().get(state).name, profile) { break; }
            blocks.push(([hit[0], y, hit[2]], state));
        }
        if !blocks.is_empty() { return Some(Target::Column(ColumnTarget { blocks })); }
        return None;
    }
    if let Some(fruit) = core::stem_fruit_profile(&clicked_name) {
        let shears = |s: &ItemStack| s.count > 0 && &*s.item == "minecraft:shears";
        if fruit.fruit == "minecraft:pumpkin"
            && (main.is_some_and(shears) || game.offhand.as_ref().is_some_and(shears))
        {
            return None;
        }
        if !game.border.contains(f64::from(hit[0]), f64::from(hit[2])) {
            return None;
        }
        for (dx, dz, facing) in [(-1, 0, "east"), (1, 0, "west"), (0, -1, "south"), (0, 1, "north")] {
            let stem_pos = [hit[0] + dx, hit[1], hit[2] + dz];
            let stem_state = game.world.block(stem_pos[0], stem_pos[1], stem_pos[2]);
            if &*game.res.facts().get(stem_state).name == fruit.attached_stem
                && game.pack.assets.registry.state(stem_state)?
                    .properties.get("facing").map(String::as_str) == Some(facing)
            {
                return Some(Target::Fruit(FruitTarget { pos: hit, state: clicked_state }));
            }
        }
        return None;
    }
    let mut pos = hit;
    let mut state = game.world.block(pos[0], pos[1], pos[2]);
    let clicked = game.pack.assets.registry.state(state)?;
    if &*game.res.facts().get(state).name == "minecraft:pitcher_crop"
        && clicked.properties.get("half").map(String::as_str) == Some("upper")
    {
        pos[1] -= 1;
        state = game.world.block(pos[0], pos[1], pos[2]);
    }
    let crop = core::crop_profile(&game.res.facts().get(state).name)?;
    let properties = &game.pack.assets.registry.state(state)?.properties;
    if crop.mature_age.is_some_and(|age| properties.get("age").map(String::as_str) != Some(age)) {
        return None;
    }
    let upper = if crop.two_blocks {
        if properties.get("half").map(String::as_str) != Some("lower") { return None; }
        let upper_pos = [pos[0], pos[1] + 1, pos[2]];
        let upper_state = game.world.block(upper_pos[0], upper_pos[1], upper_pos[2]);
        let upper_props = &game.pack.assets.registry.state(upper_state)?.properties;
        if &*game.res.facts().get(upper_state).name != "minecraft:pitcher_crop"
            || upper_props.get("age").map(String::as_str) != crop.mature_age
            || upper_props.get("half").map(String::as_str) != Some("upper")
        { return None; }
        Some((upper_pos, upper_state))
    } else { None };
    Some(Target::Crop(CropTarget { pos, state, crop, upper }))
}

pub(super) fn harvest(game: &mut Game<'_>, hit: [i32; 3]) -> horizon_client::Result<bool> {
    if !enabled(game) || game.remote.is_some() || game.hand != Hand::Main {
        return Ok(false);
    }
    let Some(target) = target(game, hit) else { return Ok(false); };
    let crop_target = match target {
        Target::Column(column) => {
            let held = game.held().cloned();
            let mut loot = Vec::new();
            if game.ui.rules.bool("block_drops") {
                for (pos, state) in &column.blocks {
                    loot.extend(game.block_loot_at(*pos, *state, held.as_ref(), None));
                }
            }
            let written = game.with_level(|b, level, ticks| {
                let mut changed: Vec<([i32; 3], BlockStateId)> = Vec::new();
                for (pos, state) in column.blocks.iter().rev() {
                    let at = BlockPos::new(pos[0], pos[1], pos[2]);
                    if !b.set_block(level, ticks, at, BlockStateId(0), UPDATE_ALL | UPDATE_SUPPRESS_DROPS) {
                        for (restore_pos, restore_state) in changed.iter().rev() {
                            let restore_at = BlockPos::new(restore_pos[0], restore_pos[1], restore_pos[2]);
                            let _ = b.set_block(level, ticks, restore_at, *restore_state, UPDATE_ALL);
                        }
                        return false;
                    }
                    changed.push((*pos, *state));
                }
                true
            });
            if !written { return handled(game, hit); }
            for stack in loot {
                if stack.count > 0 && &*stack.item != "minecraft:air" {
                    game.pop_resource(hit, stack);
                }
            }
            game.process_level_events()?;
            return handled(game, hit);
        }
        Target::Fruit(fruit) => {
            let held = game.held().cloned();
            let loot = if game.ui.rules.bool("block_drops") {
                game.block_loot_at(fruit.pos, fruit.state, held.as_ref(), None)
            } else {
                Vec::new()
            };
            let at = BlockPos::new(fruit.pos[0], fruit.pos[1], fruit.pos[2]);
            let written = game.with_level(|b, level, ticks| {
                b.set_block(level, ticks, at, BlockStateId(0), UPDATE_ALL | UPDATE_SUPPRESS_DROPS)
            });
            if !written { return handled(game, hit); }
            for stack in loot {
                if stack.count > 0 && &*stack.item != "minecraft:air" {
                    game.pop_resource(fruit.pos, stack);
                }
            }
            game.process_level_events()?;
            return handled(game, hit);
        }
        Target::Crop(crop) => crop,
    };
    let CropTarget { pos, state, crop, upper } = crop_target;
    let at = BlockPos::new(pos[0], pos[1], pos[2]);
    let young = if let Some(name) = crop.initial_block {
        let Some(default) = game.pack.assets.registry.default_state(name) else { return handled(game, hit); };
        default
    } else {
        let Some(young) = game.with_props(state, &[("age", "0")]) else { return handled(game, hit); };
        young
    };
    // Reject unsupported ground before loot/RNG, inventory or world effects.
    if !game.border.contains(f64::from(pos[0]), f64::from(pos[2]))
        || crop.support_below.is_some_and(|support| &*game.res.facts().get(game.world.block(pos[0], pos[1] - 1, pos[2])).name != support)
        || !game.with_level(|b, level, ticks| b.can_survive(level, ticks, at, young))
    {
        return handled(game, hit);
    }
    let held = game.held().cloned();
    let mut loot = if game.ui.rules.bool("block_drops") {
        game.block_loot_at(pos, state, held.as_ref(), None)
    } else {
        Vec::new()
    };
    let slots = (0..36).chain(std::iter::once(super::shield::OFFHAND_SLOT));
    let payment = core::select_payment(crop.planting_item, &loot, slots.filter_map(|slot| game.stack_in(slot).map(|s| (slot, s))));
    let after = if payment == core::Payment::None { BlockStateId(0) } else { young };
    // These crop plants have no block entity or pre-remove effects.
    // On this thread no other player can alter the target between check and write.
    let written = game.with_level(|b, level, ticks| {
        if let Some((upper_pos, upper_state)) = upper {
            let upper_at = BlockPos::new(upper_pos[0], upper_pos[1], upper_pos[2]);
            let quiet_remove = UPDATE_CLIENTS | UPDATE_KNOWN_SHAPE | UPDATE_SUPPRESS_DROPS;
            if !b.set_block(level, ticks, upper_at, BlockStateId(0), quiet_remove) { return false; }
            if b.set_block(level, ticks, at, after, UPDATE_ALL) { return true; }
            let _ = b.set_block(level, ticks, upper_at, upper_state, UPDATE_ALL);
            return false;
        }
        b.set_block(level, ticks, at, after, UPDATE_ALL)
    });
    if !written { return handled(game, hit); }
    match payment {
        core::Payment::Loot(index) => {
            loot[index].count -= 1;
        }
        core::Payment::Inventory(slot) => core::spend_one(game.stack_in_mut(slot), crop.planting_item),
        core::Payment::None => {}
    }
    for stack in loot {
        if stack.count > 0 && &*stack.item != "minecraft:air" {
            game.pop_resource(pos, stack);
        }
    }
    game.process_level_events()?;
    handled(game, hit)
}
