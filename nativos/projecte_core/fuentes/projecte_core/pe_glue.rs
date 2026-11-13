// ProjectE's glue: the game half of the port. It reads blocks, block
// entities and stacks into the engine's types (`game::pe_engine`), acts on
// what the engine decides, and keeps each player's transmutation knowledge
// in the world's folder. Added to the game as `game::pe_glue`; the hook
// helpers (one file each beside horizon-mixin.toml) only call in here.
//
// Server work that needs the console (effects, teleports, area breaking)
// is queued and run at the end of the server tick, where no player's state
// is swapped in (native_run_command refuses inside a player's hook).

use horizon_client::containers::{ModBlockEntity, ModContainer};
use horizon_client::items::ItemStack;
use horizon_client::text::Text;
use horizon_world::nbt::{Compound, JavaString, Tag};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use super::pe_engine::area;
use super::pe_engine::devices::{self, Device};
use super::pe_engine::emc;
use super::pe_engine::ids;
use super::pe_engine::items::{self, Key};
use super::pe_engine::knowledge::{Knowledge, BAG_SLOTS};
use super::pe_engine::transmutation::{self, Click};
use super::Game;

const FACES: [[i32; 3]; 6] = [[0, -1, 0], [0, 1, 0], [0, 0, -1], [0, 0, 1], [-1, 0, 0], [1, 0, 0]];

// ---- State kept between hooks ----

/// What a player has open of ProjectE's server menus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Session {
    Table { page: usize },
    Bag { color: u8 },
}

/// What the server knows of a player between their hooks.
#[derive(Clone, Debug, Default)]
struct Held {
    /// The held slot, its item, charge and mode (read each tick from the stack).
    item: String,
    charge: i32,
    mode: i32,
    /// The face of the block they last started digging.
    face: [i32; 3],
    /// The server tick a repair talisman last ticked in their inventory, and
    /// an active gem of eternal density (its mode, that tick).
    repairing: u64,
    density: Option<(usize, u64)>,
    /// EMC the density gem has gathered, waiting to become its target.
    density_pool: u64,
    /// Toggles asked for by a right click, applied to the held stack on its next tick.
    toggle: bool,
}

struct Store {
    dir: Option<PathBuf>,
    players: Vec<(u128, Knowledge)>,
    dirty: Vec<u128>,
}

impl Store {
    fn get(&self, uuid: u128) -> Option<&Knowledge> {
        self.players.iter().find(|(u, _)| *u == uuid).map(|(_, k)| k)
    }
    fn get_mut(&mut self, uuid: u128) -> Option<&mut Knowledge> {
        self.players.iter_mut().find(|(u, _)| *u == uuid).map(|(_, k)| k)
    }
}

static STORE: Mutex<Store> = Mutex::new(Store { dir: None, players: Vec::new(), dirty: Vec::new() });
static SESSIONS: Mutex<Vec<(u128, Session, i32)>> = Mutex::new(Vec::new());
static TO_OPEN: Mutex<Vec<(u128, Session)>> = Mutex::new(Vec::new());
static COMMANDS: Mutex<Vec<String>> = Mutex::new(Vec::new());
static KEYS: Mutex<Vec<(u128, u8)>> = Mutex::new(Vec::new());
static HELD: Mutex<Option<HashMap<u128, Held>>> = Mutex::new(None);
/// Server ticks since the game started, for once-a-second work.
static TICKS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn now() -> u64 {
    TICKS.load(std::sync::atomic::Ordering::Relaxed)
}

fn queue(line: String) {
    if let Ok(mut q) = COMMANDS.lock() {
        q.push(line);
    }
}

fn with_held<R>(uuid: u128, f: impl FnOnce(&mut Held) -> R) -> R {
    let mut g = HELD.lock().unwrap_or_else(|e| e.into_inner());
    f(g.get_or_insert_with(HashMap::new).entry(uuid).or_default())
}

/// The player's knowledge, read from the world's folder the first time.
fn with_knowledge<R>(game: &Game<'_>, uuid: u128, f: impl FnOnce(&mut Knowledge) -> R) -> R {
    let dir = game.native_world_dir().map(|d| d.join("projecte"));
    let mut s = STORE.lock().unwrap_or_else(|e| e.into_inner());
    if s.dir != dir {
        // Another world: what was kept belongs to the last one (saved when it closed).
        s.players.clear();
        s.dirty.clear();
        s.dir = dir.clone();
    }
    if s.get(uuid).is_none() {
        let k = dir
            .as_ref()
            .and_then(|d| std::fs::read_to_string(d.join(format!("{uuid:032x}.txt"))).ok())
            .map(|t| Knowledge::load(&t))
            .unwrap_or_default();
        s.players.push((uuid, k));
    }
    let before = s.get(uuid).cloned();
    let r = f(s.get_mut(uuid).expect("loaded"));
    if s.get(uuid).cloned() != before && !s.dirty.contains(&uuid) {
        s.dirty.push(uuid);
    }
    r
}

/// Writes every changed player's file.
fn save_all() {
    let mut s = STORE.lock().unwrap_or_else(|e| e.into_inner());
    let Some(dir) = s.dir.clone() else { return };
    if s.dirty.is_empty() {
        return;
    }
    let _ = std::fs::create_dir_all(&dir);
    for uuid in std::mem::take(&mut s.dirty) {
        if let Some(k) = s.get(uuid) {
            let path = dir.join(format!("{uuid:032x}.txt"));
            let tmp = path.with_extension("tmp");
            if std::fs::write(&tmp, k.save()).is_ok() {
                let _ = std::fs::rename(&tmp, &path);
            }
        }
    }
}

// ---- Small readers ----

fn nbt_long(nbt: &Compound, key: &str) -> i64 {
    match nbt.get(&JavaString::from(key)) {
        Some(Tag::Long(v)) => *v,
        Some(Tag::Int(v)) => *v as i64,
        _ => 0,
    }
}

fn set_long(nbt: &mut Compound, key: &str, v: i64) {
    nbt.insert(JavaString::from(key), Tag::Long(v));
}

fn new_stack(id: &str, count: u32) -> ItemStack {
    ItemStack { item: Arc::from(id), count, damage: 0, extra: None }
}

fn comp_int(game: &Game<'_>, stack: &ItemStack, id: &str) -> i64 {
    match game.native_component(stack, id) {
        Ok(Some(Tag::Long(v))) => *v,
        Ok(Some(Tag::Int(v))) => *v as i64,
        Ok(Some(Tag::Byte(v))) => *v as i64,
        _ => 0,
    }
}

fn set_comp(game: &Game<'_>, stack: &mut ItemStack, id: &str, v: i64) {
    let tag = if id == ids::C_EMC { Tag::Long(v) } else { Tag::Int(v as i32) };
    if v == 0 {
        let _ = game.native_remove_component(stack, id);
    } else {
        let _ = game.native_set_component(stack, id, tag);
    }
}

/// One item's EMC, worn tools worth less.
fn value_of(game: &Game<'_>, stack: &ItemStack) -> Option<u64> {
    if stack.count == 0 {
        return None;
    }
    let v = emc::stack_value(&stack.item, stack.damage, emc::max_damage(&stack.item))?;
    // A Klein star is worth its own EMC and what it holds.
    Some(v + comp_int(game, stack, ids::C_EMC).max(0) as u64)
}

fn is_air(b: &str) -> bool {
    matches!(b, "minecraft:air" | "minecraft:cave_air" | "minecraft:void_air")
}

/// Blocks no ProjectE tool or stone may break or change.
fn unbreakable(b: &str) -> bool {
    is_air(b)
        || matches!(
            b,
            "minecraft:bedrock" | "minecraft:barrier" | "minecraft:end_portal_frame" | "minecraft:end_portal"
                | "minecraft:nether_portal" | "minecraft:command_block" | "minecraft:chain_command_block"
                | "minecraft:repeating_command_block" | "minecraft:structure_block" | "minecraft:jigsaw"
                | "minecraft:reinforced_deepslate" | "minecraft:light" | "minecraft:water" | "minecraft:lava"
                | "minecraft:end_gateway" | "minecraft:moving_piston"
        )
}

/// Whether the block at `pos` is in the block tag (the server's merged tags).
fn block_in_tag(game: &Game<'_>, pos: [i32; 3], tag: &str) -> bool {
    game.native_block_name_in_tag(&game.native_block(pos), tag).unwrap_or(false)
}

/// Sets a block at the end of the tick, through the console: the block's
/// default state, with no need of the game's block registry.
fn set_block(dim: &str, pos: [i32; 3], block: &str) {
    queue(format!("execute in {dim} run setblock {} {} {} {block}", pos[0], pos[1], pos[2]));
}

fn tell(game: &mut Game<'_>, uuid: u128, line: String) {
    game.native_action_bar(uuid, Text::literal(line));
}

/// The block face a player looks along: the axis their view is closest to.
fn facing(yaw: f32, pitch: f32) -> [i32; 3] {
    if pitch > 60.0 {
        return [0, -1, 0];
    }
    if pitch < -60.0 {
        return [0, 1, 0];
    }
    let y = yaw.rem_euclid(360.0);
    if !(45.0..315.0).contains(&y) {
        [0, 0, 1]
    } else if y < 135.0 {
        [-1, 0, 0]
    } else if y < 225.0 {
        [0, 0, -1]
    } else {
        [1, 0, 0]
    }
}

/// The first solid block along a player's view, up to `reach` blocks.
fn raycast(game: &Game<'_>, from: [f64; 3], yaw: f32, pitch: f32, reach: f64) -> Option<[i32; 3]> {
    let (y, p) = (yaw.to_radians() as f64, pitch.to_radians() as f64);
    let dir = [-y.sin() * p.cos(), -p.sin(), y.cos() * p.cos()];
    let mut t = 0.0;
    while t < reach {
        let at = [from[0] + dir[0] * t, from[1] + dir[1] * t, from[2] + dir[2] * t];
        let pos = [at[0].floor() as i32, at[1].floor() as i32, at[2].floor() as i32];
        let b = game.native_block(pos);
        if !is_air(&b) && b != "minecraft:water" && b != "minecraft:short_grass" {
            return Some(pos);
        }
        t += 0.2;
    }
    None
}

// ---- Registration ----

pub(super) fn component_types(types: &mut Vec<String>) {
    for c in ids::COMPONENTS {
        if !types.iter().any(|t| t == c) {
            types.push(c.to_string());
        }
    }
}

pub(super) fn block_entity(block: &str, entity: &mut Option<(String, Compound)>) {
    if entity.is_none() && (Device::of(block).is_some() || block == ids::ALCH_CHEST) {
        *entity = Some((block.to_string(), Compound::new()));
    }
}

pub(super) fn ticks(block: &str, ticks: &mut bool) {
    if Device::of(block).is_some() {
        *ticks = true;
    }
}

pub(super) fn container(block: &str, container: &mut Option<ModContainer>) {
    let all = |n: usize| -> Vec<usize> { (0..n).collect() };
    if block == ids::ALCH_CHEST {
        *container = Some(ModContainer {
            slots: 54,
            faces: [all(54), all(54), all(54), all(54), all(54), all(54)],
            inputs: all(54),
            outputs: all(54),
            menu: Some("minecraft:generic_9x6".to_string()),
        });
        return;
    }
    let Some(d) = Device::of(block) else { return };
    *container = Some(match d {
        Device::Condenser(t) => {
            let (inputs, outputs): (Vec<usize>, Vec<usize>) =
                if t == 0 { ((1..54).collect(), (1..54).collect()) } else { ((1..27).collect(), (27..54).collect()) };
            let faces = [outputs.clone(), inputs.clone(), inputs.clone(), inputs.clone(), inputs.clone(), inputs.clone()];
            ModContainer { slots: 54, faces, inputs, outputs, menu: Some("minecraft:generic_9x6".to_string()) }
        }
        Device::Pedestal => ModContainer {
            slots: 5,
            faces: [vec![0], vec![0], vec![0], vec![0], vec![0], vec![0]],
            inputs: vec![0],
            outputs: vec![0],
            menu: Some("minecraft:hopper".to_string()),
        },
        Device::Collector(_) | Device::Relay(_) | Device::Furnace(_) => ModContainer {
            // As vanilla's furnace: the top reaches the input, the sides the
            // second slot (fuel, a lock, a Klein star), the bottom the output.
            slots: 3,
            faces: [vec![2, 1], vec![0], vec![1], vec![1], vec![1], vec![1]],
            inputs: vec![0, 1],
            outputs: vec![2],
            menu: Some("minecraft:furnace".to_string()),
        },
        _ => return,
    });
}

/// What clients get of a device: its EMC.
pub(super) fn update_tag(entity: &ModBlockEntity, tag: &mut Option<Compound>) {
    if Device::of(&entity.id).is_none() {
        return;
    }
    let mut c = Compound::new();
    set_long(&mut c, "emc", nbt_long(&entity.nbt, "emc"));
    *tag = Some(c);
}

// ---- Device ticks ----

pub(super) fn tick(game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity) {
    let Some(d) = Device::of(&entity.id) else { return };
    let slots = d.container_slots();
    if entity.items.len() < slots {
        entity.items.resize(slots, None);
    }
    match d {
        Device::Collector(_) => collector_tick(game, pos, entity, d),
        Device::Relay(_) => relay_tick(game, pos, entity, d),
        Device::Condenser(t) => condenser_tick(game, entity, t == 1),
        Device::Furnace(red) => furnace_tick(game, pos, entity, red),
        Device::Pedestal => pedestal_tick(game, pos, entity),
        Device::Interdiction => interdiction_tick(game, pos, entity),
        Device::Nova(big) => nova_tick(game, pos, entity, big),
    }
}

fn set_lit(game: &mut Game<'_>, pos: [i32; 3], lit: bool) {
    let v = if lit { "true" } else { "false" };
    if game.native_block_property(pos, "lit").is_some_and(|x| x != v) {
        game.native_set_block_property(pos, "lit", v);
    }
}

/// Fills `stack` (a Klein star) from `available`: what went in.
fn charge_star(game: &Game<'_>, stack: &mut ItemStack, available: i64) -> i64 {
    let Some(cap) = items::klein_capacity(&stack.item) else { return 0 };
    let held = comp_int(game, stack, ids::C_EMC);
    let add = (cap as i64 - held).min(available).max(0);
    if add > 0 {
        set_comp(game, stack, ids::C_EMC, held + add);
    }
    add
}

/// Sends up to `offer` of this device's EMC to the devices beside it that
/// take it (devices::Device::feeds): what they took.
fn push_emc(game: &mut Game<'_>, pos: [i32; 3], from: Device, stored: i64, offer: i64) -> i64 {
    let fill = stored as f64 / from.capacity().max(1) as f64;
    let mut targets = Vec::new();
    let mut room = Vec::new();
    for f in FACES {
        let p = [pos[0] + f[0], pos[1] + f[1], pos[2] + f[2]];
        let Some(to) = Device::of(&game.native_block(p)) else { continue };
        let r = game.native_with_mod_block_entity(p, |be| {
            let e = nbt_long(&be.nbt, "emc");
            let to_fill = e as f64 / to.capacity().max(1) as f64;
            // A condenser only takes EMC while it has a target to make.
            let wants = !matches!(to, Device::Condenser(_)) || be.items.first().is_some_and(|s| s.is_some());
            (from.feeds(to, fill, to_fill) && wants).then(|| (to.capacity() - e).max(0))
        });
        if let Some(Some(r)) = r {
            targets.push(p);
            room.push(r);
        }
    }
    let taken = devices::share(offer, &room);
    let mut total = 0;
    for (p, t) in targets.into_iter().zip(taken) {
        if t > 0 {
            game.native_with_mod_block_entity(p, |be| {
                let e = nbt_long(&be.nbt, "emc");
                set_long(&mut be.nbt, "emc", e + t);
            });
            total += t;
        }
    }
    total
}

/// Puts `n` of `item` in the slots `range`, onto the same item first: what didn't fit.
fn insert(game: &Game<'_>, slots: &mut [Option<ItemStack>], range: std::ops::Range<usize>, item: &str, mut n: u32) -> u32 {
    let max = game.native_max_stack_size(item).unwrap_or(64);
    for i in range.clone() {
        if let Some(s) = slots[i].as_mut().filter(|s| &*s.item == item && s.extra.is_none() && s.count < max) {
            let add = (max - s.count).min(n);
            s.count += add;
            n -= add;
        }
    }
    for i in range {
        if n == 0 {
            break;
        }
        if slots[i].is_none() {
            let add = max.min(n);
            slots[i] = Some(new_stack(item, add));
            n -= add;
        }
    }
    n
}

fn room_for(game: &Game<'_>, slots: &[Option<ItemStack>], range: std::ops::Range<usize>, item: &str) -> u32 {
    let max = game.native_max_stack_size(item).unwrap_or(64);
    range
        .map(|i| match &slots[i] {
            None => max,
            Some(s) if &*s.item == item && s.extra.is_none() => max.saturating_sub(s.count),
            _ => 0,
        })
        .sum()
}

fn take_one(slot: &mut Option<ItemStack>) {
    if let Some(s) = slot.as_mut() {
        s.count = s.count.saturating_sub(1);
        if s.count == 0 {
            *slot = None;
        }
    }
}

/// Menu data for vanilla's furnace screen: the flame shows `a` of `a_max`,
/// the arrow `b` of `b_max` (16-bit on the network, so scaled to 1000).
fn furnace_data(a: i64, a_max: i64, b: i64, b_max: i64) -> Vec<i32> {
    let scale = |v: i64, m: i64| if m <= 0 { 0 } else { (v.clamp(0, m) as i128 * 1000 / m as i128) as i32 };
    vec![scale(a, a_max), 1000, scale(b, b_max), 1000]
}

fn collector_tick(game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity, d: Device) {
    let mut stored = nbt_long(&entity.nbt, "emc");
    let mut micro = nbt_long(&entity.nbt, "micro");
    let above = [pos[0], pos[1] + 1, pos[2]];
    let (block_light, sky_light) = game.native_light(above);
    let time = game.native_day_time().rem_euclid(24_000);
    let day = (time < 12_300 || time > 23_850) && game.native_weather().0 < 0.2;
    let sun = devices::sun_level(sky_light == 15, day, block_light.max(if day { sky_light } else { 0 }));
    micro += (devices::collector_gen(d, sun) * 1_000_000.0) as i64;
    let whole = micro / 1_000_000;
    micro -= whole * 1_000_000;
    stored = (stored + whole).min(d.capacity());

    // The input: a Klein star to charge, or a fuel to upgrade (to the lock's item when one is set).
    let mut progress = (0, 1);
    if let Some(mut s) = entity.items[0].clone() {
        if items::klein_capacity(&s.item).is_some() {
            stored -= charge_star(game, &mut s, stored.min(d.transfer()));
            let full = comp_int(game, &s, ids::C_EMC) as u64 >= items::klein_capacity(&s.item).unwrap_or(0);
            if full && entity.items[2].is_none() {
                entity.items[2] = Some(s);
                entity.items[0] = None;
            } else {
                entity.items[0] = Some(s);
            }
        } else if devices::is_fuel(&s.item) {
            let lock = entity.items[1].as_ref().map(|l| l.item.to_string());
            let target = lock
                .filter(|l| devices::upgrade_cost(&s.item, l).is_some())
                .or_else(|| devices::next_fuel(&s.item).map(str::to_string));
            if let Some(target) = target {
                let cost = devices::upgrade_cost(&s.item, &target).unwrap_or(i64::MAX);
                progress = (stored.min(cost), cost);
                if stored >= cost && room_for(game, &entity.items, 2..3, &target) > 0 {
                    stored -= cost;
                    take_one(&mut entity.items[0]);
                    insert(game, &mut entity.items, 2..3, &target, 1);
                }
            }
        }
    }
    if stored > 0 {
        stored -= push_emc(game, pos, d, stored, stored.min(d.transfer()));
    }
    set_lit(game, pos, sun > 0 && whole >= 0 && devices::collector_gen(d, sun) > 0.0);
    entity.data = furnace_data(stored, d.capacity(), progress.0, progress.1);
    set_long(&mut entity.nbt, "emc", stored);
    set_long(&mut entity.nbt, "micro", micro);
}

fn relay_tick(game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity, d: Device) {
    let mut stored = nbt_long(&entity.nbt, "emc");
    // Burn what's put in the input.
    if let Some(s) = entity.items[0].clone() {
        if items::klein_capacity(&s.item).is_some() {
            // A Klein star in the input is emptied into the relay.
            let mut s = s;
            let held = comp_int(game, &s, ids::C_EMC);
            let take = held.min(d.capacity() - stored).min(d.transfer()).max(0);
            if take > 0 {
                set_comp(game, &mut s, ids::C_EMC, held - take);
                stored += take;
            }
            entity.items[0] = Some(s);
        } else if let Some(v) = value_of(game, &s) {
            if stored + v as i64 <= d.capacity() {
                stored += v as i64;
                take_one(&mut entity.items[0]);
            }
        }
    }
    // Charge a Klein star in the second slot.
    if let Some(mut s) = entity.items[1].clone() {
        if items::klein_capacity(&s.item).is_some() {
            stored -= charge_star(game, &mut s, stored.min(d.transfer()));
            entity.items[1] = Some(s);
        }
    }
    if stored > 0 {
        let sent = push_emc(game, pos, d, stored, stored.min(d.transfer()));
        stored -= sent;
        if sent > 0 {
            // The relay's bonus while it passes EMC on.
            let mut bonus = nbt_long(&entity.nbt, "bonus") + (d.relay_bonus() * 1000.0) as i64;
            stored += bonus / 1000;
            bonus %= 1000;
            set_long(&mut entity.nbt, "bonus", bonus);
        }
    }
    stored = stored.min(d.capacity());
    set_lit(game, pos, stored > 0);
    entity.data = furnace_data(stored, d.capacity(), 0, 1);
    set_long(&mut entity.nbt, "emc", stored);
}

fn condenser_tick(game: &mut Game<'_>, entity: &mut ModBlockEntity, mk2: bool) {
    let stored = nbt_long(&entity.nbt, "emc");
    let Some(target) = entity.items[0].as_ref().map(|s| s.item.to_string()) else { return };
    let tv = emc::value(&target).unwrap_or(0);
    let (input, output) = if mk2 { (1..27, 27..54) } else { (1..54, 1..54) };
    let inputs: Vec<(usize, u32, u64)> = input
        .clone()
        .filter_map(|i| {
            let s = entity.items[i].as_ref()?;
            let v = if *s.item == *target { 0 } else { value_of(game, s).unwrap_or(0) };
            Some((i, s.count, v))
        })
        .collect();
    let room = room_for(game, &entity.items, output.clone(), &target);
    let step = devices::condense(mk2, tv, stored, &inputs, room.min(64));
    for (slot, n) in &step.consumed {
        if let Some(s) = entity.items[*slot].as_mut() {
            s.count = s.count.saturating_sub(*n);
            if s.count == 0 {
                entity.items[*slot] = None;
            }
        }
    }
    if step.produced > 0 {
        insert(game, &mut entity.items, output, &target, step.produced);
    }
    if step.stored != stored || !step.consumed.is_empty() {
        set_long(&mut entity.nbt, "emc", step.stored);
    }
}

/// The smelting recipes the matter furnaces take (projecte:matter_smelting, written from vanilla's).
fn smelt_result(game: &Game<'_>, input: &str) -> Option<(String, u32)> {
    game.native_recipes_of_type("projecte:matter_smelting").iter().find_map(|r| {
        let j = &*r.json;
        let ing = j["ingredient"].as_str()?;
        if ing != input {
            return None;
        }
        let res = &j["result"];
        Some((res["id"].as_str()?.to_string(), res["count"].as_u64().unwrap_or(1) as u32))
    })
}

fn furnace_tick(game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity, red: bool) {
    let mut burn = nbt_long(&entity.nbt, "burn");
    let mut burn_total = nbt_long(&entity.nbt, "burn_total");
    let mut cook = nbt_long(&entity.nbt, "cook");
    let total = devices::furnace_ticks(red) as i64;
    let result = entity.items[0].as_ref().and_then(|s| smelt_result(game, &s.item));
    let can = result.as_ref().is_some_and(|(id, n)| room_for(game, &entity.items, 2..3, id) >= *n);
    if burn <= 0 && can {
        if let Some(f) = entity.items[1].clone() {
            let t = devices::burn_ticks(&f.item) as i64;
            if t > 0 {
                burn = t;
                burn_total = t;
                if &*f.item == "minecraft:lava_bucket" {
                    entity.items[1] = Some(new_stack("minecraft:bucket", 1));
                } else {
                    take_one(&mut entity.items[1]);
                }
            }
        }
    }
    if burn > 0 {
        burn -= 1;
        if can {
            cook += 1;
            if cook >= total {
                cook = 0;
                let (id, n) = result.expect("can smelt");
                take_one(&mut entity.items[0]);
                // ProjectE's matter furnaces double an ore's ingot half the time (red matter two in three).
                let luck = nbt_long(&entity.nbt, "luck") + 1;
                set_long(&mut entity.nbt, "luck", luck);
                let ore = n == 1 && id.ends_with("_ingot") && if red { luck % 3 != 0 } else { luck % 2 == 0 };
                insert(game, &mut entity.items, 2..3, &id, if ore { 2 } else { n });
            }
        } else {
            cook = 0;
        }
    } else {
        cook = 0;
    }
    set_lit(game, pos, burn > 0);
    entity.data = vec![burn as i32, burn_total.max(1) as i32, cook as i32, total as i32];
    set_long(&mut entity.nbt, "burn", burn);
    set_long(&mut entity.nbt, "burn_total", burn_total);
    set_long(&mut entity.nbt, "cook", cook);
}

fn every(entity: &mut ModBlockEntity, n: i64) -> bool {
    let t = nbt_long(&entity.nbt, "t") + 1;
    set_long(&mut entity.nbt, "t", t);
    t % n == 0
}

/// A pedestal: with a redstone signal, the item on it works on the area round it.
fn pedestal_tick(game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity) {
    if !every(entity, 20) {
        return;
    }
    let dim = game.native_dimension().to_string();
    if game.native_neighbor_signal(&dim, pos) == 0 {
        return;
    }
    let Some(item) = entity.items[0].as_ref().map(|s| s.item.to_string()) else { return };
    let [x, y, z] = pos;
    let at = format!("execute in {dim} positioned {x} {y} {z}");
    match item.as_str() {
        "projecte:ignition_ring" => {
            for m in items::HOSTILES {
                queue(format!("{at} run damage @e[type={m},distance=..5] 3 minecraft:on_fire"));
            }
        }
        "projecte:archangel_smite" => {
            for m in items::HOSTILES {
                queue(format!("{at} run damage @e[type={m},distance=..8] 4 minecraft:arrow"));
            }
        }
        "projecte:swiftwolf_rending_gale" => {
            for m in items::HOSTILES {
                queue(format!("{at} as @e[type={m},distance=..8,limit=1,sort=random] at @s run summon minecraft:lightning_bolt"));
            }
        }
        "projecte:harvest_goddess_band" => grow_crops(game, pos, 5),
        "projecte:watch_of_flowing_time" => {
            grow_crops(game, pos, 5);
            queue("time add 20".to_string());
        }
        "projecte:zero_ring" => {
            queue(format!("{at} run fill ~-5 ~-2 ~-5 ~5 ~2 ~5 minecraft:ice replace minecraft:water"));
            queue(format!("{at} run fill ~-5 ~-2 ~-5 ~5 ~2 ~5 minecraft:air replace minecraft:fire"));
        }
        "projecte:soul_stone" => queue(format!("{at} run effect give @a[distance=..5] minecraft:regeneration 3 0 true")),
        "projecte:body_stone" => queue(format!("{at} run effect give @a[distance=..5] minecraft:saturation 1 0 true")),
        "projecte:life_stone" => {
            queue(format!("{at} run effect give @a[distance=..5] minecraft:regeneration 3 0 true"));
            queue(format!("{at} run effect give @a[distance=..5] minecraft:saturation 1 0 true"));
        }
        "projecte:evertide_amulet" => queue("weather rain 600".to_string()),
        "projecte:volcanite_amulet" => queue("weather clear 600".to_string()),
        "projecte:repair_talisman" => queue(format!("{at} run effect give @a[distance=..5] minecraft:resistance 2 0 true")),
        "projecte:black_hole_band" | "projecte:void_ring" => {
            let (lo, hi) = ([x as f64 - 5.0, y as f64 - 5.0, z as f64 - 5.0], [x as f64 + 6.0, y as f64 + 6.0, z as f64 + 6.0]);
            for e in game.native_item_entities(&dim, lo, hi) {
                let _ = game.native_set_item_entity(&dim, e.id, ItemStack { count: 0, ..e.stack.clone() });
                queue(format!("{at} run summon minecraft:item ~ ~1 ~ {{Item:{{id:\"{}\",count:{}}}}}", e.stack.item, e.stack.count));
            }
        }
        _ => {}
    }
}

/// Grows the crops within `r` of `pos` a stage, some each call.
fn grow_crops(game: &mut Game<'_>, pos: [i32; 3], r: i32) {
    let mut n = 0u32;
    for dx in -r..=r {
        for dy in -2..=2 {
            for dz in -r..=r {
                let p = [pos[0] + dx, pos[1] + dy, pos[2] + dz];
                let b = game.native_block(p);
                let Some(max) = items::crop_max_age(&b) else { continue };
                n = n.wrapping_mul(1103515245).wrapping_add(12345 + (dx * 31 + dz) as u32);
                if n % 3 != 0 {
                    continue;
                }
                let age: u8 = game.native_block_property(p, "age").and_then(|a| a.parse().ok()).unwrap_or(max);
                if age < max {
                    game.native_set_block_property_with(p, "age", &(age + 1).to_string(), super::native_hooks::flags::UPDATE_CLIENTS);
                }
            }
        }
    }
}

/// The interdiction torch pushes hostile mobs out of an 8-block sphere.
fn interdiction_tick(game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity) {
    if !every(entity, 10) {
        return;
    }
    let dim = game.native_dimension().to_string();
    let [x, y, z] = pos;
    for m in items::HOSTILES {
        queue(format!(
            "execute in {dim} positioned {x}.5 {y}.5 {z}.5 as @e[type={m},distance=..8] at @s facing {x}.5 {y}.5 {z}.5 run tp @s ^ ^ ^-0.8"
        ));
    }
}

/// Nova Catalyst and Cataclysm: a redstone signal lights the fuse; at its end, the blast.
fn nova_tick(game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity, big: bool) {
    let dim = game.native_dimension().to_string();
    let fuse = nbt_long(&entity.nbt, "fuse");
    if fuse == 0 {
        if game.native_neighbor_signal(&dim, pos) > 0 {
            set_long(&mut entity.nbt, "fuse", 80);
            set_lit(game, pos, true);
        }
        return;
    }
    if fuse > 1 {
        set_long(&mut entity.nbt, "fuse", fuse - 1);
        return;
    }
    set_long(&mut entity.nbt, "fuse", 0);
    set_block(&dim, pos, "minecraft:air");
    let c = [pos[0] as f64 + 0.5, pos[1] as f64 + 0.5, pos[2] as f64 + 0.5];
    let _ = game.native_explode(&dim, c, if big { 24.0 } else { 12.0 }, false, super::native_explosions::NativeBlast::Tnt);
}

// ---- Players: right clicks, item use, menus ----

pub(super) fn right_click_block(game: &mut Game<'_>, pos: [i32; 3], canceled: &mut bool) {
    let block = game.native_block(pos);
    let Some(p) = game.native_acting_player() else { return };
    if block == ids::TABLE {
        *canceled = true;
        open_later(p.uuid, Session::Table { page: 0 });
        return;
    }
    if let Some(d) = Device::of(&block) {
        if matches!(d, Device::Collector(_) | Device::Relay(_) | Device::Condenser(_)) {
            let e = game.native_with_mod_block_entity(pos, |be| nbt_long(&be.nbt, "emc")).unwrap_or(0);
            let line = match d {
                Device::Condenser(_) => format!("EMC: {}", emc::format(e.max(0) as u128)),
                _ => format!("EMC: {} / {}", emc::format(e.max(0) as u128), emc::format(d.capacity() as u128)),
            };
            tell(game, p.uuid, line);
        }
    }
}

fn open_later(uuid: u128, s: Session) {
    if let Ok(mut q) = TO_OPEN.lock() {
        q.retain(|(u, _)| *u != uuid);
        q.push((uuid, s));
    }
}

pub(super) fn left_click_block(game: &mut Game<'_>, face: [i32; 3]) {
    if let Some(p) = game.native_acting_player() {
        with_held(p.uuid, |h| h.face = face);
    }
}

pub(super) fn use_item_on(game: &mut Game<'_>, stack: &ItemStack, pos: [i32; 3], face: [i32; 3], claimed: &mut bool) {
    let item = stack.item.to_string();
    if !item.starts_with("projecte:") {
        return;
    }
    let Some(p) = game.native_acting_player() else { return };
    let charge = comp_int(game, stack, ids::C_CHARGE) as i32;
    let mode = comp_int(game, stack, ids::C_MODE) as i32;
    let dim = game.native_dimension().to_string();
    match item.as_str() {
        ids::STONE => {
            *claimed = true;
            let alt = game.native_sneaking();
            let Some(origin) = area::transmute(&game.native_block(pos), alt).map(|_| game.native_block(pos)) else { return };
            let mut changed = 0;
            for q in area::stone_area(pos, face, charge, mode) {
                if game.native_block(q) != origin {
                    continue;
                }
                let Some(to) = area::transmute(&origin, alt) else { continue };
                set_block(&dim, q, to);
                changed += 1;
            }
            if changed > 0 {
                queue(format!("execute in {dim} run playsound minecraft:block.amethyst_block.chime block @a {} {} {}", pos[0], pos[1], pos[2]));
            }
        }
        "projecte:divining_rod_1" | "projecte:divining_rod_2" | "projecte:divining_rod_3" => {
            *claimed = true;
            let depth = match item.as_str() { "projecte:divining_rod_1" => 3, "projecte:divining_rod_2" => 16, _ => 64 };
            let (mut total, mut n, mut best) = (0u128, 0u128, 0u64);
            for d in 0..depth {
                for dx in -1..=1 {
                    for dz in -1..=1 {
                        let q = [pos[0] + dx - face[0] * d, pos[1] - face[1] * d, pos[2] + dz - face[2] * d];
                        if let Some(v) = emc::value(&game.native_block(q)).or_else(|| ore_value(&game.native_block(q))) {
                            total += v as u128;
                            n += 1;
                            best = best.max(v);
                        }
                    }
                }
            }
            let avg = if n > 0 { total / n } else { 0 };
            tell(game, p.uuid, format!("Average EMC for {n} blocks: {} (highest {})", emc::format(avg), emc::format(best as u128)));
        }
        "projecte:destruction_catalyst" | "projecte:catalytic_lens" => {
            *claimed = true;
            let depth = 1 + charge * 2;
            let [fx, fy, fz] = face;
            for d in 0..depth {
                for a in -1..=1 {
                    for b in -1..=1 {
                        let q = if fy != 0 {
                            [pos[0] + a, pos[1] - fy * d, pos[2] + b]
                        } else if fx != 0 {
                            [pos[0] - fx * d, pos[1] + a, pos[2] + b]
                        } else {
                            [pos[0] + a, pos[1] + b, pos[2] - fz * d]
                        };
                        if !unbreakable(&game.native_block(q)) && !block_in_tag(game, q, "minecraft:features_cannot_replace") {
                            queue(format!("execute in {dim} run setblock {} {} {} minecraft:air destroy", q[0], q[1], q[2]));
                        }
                    }
                }
            }
        }
        "projecte:evertide_amulet" | "projecte:volcanite_amulet" => {
            *claimed = true;
            let q = [pos[0] + face[0], pos[1] + face[1], pos[2] + face[2]];
            let fluid = if item == "projecte:evertide_amulet" { "minecraft:water" } else { "minecraft:lava" };
            if is_air(&game.native_block(q)) {
                set_block(&dim, q, fluid);
            }
        }
        "projecte:dm_hoe" | "projecte:rm_hoe" => {
            *claimed = true;
            let r = charge;
            for dx in -r..=r {
                for dz in -r..=r {
                    let q = [pos[0] + dx, pos[1], pos[2] + dz];
                    let b = game.native_block(q);
                    let above = game.native_block([q[0], q[1] + 1, q[2]]);
                    if matches!(b.as_str(), "minecraft:dirt" | "minecraft:grass_block" | "minecraft:dirt_path" | "minecraft:coarse_dirt")
                        && is_air(&above)
                    {
                        set_block(&dim, q, "minecraft:farmland");
                    }
                }
            }
        }
        _ => {}
    }
}

/// Ores have no EMC; the divining rod reads them as what they smelt into.
fn ore_value(block: &str) -> Option<u64> {
    let name = block.strip_prefix("minecraft:")?.trim_start_matches("deepslate_").strip_suffix("_ore")?;
    let drop = match name {
        "iron" => "minecraft:iron_ingot",
        "gold" | "nether_gold" => "minecraft:gold_ingot",
        "copper" => "minecraft:copper_ingot",
        "coal" => "minecraft:coal",
        "diamond" => "minecraft:diamond",
        "emerald" => "minecraft:emerald",
        "lapis" => "minecraft:lapis_lazuli",
        "redstone" => "minecraft:redstone",
        "nether_quartz" => "minecraft:quartz",
        _ => return None,
    };
    emc::value(drop)
}

pub(super) fn use_item(game: &mut Game<'_>, stack: &ItemStack, claimed: &mut bool) {
    let item = stack.item.to_string();
    if !item.starts_with("projecte:") {
        return;
    }
    let Some(p) = game.native_acting_player() else { return };
    if item == ids::TABLET {
        *claimed = true;
        open_later(p.uuid, Session::Table { page: 0 });
    } else if let Some(c) = ids::bag_color(&item) {
        *claimed = true;
        open_later(p.uuid, Session::Bag { color: c });
    } else if item == ids::TOME {
        *claimed = true;
        let n = with_knowledge(game, p.uuid, |k| k.learn_all());
        tell(game, p.uuid, format!("Learned {n} new items"));
    } else if items::toggles(&item) {
        *claimed = true;
        with_held(p.uuid, |h| h.toggle = true);
    } else if matches!(item.as_str(), "projecte:hyperkinetic_lens" | "projecte:catalytic_lens" | "projecte:archangel_smite" | "projecte:void_ring") {
        *claimed = true;
        let charge = comp_int(game, stack, ids::C_CHARGE) as i32;
        fire(game, &item, charge);
    } else if item == "projecte:evertide_amulet" {
        *claimed = true;
        queue("weather rain 6000".to_string());
    } else if item == "projecte:volcanite_amulet" {
        *claimed = true;
        queue("weather clear 6000".to_string());
    } else if item == "projecte:mercurial_eye" {
        tell(game, p.uuid, "The Mercurial Eye isn't ported yet".to_string());
    }
}

/// The fire-projectile key (and right click) of the lenses, the smite and the void ring.
fn fire(game: &mut Game<'_>, item: &str, charge: i32) {
    let Some(p) = game.native_acting_player() else { return };
    let eye = [p.position[0], p.position[1] + 1.62, p.position[2]];
    let dim = p.dimension.clone();
    match item {
        "projecte:hyperkinetic_lens" | "projecte:catalytic_lens" => {
            if let Some(hit) = raycast(game, eye, p.yaw, p.pitch, 64.0) {
                let c = [hit[0] as f64 + 0.5, hit[1] as f64 + 0.5, hit[2] as f64 + 0.5];
                let _ = game.native_explode(&dim, c, 4.0 + charge as f32 * 2.0, false, super::native_explosions::NativeBlast::Tnt);
            }
        }
        "projecte:archangel_smite" => {
            let name = &p.name;
            for m in items::HOSTILES {
                queue(format!("execute as {name} at @s anchored eyes positioned ^ ^ ^8 run damage @e[type={m},distance=..8,limit=1,sort=nearest] 6 minecraft:arrow by {name}"));
            }
            queue(format!("execute at {name} run playsound minecraft:entity.arrow.shoot player @a ~ ~ ~"));
        }
        "projecte:void_ring" => {
            if let Some(hit) = raycast(game, eye, p.yaw, p.pitch, 64.0) {
                queue(format!("execute in {dim} run tp {} {}.5 {} {}.5", p.name, hit[0], hit[1] + 1, hit[2]));
            }
        }
        _ => {}
    }
}

/// A player's held stack this tick: keys, toggles, the area effects of what
/// they carry. Runs for every stack each tick; most return at once.
pub(super) fn inventory_tick(game: &mut Game<'_>, slot: i32, selected: bool) {
    let Some(stack) = game.native_carried_stack(slot).map(|s| s.clone()) else { return };
    let Some(p) = game.native_acting_player() else { return };
    let item = stack.item.to_string();
    let uuid = p.uuid;
    let t = now();
    let second = t % 20 == 0;

    // Repair talisman / density gem carried: they work on the other stacks.
    if !item.starts_with("projecte:") || items::is_tool(&item) {
        let (repairing, density) = with_held(uuid, |h| (t.saturating_sub(h.repairing) <= 1, h.density.filter(|d| t.saturating_sub(d.1) <= 1).map(|d| d.0)));
        if repairing && second && stack.damage > 0 {
            if let Some(s) = game.native_carried_stack(slot) {
                s.damage -= 1;
            }
        }
        if let Some(mode) = density {
            let target = items::DENSITY_TARGETS[mode.min(4)];
            let tv = emc::value(target).unwrap_or(0);
            if *stack.item != *target && emc::max_damage(&stack.item) == 0 && stack.extra.is_none() {
                if let Some(v) = emc::value(&stack.item).filter(|v| *v < tv && *v > 0) {
                    let gained = v * stack.count as u64;
                    if let Some(s) = game.native_carried_stack(slot) {
                        s.count = 0;
                    }
                    with_held(uuid, |h| h.density_pool += gained);
                }
            }
        }
        if !item.starts_with("projecte:") {
            if selected {
                with_held(uuid, |h| h.item.clear());
            }
            return;
        }
    }

    let mut stack = stack;
    let mut changed = false;
    if selected {
        // Keys from this player's client, then a right click's toggle.
        let keys: Vec<u8> = match KEYS.lock() {
            Ok(mut q) => {
                let mine: Vec<u8> = q.iter().filter(|(u, _)| *u == uuid).map(|(_, k)| *k).collect();
                q.retain(|(u, _)| *u != uuid);
                mine
            }
            Err(_) => Vec::new(),
        };
        for k in keys.into_iter().filter_map(Key::from_code) {
            match k {
                Key::Charge => {
                    let c = items::next_charge(&item, comp_int(game, &stack, ids::C_CHARGE) as i32);
                    if items::max_charge(&item) > 0 {
                        set_comp(game, &mut stack, ids::C_CHARGE, c as i64);
                        changed = true;
                        tell(game, uuid, format!("Charge: {c} / {}", items::max_charge(&item)));
                    }
                }
                Key::Mode => {
                    let names = items::modes(&item);
                    if !names.is_empty() {
                        let m = items::next_mode(&item, comp_int(game, &stack, ids::C_MODE) as i32);
                        set_comp(game, &mut stack, ids::C_MODE, m as i64);
                        changed = true;
                        tell(game, uuid, format!("Mode: {}", names[m as usize]));
                    }
                }
                Key::Extra => {
                    if items::toggles(&item) {
                        with_held(uuid, |h| h.toggle = true);
                    } else if item == ids::STONE || item == ids::TABLET {
                        open_later(uuid, Session::Table { page: 0 });
                    }
                }
                Key::Fire => {
                    let charge = comp_int(game, &stack, ids::C_CHARGE) as i32;
                    fire(game, &item, charge);
                }
            }
        }
        if with_held(uuid, |h| std::mem::take(&mut h.toggle)) && items::toggles(&item) {
            let on = comp_int(game, &stack, ids::C_ACTIVE) == 0;
            set_comp(game, &mut stack, ids::C_ACTIVE, on as i64);
            changed = true;
            tell(game, uuid, format!("{}: {}", item.trim_start_matches("projecte:").replace('_', " "), if on { "on" } else { "off" }));
        }
        let (charge, mode) = (comp_int(game, &stack, ids::C_CHARGE) as i32, comp_int(game, &stack, ids::C_MODE) as i32);
        with_held(uuid, |h| {
            h.item = item.clone();
            h.charge = charge;
            h.mode = mode;
        });
    }

    let active = comp_int(game, &stack, ids::C_ACTIVE) != 0;
    if item == "projecte:repair_talisman" {
        with_held(uuid, |h| h.repairing = t);
    }
    if item == "projecte:gem_of_eternal_density" {
        let mode = comp_int(game, &stack, ids::C_MODE) as usize;
        let target = items::DENSITY_TARGETS[mode.min(4)];
        with_held(uuid, |h| h.density = active.then_some((mode, t)));
        // Turn what it gathered into its target.
        let tv = emc::value(target).unwrap_or(u64::MAX);
        let pool = with_held(uuid, |h| std::mem::take(&mut h.density_pool)) + comp_int(game, &stack, ids::C_EMC).max(0) as u64;
        let n = (pool / tv) as u32;
        let rest = pool - n as u64 * tv;
        if n > 0 {
            let left = game.native_give_stack(uuid, new_stack(target, n)).map_or(0, |s| s.count);
            set_comp(game, &mut stack, ids::C_EMC, (rest + left as u64 * tv) as i64);
        } else if rest as i64 != comp_int(game, &stack, ids::C_EMC) {
            set_comp(game, &mut stack, ids::C_EMC, rest as i64);
        }
        changed = true;
    }
    if second {
        if (100..=103).contains(&slot) {
            armor_second(&item, &p.name);
        } else if active {
            let mode = comp_int(game, &stack, ids::C_MODE) as i32;
            active_second(game, &item, mode, &p);
        }
    }
    if changed {
        if let Some(s) = game.native_carried_stack(slot) {
            *s = stack;
        }
    }
}

fn effect(name: &str, e: &str, secs: u32, amp: u32) {
    queue(format!("effect give {name} minecraft:{e} {secs} {amp} true"));
}

/// Gem armor, worn: each piece's lasting effect.
fn armor_second(item: &str, name: &str) {
    match item {
        "projecte:gem_helmet" => {
            effect(name, "night_vision", 15, 0);
            effect(name, "water_breathing", 3, 0);
        }
        "projecte:gem_chestplate" => effect(name, "fire_resistance", 3, 0),
        "projecte:gem_leggings" => effect(name, "resistance", 3, 0),
        "projecte:gem_boots" => {
            effect(name, "speed", 3, 1);
            effect(name, "jump_boost", 3, 1);
            effect(name, "slow_falling", 3, 0);
        }
        _ => {}
    }
}

/// An active ring, band, stone or the watch, once a second.
fn active_second(game: &mut Game<'_>, item: &str, mode: i32, p: &horizon_client::mod_api::NativeServerPlayer) {
    let name = &p.name;
    let pos = [p.position[0].floor() as i32, p.position[1].floor() as i32, p.position[2].floor() as i32];
    let item = if item == "projecte:arcana_ring" {
        ["projecte:zero_ring", "projecte:ignition_ring", "projecte:harvest_goddess_band", "projecte:swiftwolf_rending_gale"]
            [mode.rem_euclid(4) as usize]
    } else {
        item
    };
    match item {
        "projecte:swiftwolf_rending_gale" => {
            // No hook gives a player creative flight: the gale lifts and carries instead.
            effect(name, "slow_falling", 3, 0);
            effect(name, "speed", 3, 2);
            effect(name, "jump_boost", 3, 3);
        }
        "projecte:zero_ring" => {
            queue(format!("execute at {name} run fill ~-3 ~-1 ~-3 ~3 ~-1 ~3 minecraft:frosted_ice replace minecraft:water"));
            queue(format!("execute at {name} run fill ~-3 ~-1 ~-3 ~3 ~2 ~3 minecraft:air replace minecraft:fire"));
        }
        "projecte:ignition_ring" => {
            effect(name, "fire_resistance", 3, 0);
            for m in items::HOSTILES {
                queue(format!("execute at {name} run damage @e[type={m},distance=..5] 2 minecraft:on_fire by {name}"));
            }
        }
        "projecte:harvest_goddess_band" => grow_crops(game, pos, 5),
        "projecte:black_hole_band" | "projecte:void_ring" => {
            let dim = p.dimension.clone();
            let r = 7.0;
            let (lo, hi) = (
                [p.position[0] - r, p.position[1] - r, p.position[2] - r],
                [p.position[0] + r, p.position[1] + r, p.position[2] + r],
            );
            for e in game.native_item_entities(&dim, lo, hi) {
                let left = game.native_give_stack(p.uuid, e.stack.clone());
                let rest = left.unwrap_or(ItemStack { count: 0, ..e.stack.clone() });
                let _ = game.native_set_item_entity(&dim, e.id, rest);
            }
        }
        "projecte:body_stone" => effect(name, "saturation", 1, 0),
        "projecte:soul_stone" => effect(name, "regeneration", 3, 1),
        "projecte:life_stone" => {
            effect(name, "saturation", 1, 0);
            effect(name, "regeneration", 3, 1);
        }
        "projecte:watch_of_flowing_time" => {
            grow_crops(game, pos, 5);
            match mode {
                1 => queue("time add 100".to_string()),
                2 => {
                    let t = game.native_day_time();
                    queue(format!("time set {}", (t - 100).max(0)));
                }
                _ => {}
            }
        }
        _ => {}
    }
}

/// Matter tools break more than the block dug: the pickaxe's modes, the
/// hammer's, shovel's, morning star's and katar's squares by charge.
pub(super) fn destroy_block(game: &mut Game<'_>, pos: [i32; 3]) {
    let Some(p) = game.native_acting_player() else { return };
    let h = with_held(p.uuid, |h| h.clone());
    let Some(name) = h.item.strip_prefix("projecte:") else { return };
    let kind = name.trim_start_matches("dm_").trim_start_matches("rm_");
    let tag = match kind {
        "pick" | "hammer" => "minecraft:mineable/pickaxe",
        "shovel" => "minecraft:mineable/shovel",
        "axe" => "minecraft:mineable/axe",
        "morning_star" => "c:mineable/morning_star",
        "katar" => "c:mineable/katar",
        _ => return,
    };
    let dir = facing(p.yaw, p.pitch);
    let face = if h.face == [0, 0, 0] { [-dir[0], -dir[1], -dir[2]] } else { h.face };
    let extra = if kind == "axe" {
        // The axe takes the whole trunk above.
        (1..=(4 + h.charge * 8)).map(|d| [pos[0], pos[1] + d, pos[2]]).collect()
    } else {
        area::dig_area(kind, pos, face, h.charge, h.mode, dir)
    };
    let dim = game.native_dimension().to_string();
    for q in extra {
        let b = game.native_block(q);
        if unbreakable(&b) {
            continue;
        }
        let ok = block_in_tag(game, q, tag)
            || (matches!(kind, "morning_star" | "katar") && block_in_tag(game, q, "minecraft:mineable/pickaxe"));
        if ok {
            queue(format!("execute in {dim} run setblock {} {} {} minecraft:air destroy", q[0], q[1], q[2]));
        }
    }
}

// ---- Menus ----

fn knowledge_title(k: &Knowledge, page: usize) -> String {
    format!(
        "Transmutation  EMC: {}  ({}/{})",
        emc::format(k.emc),
        page + 1,
        transmutation::pages(k.known.len())
    )
}

fn table_items(k: &Knowledge, page: usize) -> Vec<Option<ItemStack>> {
    let mut items: Vec<Option<ItemStack>> = vec![None; transmutation::SIZE];
    for (i, id) in transmutation::page_items(k, page).into_iter().enumerate() {
        items[i] = Some(new_stack(id, 1));
    }
    items[transmutation::PREV] = Some(new_stack("minecraft:arrow", 1));
    items[transmutation::NEXT] = Some(new_stack("minecraft:spectral_arrow", 1));
    items[transmutation::INFO] = Some(new_stack(ids::TABLE, 1));
    items
}

fn bag_items(k: &mut Knowledge, color: u8) -> Vec<Option<ItemStack>> {
    k.bag(color)
        .iter()
        .map(|s| s.as_ref().map(|(i, n, d)| ItemStack { item: Arc::from(i.as_str()), count: *n, damage: *d, extra: None }))
        .collect()
}

fn open_menus(game: &mut Game<'_>) {
    let todo: Vec<(u128, Session)> = match TO_OPEN.lock() {
        Ok(mut q) if !q.is_empty() => std::mem::take(&mut *q),
        _ => return,
    };
    for (uuid, s) in todo {
        let (title, items) = match s {
            Session::Table { page } => with_knowledge(game, uuid, |k| (knowledge_title(k, page), table_items(k, page))),
            Session::Bag { color } => with_knowledge(game, uuid, |k| {
                (format!("Alchemical Bag ({})", ids::BAG_COLORS[color as usize].replace('_', " ")), bag_items(k, color))
            }),
        };
        if let Some(window) = game.native_open_menu(uuid, transmutation::ROWS, Text::literal(title), items) {
            if let Ok(mut open) = SESSIONS.lock() {
                open.retain(|(u, _, _)| *u != uuid);
                open.push((uuid, s, window));
            }
        }
    }
}

fn session(uuid: u128) -> Option<Session> {
    SESSIONS.lock().ok()?.iter().find(|(u, _, _)| *u == uuid).map(|(_, s, _)| *s)
}

fn set_session(uuid: u128, s: Session) {
    if let Ok(mut open) = SESSIONS.lock() {
        for e in open.iter_mut().filter(|(u, _, _)| *u == uuid) {
            e.1 = s;
        }
    }
}

fn player_name(game: &Game<'_>, uuid: u128) -> Option<String> {
    game.native_server_players().into_iter().find(|p| p.uuid == uuid).map(|p| p.name)
}

/// Empties inventory slot `inv` (0..9 the hotbar) of a player.
fn clear_slot(game: &mut Game<'_>, name: &str, inv: usize) -> bool {
    game.native_run_command(&format!("item replace entity {name} container.{inv} with minecraft:air"))
}

pub(super) fn menu_clicked(game: &mut Game<'_>, uuid: u128, slot: i32, button: i32, mode: i32) {
    let Some(s) = session(uuid) else { return };
    let Some(name) = player_name(game, uuid) else { return };
    let inv_slot = super::native_menus::native_menu_inventory_slot(transmutation::SIZE, slot);
    let stacks = game.native_player_stacks(uuid).unwrap_or_default();
    let clicked = inv_slot.and_then(|i| stacks.get(i).cloned().flatten());
    match s {
        Session::Table { page } => {
            let is_star = clicked.as_ref().is_some_and(|c| items::klein_capacity(&c.item).is_some());
            let max = match (inv_slot, usize::try_from(slot)) {
                (None, Ok(i)) if i < transmutation::GRID => with_knowledge(game, uuid, |k| {
                    transmutation::page_items(k, page).get(i).map(|id| id.to_string())
                })
                .and_then(|id| game.native_max_stack_size(&id))
                .unwrap_or(64),
                _ => 64,
            };
            let click = with_knowledge(game, uuid, |k| transmutation::click(k, page, slot, button, mode, inv_slot, is_star, max));
            let mut page = page;
            match click {
                Click::Withdraw { item, count } => {
                    let room = game.native_max_stack_size(&item).unwrap_or(64);
                    let got = with_knowledge(game, uuid, |k| k.withdraw(&item, count.min(room)));
                    if got > 0 {
                        if let Some(back) = game.native_give_stack(uuid, new_stack(&item, got)) {
                            // What didn't fit is paid back.
                            let v = emc::value(&item).unwrap_or(0) as u128 * back.count as u128;
                            with_knowledge(game, uuid, |k| k.emc += v);
                        }
                    }
                }
                Click::Burn { slot: inv, all } => {
                    if let Some(st) = clicked.filter(|c| value_of(game, c).is_some()) {
                        let each = value_of(game, &st).unwrap_or(0);
                        let n = if all { st.count } else { 1 };
                        if clear_slot(game, &name, inv) {
                            if n < st.count {
                                let _ = game.native_give_stack(uuid, ItemStack { count: st.count - n, ..st.clone() });
                            }
                            with_knowledge(game, uuid, |k| k.burn(&st.item, n, each));
                        }
                    }
                }
                Click::Star { slot: inv, fill } => {
                    if let Some(mut st) = clicked {
                        let cap = items::klein_capacity(&st.item).unwrap_or(0) as i64;
                        let held = comp_int(game, &st, ids::C_EMC);
                        let moved = if fill {
                            with_knowledge(game, uuid, |k| {
                                let m = (cap - held).max(0).min(k.emc.min(i64::MAX as u128) as i64);
                                k.emc -= m as u128;
                                m
                            })
                        } else {
                            with_knowledge(game, uuid, |k| {
                                k.emc += held.max(0) as u128;
                                -held
                            })
                        };
                        if moved != 0 && clear_slot(game, &name, inv) {
                            set_comp(game, &mut st, ids::C_EMC, held + moved);
                            if let Some(back) = game.native_give_stack(uuid, st) {
                                // No room: put it back as it was and undo.
                                let _ = back;
                                with_knowledge(game, uuid, |k| {
                                    if moved > 0 { k.emc += moved as u128 } else { k.emc -= (-moved) as u128 }
                                });
                            }
                        }
                    }
                }
                Click::Forget(item) => {
                    with_knowledge(game, uuid, |k| k.forget(&item));
                }
                Click::Page(d) => {
                    let pages = with_knowledge(game, uuid, |k| transmutation::pages(k.known.len()));
                    page = (page as isize + d).rem_euclid(pages as isize) as usize;
                    set_session(uuid, Session::Table { page });
                }
                Click::None => {}
            }
            // The title showed the EMC when the menu opened; the action bar follows each click.
            let (emc_now, items) = with_knowledge(game, uuid, |k| (k.emc, table_items(k, page)));
            game.native_set_menu_items(uuid, items);
            tell(game, uuid, format!("EMC: {}", emc::format(emc_now)));
        }
        Session::Bag { color } => {
            if let Some(inv) = inv_slot {
                // From the inventory into the bag: plain stacks only.
                if let Some(st) = clicked.filter(|c| c.extra.is_none()) {
                    let fits = with_knowledge(game, uuid, |k| {
                        let bag = k.bag(color);
                        let free = bag.iter().position(|s| s.is_none());
                        if let Some(i) = free {
                            bag[i] = Some((st.item.to_string(), st.count, st.damage));
                        }
                        free.is_some()
                    });
                    if fits && !clear_slot(game, &name, inv) {
                        // Couldn't take it out of the inventory: take it back out of the bag.
                        with_knowledge(game, uuid, |k| {
                            if let Some(e) = k.bag(color).iter_mut().rev().find(|s| s.as_ref().is_some_and(|x| x.0 == *st.item)) {
                                *e = None;
                            }
                        });
                    }
                }
            } else if let Ok(i) = usize::try_from(slot) {
                if i < BAG_SLOTS {
                    let taken = with_knowledge(game, uuid, |k| k.bag(color)[i].take());
                    if let Some((item, n, d)) = taken {
                        let back = game.native_give_stack(uuid, ItemStack { item: Arc::from(item.as_str()), count: n, damage: d, extra: None });
                        if let Some(b) = back {
                            with_knowledge(game, uuid, |k| k.bag(color)[i] = Some((item, b.count, d)));
                        }
                    }
                }
            }
            let items = with_knowledge(game, uuid, |k| bag_items(k, color));
            game.native_set_menu_items(uuid, items);
        }
    }
    save_all();
}

/// A menu closed: forget the session, unless it is an older window than the one open now.
pub(super) fn menu_closed(uuid: u128, window: Option<i32>) {
    if let Ok(mut open) = SESSIONS.lock() {
        open.retain(|(u, _, w)| *u != uuid || window.is_some_and(|x| x != *w));
    }
}

// ---- Client: keys, tooltips, bars ----

pub(super) fn key(game: &mut Game<'_>, key: &str, pressed: bool) {
    if !pressed || !game.native_playing() {
        return;
    }
    for (k, name, default) in Key::ALL {
        if key == game.native_mod_key("projecte", name, default) {
            let _ = game.native_net_send_to_server("projecte:key", &[k.code()]);
        }
    }
}

pub(super) fn net_from_client(player: u128, channel: &str, data: &[u8]) {
    if channel != "projecte:key" || data.len() != 1 || Key::from_code(data[0]).is_none() {
        return;
    }
    if let Ok(mut q) = KEYS.lock() {
        if q.len() < 256 {
            q.push((player, data[0]));
        }
    }
}

pub(super) fn tooltip(game: &Game<'_>, stack: &ItemStack, lines: &mut Vec<(Text, u32)>) {
    let yellow = 0xFFFF_FF55;
    if let Some(v) = emc::stack_value(&stack.item, stack.damage, emc::max_damage(&stack.item)) {
        lines.push((Text::literal(format!("EMC: {}", emc::format(v as u128))), yellow));
        if stack.count > 1 {
            lines.push((Text::literal(format!("Stack EMC: {}", emc::format(v as u128 * stack.count as u128))), yellow));
        }
    }
    if let Some(cap) = items::klein_capacity(&stack.item) {
        let held = comp_int(game, stack, ids::C_EMC).max(0);
        lines.push((Text::literal(format!("Stored EMC: {} / {}", emc::format(held as u128), emc::format(cap as u128))), 0xFF55_FFFF));
    }
    if !stack.item.starts_with("projecte:") {
        return;
    }
    if items::max_charge(&stack.item) > 0 {
        let c = comp_int(game, stack, ids::C_CHARGE);
        lines.push((Text::literal(format!("Charge: {c} / {} (V)", items::max_charge(&stack.item))), 0xFFAA_AAAA));
    }
    let modes = items::modes(&stack.item);
    if !modes.is_empty() {
        let m = comp_int(game, stack, ids::C_MODE).clamp(0, modes.len() as i64 - 1) as usize;
        lines.push((Text::literal(format!("Mode: {} (G)", modes[m])), 0xFFAA_AAAA));
    }
    if items::toggles(&stack.item) {
        let on = comp_int(game, stack, ids::C_ACTIVE) != 0;
        lines.push((Text::literal(if on { "Active (right click or C)" } else { "Inactive (right click or C)" }.to_string()), 0xFFAA_AAAA));
    }
}

pub(super) fn item_bar(game: &Game<'_>, stack: &ItemStack, bar: &mut Option<(i32, u32)>) {
    if let Some(cap) = items::klein_capacity(&stack.item) {
        let held = comp_int(game, stack, ids::C_EMC).clamp(0, cap as i64);
        if held > 0 {
            *bar = Some(((held * 13 / cap as i64) as i32, 0x55_FFFF));
        }
    }
}

// ---- Server tick and lifecycle ----

pub(super) fn server_tick_end(game: &mut Game<'_>) {
    open_menus(game);
    let lines = match COMMANDS.lock() {
        Ok(mut q) if !q.is_empty() => std::mem::take(&mut *q),
        _ => Vec::new(),
    };
    for line in lines {
        game.native_run_command(&line);
    }
    let t = TICKS.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
    if t % 20 == 0 {
        save_all();
    }
}

pub(super) fn player_left(player: u128) {
    save_all();
    menu_closed(player, None);
    if let Ok(mut s) = STORE.lock() {
        s.players.retain(|(u, _)| *u != player);
    }
    if let Ok(mut g) = HELD.lock() {
        if let Some(m) = g.as_mut() {
            m.remove(&player);
        }
    }
}

pub(super) fn world_closing() {
    save_all();
    if let Ok(mut s) = STORE.lock() {
        s.players.clear();
        s.dir = None;
    }
    if let Ok(mut q) = SESSIONS.lock() {
        q.clear();
    }
}
