// Voltz Core's glue: reads a machine's block entity into the engine's types,
// ticks it, writes it back, and moves energy over the wire. Added to the game
// as `game::voltz_glue`; everything it decides is in `voltz_core_engine`.

use horizon_client::containers::{ModBlockEntity, ModContainer};
use horizon_client::items::ItemStack;
use horizon_world::nbt::{Compound, JavaString, Tag};
use std::sync::Arc;
use super::voltz_engine::energy::{self, Node};
use super::voltz_engine::machines::{self, Kind, State, Surroundings, FUEL, INPUT, OUTPUT};
use super::voltz_engine::recipes::{self, Recipe, Stack};
use super::voltz_engine::explosives::{self, After, Blast};
use std::sync::Mutex;
use super::voltz_engine::ids;
use super::voltz_engine::worldgen;
use super::voltz_engine::forcefield::{self, Shape};
use super::voltz_engine::buildcraft::{self, Node as PipeNode};
use super::voltz_engine::ae2::{self, Store};
use super::voltz_engine::ic2reactor::{self, Cell};
use super::voltz_engine::weapons;
use std::collections::HashMap;
use horizon_core::geometry::Direction;

use super::Game;
use super::native_explosions::NativeBlast;
use horizon_world::registry::BlockStateId;

pub(super) fn block_entity(block: &str, entity: &mut Option<(String, Compound)>) {
    if entity.is_none() && ids::has_block_entity(block) {
        *entity = Some((block.to_string(), Compound::new()));
    }
}

pub(super) fn ticks(block: &str, ticks: &mut bool) {
    if ids::ticks(block) {
        *ticks = true;
    }
}

pub(super) fn container(block: &str, container: &mut Option<ModContainer>) {
    if let Some(n) = ids::chest_slots(block) {
        let all: Vec<usize> = (0..n).collect();
        *container = Some(ModContainer {
            slots: n,
            faces: [all.clone(), all.clone(), all.clone(), all.clone(), all.clone(), all.clone()],
            inputs: all.clone(),
            outputs: all,
            menu: Some(format!("minecraft:generic_9x{}", n / 9)),
        });
        return;
    }
    if block == ae2::DRIVE {
        let all: Vec<usize> = (0..ae2::DRIVE_SLOTS).collect();
        *container = Some(ModContainer {
            slots: ae2::DRIVE_SLOTS,
            faces: [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            inputs: Vec::new(),
            outputs: Vec::new(),
            menu: Some("minecraft:generic_3x3".to_string()),
        });
        let _ = all;
        return;
    }
    if block == weapons::LAUNCHER {
        *container = Some(ModContainer {
            slots: 5,
            faces: [vec![0], vec![0], vec![0], vec![0], vec![0], vec![0]],
            inputs: vec![0],
            outputs: Vec::new(),
            menu: Some("minecraft:hopper".to_string()),
        });
        return;
    }
    if block == ic2reactor::REACTOR {
        let all: Vec<usize> = (0..ic2reactor::SLOTS).collect();
        *container = Some(ModContainer {
            slots: ic2reactor::SLOTS,
            faces: [all.clone(), all.clone(), all.clone(), all.clone(), all.clone(), all.clone()],
            inputs: all.clone(),
            outputs: all,
            menu: Some("minecraft:generic_9x6".to_string()),
        });
        return;
    }
    if buildcraft::is_quarry(block) {
        let all: Vec<usize> = (0..buildcraft::QUARRY_SLOTS).collect();
        *container = Some(ModContainer {
            slots: buildcraft::QUARRY_SLOTS,
            faces: [all.clone(), all.clone(), all.clone(), all.clone(), all.clone(), all.clone()],
            inputs: Vec::new(),
            outputs: all,
            menu: Some("minecraft:generic_9x3".to_string()),
        });
        return;
    }
    if !Kind::of(block).is_some_and(Kind::has_slots) {
        return;
    }
    // As vanilla's furnace: the top reaches the input, the sides the fuel,
    // the bottom the output (and the fuel slot, for an empty bucket).
    let side = vec![FUEL];
    *container = Some(ModContainer {
        slots: 3,
        faces: [vec![OUTPUT, FUEL], vec![INPUT], side.clone(), side.clone(), side.clone(), side],
        inputs: vec![INPUT, FUEL],
        outputs: vec![OUTPUT],
        menu: Some("minecraft:furnace".to_string()),
    });
}

fn load(nbt: &Compound) -> State {
    State::load(|k| match nbt.get(&JavaString::from(k)) {
        Some(Tag::Long(v)) => Some(*v),
        Some(Tag::Int(v)) => Some(*v as i64),
        _ => None,
    })
}

fn save(nbt: &mut Compound, s: &State) {
    for (k, v) in s.fields() {
        nbt.insert(JavaString::from(k), Tag::Long(v));
    }
}

fn stack(slot: &Option<ItemStack>) -> Stack {
    slot.as_ref().filter(|s| s.count > 0).map(|s| (s.item.to_string(), s.count))
}

fn new_stack(id: &str, count: u32) -> ItemStack {
    ItemStack { item: Arc::from(id), count, damage: 0, extra: None }
}

/// The recipes of `kind`'s type from the content mod and data packs.
fn recipes_for(game: &Game<'_>, kind: Kind) -> Vec<Recipe> {
    let Some(t) = kind.recipe_type() else { return Vec::new() };
    game.native_recipes_of_type(t)
        .iter()
        .filter_map(|r| {
            let j = &*r.json;
            let ingredient = match &j["ingredient"] {
                serde_json::Value::String(s) => s.clone(),
                v => v["item"].as_str().unwrap_or_default().to_string(),
            };
            let result = &j["result"];
            let id = result.as_str().or_else(|| result["id"].as_str()).unwrap_or_default();
            let count = result["count"].as_u64().map(|c| c as u32);
            let secondary = match &j["secondary"] {
                serde_json::Value::String(s) => Some(s.as_str()),
                v => v["item"].as_str(),
            };
            recipes::from_fields2(&ingredient, secondary, id, count)
        })
        .collect()
}

pub(super) fn tick(game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity) {
    if let Some(blast) = explosives::blast(&entity.id) {
        explosive_tick(game, pos, entity, blast);
        return;
    }
    if entity.id == buildcraft::WOODEN_PIPE {
        pipe_tick(game, pos, entity);
        return;
    }
    if entity.id == ae2::IMPORT_BUS {
        import_tick(game, pos, entity);
        return;
    }
    if entity.id == weapons::LAUNCHER {
        launcher_tick(game, pos, entity);
        return;
    }
    let Some(kind) = Kind::of(&entity.id) else { return };
    let slots_wanted = match kind {
        Kind::Quarry => buildcraft::QUARRY_SLOTS,
        Kind::Ic2Reactor => ic2reactor::SLOTS,
        _ => 3,
    };
    if entity.items.len() < slots_wanted {
        entity.items.resize(slots_wanted, None);
    }
    let mut state = load(&entity.nbt);
    let slots: [Stack; 3] = [stack(&entity.items[0]), stack(&entity.items[1]), stack(&entity.items[2])];
    let recipes = recipes_for(game, kind);
    let around = Surroundings {
        sunlit: kind == Kind::SolarPanel && sunlit(game, pos),
        sky: open_sky(game, pos),
        height: pos[1],
        cooled: kind.is_reactor() && energy::FACES.iter().any(|d| {
            game.native_block([pos[0] + d[0], pos[1] + d[1], pos[2] + d[2]]) == "minecraft:water"
        }),
    };
    let fx = machines::tick_in(kind, &mut state, &slots, &recipes, |_| 64, around);

    for &slot in &fx.consume {
        if let Some(s) = entity.items[slot].as_mut() {
            s.count = s.count.saturating_sub(1);
            if s.count == 0 {
                entity.items[slot] = None;
            }
        }
    }
    if fx.bucket && entity.items[FUEL].is_none() {
        entity.items[FUEL] = Some(new_stack("minecraft:bucket", 1));
    }
    if let Some((id, n)) = &fx.produce {
        match entity.items[OUTPUT].as_mut() {
            Some(s) if &*s.item == id.as_str() => s.count += n,
            Some(_) => {} // something else sits there (a reactor's waste): left out
            None => entity.items[OUTPUT] = Some(new_stack(id, *n)),
        }
    }

    if kind.offers(&state) > 0 {
        state.energy -= push_energy(game, pos, kind, kind.offers(&state));
    }

    if kind == Kind::Projector {
        project(game, pos, entity, &mut state);
    }
    if kind == Kind::Quarry {
        quarry(game, pos, entity, &mut state);
    }
    if kind == Kind::Turret {
        turret_tick(pos, entity, &mut state);
    }
    if kind == Kind::Ic2Reactor && reactor(game, pos, entity, &mut state) {
        save(&mut entity.nbt, &state);
        if let Some(blast) = explosives::blast(ids::NUCLEAR_EXPLOSIVE) {
            detonate(game, pos, blast);
        }
        return;
    }
    if fx.meltdown {
        save(&mut entity.nbt, &state);
        if let Some(blast) = explosives::blast(ids::NUCLEAR_EXPLOSIVE) {
            detonate(game, pos, blast);
        }
        return;
    }
    let lit = if fx.lit { "true" } else { "false" };
    if game.native_block_property(pos, "lit").is_some_and(|v| v != lit) {
        game.native_set_block_property(pos, "lit", lit);
    }
    entity.data = machines::menu_data(kind, &state).to_vec();
    save(&mut entity.nbt, &state);
}

/// Whether a redstone signal reaches the block at `pos`.
fn signalled(game: &mut Game<'_>, pos: [i32; 3]) -> bool {
    let dimension = game.dim.id().to_string();
    game.native_neighbor_signal(&dimension, pos) > 0
}

fn open_sky(game: &Game<'_>, pos: [i32; 3]) -> bool {
    game.native_light([pos[0], pos[1] + 1, pos[2]]).1 == 15
}

/// Open sky straight above, daytime, no rain, in the overworld.
fn sunlit(game: &Game<'_>, pos: [i32; 3]) -> bool {
    let time = game.native_day_time().rem_euclid(24_000);
    game.dim.id() == "minecraft:overworld"
        && game.native_weather().0 <= 0.2
        && (time < 12_300 || time > 23_850)
        && open_sky(game, pos)
}

/// Offers `offer` joules to the machines the wire reaches from `pos`;
/// returns what they took.
fn push_energy(game: &mut Game<'_>, pos: [i32; 3], from: Kind, offer: i64) -> i64 {
    let sinks = energy::reachable(pos, |p| {
        let b = game.native_block(p);
        if ids::is_wire(&b) {
            Node::Wire
        } else if ids::is_machine(&b) {
            Node::Machine
        } else {
            Node::Other
        }
    });
    let mut targets = Vec::new();
    let mut accepts = Vec::new();
    for p in sinks {
        let want = game.native_with_mod_block_entity(p, |be| {
            let k = Kind::of(&be.id)?;
            from.feeds(k).then(|| k.accepts(&load(&be.nbt)))
        });
        if let Some(Some(a)) = want.filter(|w| w.is_some_and(|a| a > 0)) {
            targets.push(p);
            accepts.push(a);
        }
    }
    let taken = energy::share(offer, &accepts);
    let mut total = 0;
    for (p, t) in targets.into_iter().zip(taken) {
        if t == 0 {
            continue;
        }
        game.native_with_mod_block_entity(p, |be| {
            let mut s = load(&be.nbt);
            s.energy += t;
            save(&mut be.nbt, &s);
        });
        total += t;
    }
    total
}

/// Clients see the machine's energy and progress (the menu reads its data
/// slots; this keeps a client's copy of the block entity in step too).
pub(super) fn update_tag(entity: &ModBlockEntity, tag: &mut Option<Compound>) {
    if Kind::of(&entity.id).is_none() {
        return;
    }
    let mut c = Compound::new();
    save(&mut c, &load(&entity.nbt));
    *tag = Some(c);
}

/// An explosive's tick: a redstone signal arms it (`lit` shows it), and when
/// its fuse runs out the block goes and the blast takes its place.
fn explosive_tick(game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity, blast: Blast) {
    let fuse = match entity.nbt.get(&JavaString::from("fuse")) {
        Some(Tag::Int(f)) => *f,
        _ => 0,
    };
    let powered = fuse == 0 && signalled(game, pos);
    let (fuse, boom) = explosives::tick(fuse, powered);
    entity.nbt.insert(JavaString::from("fuse"), Tag::Int(fuse));
    if fuse > 0 && game.native_block_property(pos, "lit").is_some_and(|v| v != "true") {
        game.native_set_block_property(pos, "lit", "true");
    }
    if boom {
        detonate(game, pos, blast);
    }
}

/// Removes the block at `pos` and sets off `blast` in its place.
fn detonate(game: &mut Game<'_>, pos: [i32; 3], blast: Blast) {
    let dimension = game.dim.id().to_string();
    let air = game.pack.assets.registry.default_state("minecraft:air").unwrap_or(BlockStateId(0));
    game.native_set_block(&dimension, pos, air, super::native_hooks::flags::UPDATE_ALL);
    let kind = if blast.breaks_blocks { NativeBlast::Tnt } else { NativeBlast::Keep };
    if blast.radius > 0.0 {
        for d in 0..blast.depth.max(1) {
            let center = [pos[0] as f64 + 0.5, pos[1] as f64 + 0.5 - d as f64, pos[2] as f64 + 0.5];
            game.native_explode(&dimension, center, blast.radius, blast.fire, kind);
        }
    }
    if blast.after == After::Emp {
        emp(game, pos);
    }
    let lines = explosives::commands(blast.after, pos);
    if !lines.is_empty() {
        if let Ok(mut q) = COMMANDS.lock() {
            q.extend(lines);
        }
    }
}

/// Console commands an explosive left for the end of the server tick (a block
/// entity's tick may not run commands itself).
static COMMANDS: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub(super) fn server_tick_end(game: &mut Game<'_>) {
    open_terminals(game);
    fire_shots(game);
    let lines = match COMMANDS.lock() {
        Ok(mut q) if !q.is_empty() => std::mem::take(&mut *q),
        _ => return,
    };
    for line in lines {
        game.native_run_command(&line);
    }
}

/// Drains every Voltz machine within the EMP's range.
fn emp(game: &mut Game<'_>, pos: [i32; 3]) {
    let r = explosives::EFFECT_RANGE;
    for dx in -r..=r {
        for dy in -r..=r {
            for dz in -r..=r {
                let p = [pos[0] + dx, pos[1] + dy, pos[2] + dz];
                if !ids::is_machine(&game.native_block(p)) {
                    continue;
                }
                game.native_with_mod_block_entity(p, |be| {
                    let mut s = load(&be.nbt);
                    s.energy = 0;
                    save(&mut be.nbt, &s);
                });
            }
        }
    }
}

/// New overworld chunks waiting for their ore (placed after the level's tick,
/// when the world is ready to change).
static NEW_CHUNKS: Mutex<Vec<(String, [i32; 2])>> = Mutex::new(Vec::new());

pub(super) fn chunk_load(dimension: &str, chunk: [i32; 2], new: bool) {
    if new && worldgen::ORES.iter().any(|o| o.dimension == dimension) {
        if let Ok(mut q) = NEW_CHUNKS.lock() {
            q.push((dimension.to_string(), chunk));
        }
    }
}

/// After a level's tick: missiles in flight there, and ore veins in up to 32 new chunks.
pub(super) fn level_tick_post(game: &mut Game<'_>, dimension: &str) {
    if game.dim.id() != dimension {
        return;
    }
    fly_missiles(game, dimension);
    let chunks: Vec<[i32; 2]> = match NEW_CHUNKS.lock() {
        Ok(mut q) if q.iter().any(|(d, _)| d == dimension) => {
            let mut mine = Vec::new();
            q.retain(|(d, c)| {
                if d == dimension && mine.len() < 32 {
                    mine.push(*c);
                    false
                } else {
                    true
                }
            });
            mine
        }
        _ => return,
    };
    let registry = &game.pack.assets.registry;
    let ores: Vec<_> = worldgen::ORES
        .iter()
        .filter(|o| o.dimension == dimension)
        .filter_map(|o| Some((registry.default_state(o.stone)?, registry.default_state(o.deepslate)?, o)))
        .collect();
    let flags = super::native_hooks::flags::UPDATE_CLIENTS;
    for chunk in chunks {
        for (ore, deep, spec) in &ores {
            for p in worldgen::positions(spec, game.level.seed, chunk) {
                let here = game.native_block(p);
                let state = if here == spec.hosts[0] {
                    *ore
                } else if here == spec.hosts[1] {
                    *deep
                } else {
                    continue;
                };
                game.native_set_block(dimension, p, state, flags);
            }
        }
    }
}

fn nbt_int(nbt: &Compound, key: &str) -> i32 {
    match nbt.get(&JavaString::from(key)) {
        Some(Tag::Int(v)) => *v,
        _ => 0,
    }
}

const SHAPES: [Shape; 4] = [Shape::Cube, Shape::Sphere, Shape::Wall, Shape::Tube];

/// A force field projector's tick: raise its field while it has a redstone
/// signal, a shape module and the energy to hold it; drop it otherwise.
fn project(game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity, state: &mut State) {
    let shape = entity.items.get(INPUT).and_then(|s| s.as_ref()).and_then(|s| Shape::of_module(&s.item));
    let scale = entity.items.get(FUEL).and_then(|s| s.as_ref()).filter(|s| &*s.item == forcefield::SCALE_MODULE).map_or(0, |s| s.count);
    let radius = forcefield::radius(scale);
    let powered = signalled(game, pos);
    let field = game.pack.assets.registry.default_state(forcefield::FIELD);
    let air = game.pack.assets.registry.default_state("minecraft:air").unwrap_or(BlockStateId(0));
    let dimension = game.dim.id().to_string();
    let flags = super::native_hooks::flags::UPDATE_CLIENTS;

    // The field it held last tick: dropped when the shape or size changed.
    let last = nbt_int(&entity.nbt, "field_shape");
    let last_r = nbt_int(&entity.nbt, "field_radius");
    let wanted = shape.filter(|_| powered).map(|s| (SHAPES.iter().position(|x| *x == s).unwrap_or(0) as i32 + 1, radius));
    if last > 0 && wanted != Some((last, last_r)) {
        let old = forcefield::shell(SHAPES[(last - 1).clamp(0, 3) as usize], pos, last_r);
        let mut left = false;
        let mut cleared = 0;
        for p in old {
            if game.native_block(p) != forcefield::FIELD {
                continue;
            }
            if cleared >= forcefield::BLOCKS_PER_TICK {
                left = true;
                break;
            }
            game.native_set_block(&dimension, p, air, flags);
            cleared += 1;
        }
        if !left {
            entity.nbt.insert(JavaString::from("field_shape"), Tag::Int(0));
        }
        return;
    }
    let (Some((code, r)), Some(field)) = (wanted, field) else { return };
    let shell = forcefield::shell(SHAPES[(code - 1) as usize], pos, r);
    let cost = forcefield::upkeep(shell.len());
    if state.energy < cost {
        // Out of energy: next tick drops the field.
        entity.nbt.insert(JavaString::from("field_radius"), Tag::Int(-1));
        return;
    }
    state.energy -= cost;
    let mut placed = 0;
    for p in shell {
        if placed >= forcefield::BLOCKS_PER_TICK {
            break;
        }
        if matches!(game.native_block(p).as_str(), "minecraft:air" | "minecraft:cave_air") {
            game.native_set_block(&dimension, p, field, flags);
            placed += 1;
        }
    }
    entity.nbt.insert(JavaString::from("field_shape"), Tag::Int(code));
    entity.nbt.insert(JavaString::from("field_radius"), Tag::Int(r));
}

const DIRECTIONS: [Direction; 6] =
    [Direction::Down, Direction::Up, Direction::North, Direction::South, Direction::West, Direction::East];

/// A wooden pipe: every few ticks, moves items from a container it touches
/// to another container on its pipe network.
fn pipe_tick(game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity) {
    let t = nbt_int(&entity.nbt, "t") + 1;
    entity.nbt.insert(JavaString::from("t"), Tag::Int(t % buildcraft::PULL_TICKS));
    if t < buildcraft::PULL_TICKS {
        return;
    }
    let dimension = game.native_dimension();
    let turn = nbt_int(&entity.nbt, "turn");
    entity.nbt.insert(JavaString::from("turn"), Tag::Int(turn.wrapping_add(1)));
    for (i, d) in energy::FACES.iter().enumerate() {
        let source = [pos[0] + d[0], pos[1] + d[1], pos[2] + d[2]];
        if buildcraft::is_pipe(&game.native_block(source)) {
            continue;
        }
        // The side of the source the pipe touches: the opposite of `i`.
        let source_face = DIRECTIONS[i ^ 1];
        let has_items = game
            .native_container_slots(dimension, source, source_face)
            .is_some_and(|slots| slots.iter().any(|s| s.stack.is_some() && s.can_take));
        if !has_items {
            continue;
        }
        let (targets, gold) = buildcraft::targets(pos, source, |p| {
            let b = game.native_block(p);
            if buildcraft::is_pipe(&b) {
                PipeNode::Pipe { gold: b == buildcraft::GOLD_PIPE }
            } else if game.native_container_slots(dimension, p, Direction::Up).is_some() {
                PipeNode::Container
            } else {
                PipeNode::Other
            }
        });
        if targets.is_empty() {
            continue;
        }
        let n = targets.len();
        for k in 0..n {
            let target = targets[(turn as usize + k) % n];
            let to_face = DIRECTIONS[buildcraft::face_toward(target.pos, target.pipe)];
            let moved =
                game.native_move_items(dimension, source, source_face, target.pos, to_face, None, if gold { 4 } else { 1 });
            if moved > 0 {
                return;
            }
        }
    }
}

/// A quarry's tick: dig the next block of its shaft into its own slots.
fn quarry(game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity, state: &mut State) {
    let t = nbt_int(&entity.nbt, "t") + 1;
    entity.nbt.insert(JavaString::from("t"), Tag::Int(t % buildcraft::QUARRY_TICKS));
    if t < buildcraft::QUARRY_TICKS || state.energy < buildcraft::QUARRY_JOULES {
        return;
    }
    let bottom = game.native_world_height().0;
    let dimension = game.dim.id().to_string();
    let air = game.pack.assets.registry.default_state("minecraft:air").unwrap_or(BlockStateId(0));
    let mut cursor = nbt_int(&entity.nbt, "cursor");
    // Skip what it leaves (air, fluids, bedrock) without spending a turn on it.
    for _ in 0..256 {
        let radius = buildcraft::quarry_radius(&entity.id);
        let Some(p) = buildcraft::quarry_target_r(pos, cursor, bottom, radius) else { break };
        let block = game.native_block(p);
        let max = if buildcraft::quarry_skips(&block) { None } else { game.native_max_stack_size(&block) };
        let Some(max) = max else {
            cursor += 1;
            continue;
        };
        // Into a slot holding the same item, else an empty one; a full
        // quarry waits.
        let slot = entity
            .items
            .iter()
            .position(|s| s.as_ref().is_some_and(|s| &*s.item == block.as_str() && s.count < max))
            .or_else(|| entity.items.iter().position(|s| s.is_none()));
        let Some(slot) = slot else { break };
        match entity.items[slot].as_mut() {
            Some(s) => s.count += 1,
            None => entity.items[slot] = Some(new_stack(&block, 1)),
        }
        game.native_set_block(&dimension, p, air, super::native_hooks::flags::UPDATE_ALL);
        state.energy -= buildcraft::QUARRY_JOULES;
        cursor += 1;
        break;
    }
    entity.nbt.insert(JavaString::from("cursor"), Tag::Int(cursor));
}

// ---- AE2 ----

fn load_store(be: &ModBlockEntity) -> Store {
    let mut store = Store::default();
    store.capacity = be.items.iter().flatten().map(|s| ae2::cell_capacity(&s.item) * s.count as u64).sum();
    if let Some(Tag::Compound(c)) = be.nbt.get(&JavaString::from("store")) {
        for (k, v) in c {
            if let Tag::Long(n) = v {
                if *n > 0 {
                    store.items.insert(k.to_string_lossy(), *n as u64);
                }
            }
        }
    }
    store
}

fn save_store(be: &mut ModBlockEntity, store: &Store) {
    let c: Compound = store.items.iter().map(|(k, v)| (JavaString::from(k.as_str()), Tag::Long(*v as i64))).collect();
    be.nbt.insert(JavaString::from("store"), Tag::Compound(c));
}

/// The network at `pos`: whether a controller keeps it online, its drives
/// and what they hold.
fn me_network(game: &mut Game<'_>, pos: [i32; 3]) -> (bool, Vec<[i32; 3]>, Vec<Store>) {
    let net = ae2::walk(pos, |p| game.native_block(p));
    let online = net.controllers.iter().any(|c| {
        game.native_with_mod_block_entity(*c, |be| load(&be.nbt).energy > 0).unwrap_or(false)
    });
    let mut drives = Vec::new();
    let mut stores = Vec::new();
    for d in net.drives {
        if let Some(s) = game.native_with_mod_block_entity(d, |be| load_store(be)) {
            drives.push(d);
            stores.push(s);
        }
    }
    (online, drives, stores)
}

fn write_stores(game: &mut Game<'_>, drives: &[[i32; 3]], stores: &[Store]) {
    for (d, s) in drives.iter().zip(stores) {
        game.native_with_mod_block_entity(*d, |be| save_store(be, s));
    }
}

struct Session {
    pos: [i32; 3],
    name: String,
    page: usize,
}

static SESSIONS: Mutex<Option<HashMap<u128, Session>>> = Mutex::new(None);
static OPEN_TERMINALS: Mutex<Vec<(u128, String, [i32; 3])>> = Mutex::new(Vec::new());

/// A right click on an ME terminal opens its menu (at the end of the tick:
/// a player's hook can't open one).
pub(super) fn right_click_block(game: &mut Game<'_>, pos: [i32; 3], canceled: &mut bool) {
    let block = game.native_block(pos);
    if block == weapons::LAUNCHER {
        aim_launcher(game, pos);
        return;
    }
    if block != ae2::TERMINAL {
        return;
    }
    *canceled = true;
    if let (Some(p), Ok(mut q)) = (game.native_acting_player(), OPEN_TERMINALS.lock()) {
        q.push((p.uuid, p.name, pos));
    }
}

fn button(item: &str, count: u32) -> Option<ItemStack> {
    Some(new_stack(item, count.clamp(1, 64)))
}

/// The terminal's 54 slots for `page`: 45 items, then prev / info / next.
fn terminal_items(game: &Game<'_>, stores: &[Store], online: bool, page: usize) -> Vec<Option<ItemStack>> {
    let mut items = vec![None; 54];
    if online {
        let list = ae2::listing(stores);
        for (i, (id, n)) in list.iter().skip(page * ae2::PAGE).take(ae2::PAGE).enumerate() {
            let max = game.native_max_stack_size(id).unwrap_or(64) as u64;
            items[i] = Some(new_stack(id, (*n).min(max).max(1) as u32));
        }
        items[ae2::PREV_SLOT as usize] = button("minecraft:arrow", 1);
        items[ae2::NEXT_SLOT as usize] = button("minecraft:spectral_arrow", 1);
        items[ae2::INFO_SLOT as usize] = button("minecraft:paper", (page + 1) as u32);
    } else {
        items[ae2::INFO_SLOT as usize] = button("minecraft:barrier", 1);
    }
    items
}

fn open_terminals(game: &mut Game<'_>) {
    let opens: Vec<_> = match OPEN_TERMINALS.lock() {
        Ok(mut q) if !q.is_empty() => std::mem::take(&mut *q),
        _ => return,
    };
    for (uuid, name, pos) in opens {
        let (online, _, stores) = me_network(game, pos);
        let items = terminal_items(game, &stores, online, 0);
        let used: u64 = stores.iter().map(Store::used).sum();
        let cap: u64 = stores.iter().map(|s| s.capacity).sum();
        let title = if online { format!("ME Terminal  {used}/{cap}") } else { "ME Terminal (offline)".to_string() };
        if game.native_open_menu(uuid, 6, horizon_client::text::Text::literal(title), items).is_some() {
            if let Ok(mut s) = SESSIONS.lock() {
                s.get_or_insert_with(HashMap::new).insert(uuid, Session { pos, name, page: 0 });
            }
        }
    }
}

pub(super) fn menu_clicked(game: &mut Game<'_>, uuid: u128, slot: i32, button_id: i32, mode: i32) {
    let (pos, name, mut page) = match SESSIONS.lock() {
        Ok(s) => match s.as_ref().and_then(|m| m.get(&uuid)) {
            Some(x) => (x.pos, x.name.clone(), x.page),
            None => return,
        },
        Err(_) => return,
    };
    let (online, drives, mut stores) = me_network(game, pos);
    if !online {
        return;
    }
    let list = ae2::listing(&stores);
    if slot == ae2::PREV_SLOT {
        page = page.saturating_sub(1);
    } else if slot == ae2::NEXT_SLOT {
        page = (page + 1).min(ae2::pages(list.len()) - 1);
    } else if (0..ae2::PAGE as i32).contains(&slot) {
        // Take: a left click a stack, a right click one.
        if let Some((id, _)) = list.get(page * ae2::PAGE + slot as usize) {
            let max = game.native_max_stack_size(id).unwrap_or(64) as u64;
            let want = if button_id == 1 { 1 } else { max };
            let got = ae2::take(&mut stores, id, want);
            if got > 0 {
                game.native_run_command(&format!("give {name} {id} {got}"));
                write_stores(game, &drives, &stores);
            }
        }
    } else if let Some(inv) = super::native_menus::native_menu_inventory_slot(54, slot) {
        // Store a stack from the player's inventory (plain items only).
        let stack = game.native_player_stacks(uuid).and_then(|s| s.get(inv).cloned().flatten());
        if let Some(stack) = stack.filter(|s| s.extra.is_none() && s.damage == 0) {
            let put = ae2::insert(&mut stores, &stack.item, stack.count as u64);
            if put > 0 {
                let left = stack.count as u64 - put;
                let with = if left == 0 { "minecraft:air".to_string() } else { format!("{} {left}", stack.item) };
                game.native_run_command(&format!("item replace entity {name} container.{inv} with {with}"));
                write_stores(game, &drives, &stores);
            }
        }
    }
    let _ = mode;
    if let Ok(mut s) = SESSIONS.lock() {
        if let Some(x) = s.as_mut().and_then(|m| m.get_mut(&uuid)) {
            x.page = page;
        }
    }
    let items = terminal_items(game, &stores, online, page);
    game.native_set_menu_items(uuid, items);
}

pub(super) fn menu_closed(uuid: u128) {
    if let Ok(mut s) = SESSIONS.lock() {
        if let Some(m) = s.as_mut() {
            m.remove(&uuid);
        }
    }
}

/// An import bus pulls a few items at a time from the containers it touches
/// into its network.
fn import_tick(game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity) {
    let t = nbt_int(&entity.nbt, "t") + 1;
    entity.nbt.insert(JavaString::from("t"), Tag::Int(t % ae2::IMPORT_TICKS));
    if t < ae2::IMPORT_TICKS {
        return;
    }
    let (online, drives, mut stores) = me_network(game, pos);
    let room: u64 = stores.iter().map(Store::room).sum();
    if !online || room == 0 {
        return;
    }
    let dimension = game.native_dimension();
    for (i, d) in energy::FACES.iter().enumerate() {
        let p = [pos[0] + d[0], pos[1] + d[1], pos[2] + d[2]];
        let b = game.native_block(p);
        if ae2::is_device(&b) || b == ae2::CABLE {
            continue;
        }
        let face = DIRECTIONS[i ^ 1];
        let Some(slots) = game.native_container_slots(dimension, p, face) else { continue };
        let pick = slots.iter().find(|s| s.can_take && s.stack.as_ref().is_some_and(|st| st.extra.is_none() && st.damage == 0));
        let Some(pick) = pick else { continue };
        let n = pick.stack.as_ref().map_or(0, |s| s.count).min(16).min(room as u32);
        if let Some(got) = game.native_extract_item(dimension, p, face, pick.slot, n) {
            ae2::insert(&mut stores, &got.item, got.count as u64);
            write_stores(game, &drives, &stores);
        }
        return;
    }
}

// ---- IC2 nuclear reactor ----

/// A reactor's tick: with a redstone signal, once a second run its grid
/// (`ic2reactor::step`), and every tick add the energy that second makes.
/// Returns true when it melts down.
fn reactor(game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity, state: &mut State) -> bool {
    let powered = signalled(game, pos);
    if !powered {
        entity.nbt.insert(JavaString::from("eu"), Tag::Int(0));
        return false;
    }
    let t = nbt_int(&entity.nbt, "t") + 1;
    entity.nbt.insert(JavaString::from("t"), Tag::Int(t % ic2reactor::TICKS));
    if t >= ic2reactor::TICKS {
        // The grid, with each slot's heat and use kept by item id.
        let saved = match entity.nbt.get(&JavaString::from("cells")) {
            Some(Tag::Compound(c)) => c.clone(),
            _ => Compound::new(),
        };
        let mut grid: Vec<Option<Cell>> = vec![None; ic2reactor::SLOTS];
        for (i, slot) in entity.items.iter().enumerate().take(ic2reactor::SLOTS) {
            let Some(stack) = slot else { continue };
            let Some(part) = ic2reactor::part(&stack.item) else { continue };
            let (mut heat, mut used) = (0, 0);
            if let Some(Tag::Compound(c)) = saved.get(&JavaString::from(i.to_string().as_str())) {
                if matches!(c.get(&JavaString::from("id")), Some(Tag::String(s)) if s.to_string_lossy() == *stack.item) {
                    heat = match c.get(&JavaString::from("heat")) { Some(Tag::Int(v)) => *v, _ => 0 };
                    used = match c.get(&JavaString::from("used")) { Some(Tag::Int(v)) => *v, _ => 0 };
                }
            }
            grid[i] = Some(Cell { part, heat, used });
        }
        let mut hull = nbt_int(&entity.nbt, "hull");
        let out = ic2reactor::step(&mut grid, &mut hull);
        for i in &out.destroyed {
            entity.items[*i] = None;
        }
        for i in &out.spent {
            let next = entity.items[*i].as_ref().and_then(|s| ic2reactor::depleted(&s.item));
            entity.items[*i] = next.map(|id| new_stack(id, 1));
            grid[*i] = None;
        }
        let mut cells = Compound::new();
        for (i, c) in grid.iter().enumerate() {
            let (Some(c), Some(stack)) = (c, entity.items.get(i).and_then(|s| s.as_ref())) else { continue };
            let mut e = Compound::new();
            e.insert(JavaString::from("id"), Tag::String(JavaString::from(&*stack.item)));
            e.insert(JavaString::from("heat"), Tag::Int(c.heat));
            e.insert(JavaString::from("used"), Tag::Int(c.used));
            cells.insert(JavaString::from(i.to_string().as_str()), Tag::Compound(e));
        }
        entity.nbt.insert(JavaString::from("cells"), Tag::Compound(cells));
        entity.nbt.insert(JavaString::from("hull"), Tag::Int(hull));
        entity.nbt.insert(JavaString::from("eu"), Tag::Int(out.eu_per_tick.min(i32::MAX as i64) as i32));
        if out.meltdown {
            return true;
        }
    }
    let eu = nbt_int(&entity.nbt, "eu") as i64;
    state.energy = (state.energy + eu * ic2reactor::JOULES_PER_EU).min(Kind::Ic2Reactor.capacity());
    false
}

// ---- Weapons: missiles, turrets, guns ----

fn queue(lines: impl IntoIterator<Item = String>) {
    if let Ok(mut q) = COMMANDS.lock() {
        q.extend(lines);
    }
}

struct Flight {
    id: u64,
    dimension: String,
    from: [f64; 3],
    to: [f64; 3],
    t: i32,
    total: i32,
    warhead: &'static str,
}

static FLIGHTS: Mutex<Vec<Flight>> = Mutex::new(Vec::new());
static NEXT_FLIGHT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
/// Each player's last designated target.
static TARGETS: Mutex<Option<HashMap<u128, [i32; 3]>>> = Mutex::new(None);

/// A right click with an item on a block: the target designator marks a
/// target, or hands its target to the launcher it is used on.
pub(super) fn use_item_on(game: &mut Game<'_>, item: &str, pos: [i32; 3], claimed: &mut bool) {
    if item != weapons::DESIGNATOR {
        return;
    }
    let Some(player) = game.native_acting_player() else { return };
    *claimed = true;
    let mut targets = match TARGETS.lock() {
        Ok(t) => t,
        Err(_) => return,
    };
    let map = targets.get_or_insert_with(HashMap::new);
    let line = if game.native_block(pos) == weapons::LAUNCHER {
        match map.get(&player.uuid).copied() {
            Some(t) => {
                game.native_with_mod_block_entity(pos, |be| {
                    for (k, v) in [("tx", t[0]), ("ty", t[1]), ("tz", t[2]), ("armed", 1)] {
                        be.nbt.insert(JavaString::from(k), Tag::Int(v));
                    }
                });
                format!("Launcher aimed at {} {} {}", t[0], t[1], t[2])
            }
            None => "Mark a target first: right-click a block with the designator".to_string(),
        }
    } else {
        map.insert(player.uuid, pos);
        format!("Target set: {} {} {}", pos[0], pos[1], pos[2])
    };
    drop(targets);
    game.native_action_bar(player.uuid, horizon_client::text::Text::literal(line));
}

/// Opening a launcher aims it at the opener's last designated target.
fn aim_launcher(game: &mut Game<'_>, pos: [i32; 3]) {
    let Some(player) = game.native_acting_player() else { return };
    let target = TARGETS.lock().ok().and_then(|t| t.as_ref().and_then(|m| m.get(&player.uuid).copied()));
    let Some(t) = target else { return };
    game.native_with_mod_block_entity(pos, |be| {
        for (k, v) in [("tx", t[0]), ("ty", t[1]), ("tz", t[2]), ("armed", 1)] {
            be.nbt.insert(JavaString::from(k), Tag::Int(v));
        }
    });
    let line = format!("Launcher aimed at {} {} {}", t[0], t[1], t[2]);
    game.native_action_bar(player.uuid, horizon_client::text::Text::literal(line));
}

/// A launcher fires the missile in its slot at its target on a rising redstone signal.
fn launcher_tick(game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity) {
    if entity.items.len() < 5 {
        entity.items.resize(5, None);
    }
    let powered = signalled(game, pos);
    let was = nbt_int(&entity.nbt, "powered") == 1;
    entity.nbt.insert(JavaString::from("powered"), Tag::Int(powered as i32));
    if !powered || was || nbt_int(&entity.nbt, "armed") != 1 {
        return;
    }
    let Some(missile) = entity.items[0].as_ref().map(|s| s.item.to_string()) else { return };
    let Some(warhead) = weapons::warhead(&missile) else { return };
    if let Some(s) = entity.items[0].as_mut() {
        s.count -= 1;
        if s.count == 0 {
            entity.items[0] = None;
        }
    }
    let from = [pos[0] as f64 + 0.5, pos[1] as f64 + 1.5, pos[2] as f64 + 0.5];
    let to = [nbt_int(&entity.nbt, "tx") as f64 + 0.5, nbt_int(&entity.nbt, "ty") as f64 + 0.5, nbt_int(&entity.nbt, "tz") as f64 + 0.5];
    let dist = ((to[0] - from[0]).powi(2) + (to[2] - from[2]).powi(2)).sqrt();
    let id = NEXT_FLIGHT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    queue([
        format!(
            "summon minecraft:item_display {} {} {} {{item:{{id:\"{missile}\",count:1}},Tags:[\"voltz_missile\",\"voltz_m{id}\"]}}",
            from[0], from[1], from[2]
        ),
        format!("playsound minecraft:entity.firework_rocket.launch block @a {} {} {} 4 0.6", from[0], from[1], from[2]),
    ]);
    if let Ok(mut f) = FLIGHTS.lock() {
        f.push(Flight {
            id,
            dimension: game.dim.id().to_string(),
            from,
            to,
            t: 0,
            total: weapons::flight_ticks(dist),
            warhead,
        });
    }
}

fn fly_missiles(game: &mut Game<'_>, dimension: &str) {
    let mut landed = Vec::new();
    {
        let Ok(mut flights) = FLIGHTS.lock() else { return };
        let mut lines = Vec::new();
        for f in flights.iter_mut().filter(|f| f.dimension == dimension) {
            f.t += 1;
            let p = weapons::missile_pos(f.from, f.to, f.t, f.total);
            lines.push(format!("tp @e[tag=voltz_m{},limit=1] {:.2} {:.2} {:.2}", f.id, p[0], p[1], p[2]));
            lines.push(format!("particle minecraft:flame {:.2} {:.2} {:.2} 0.1 0.1 0.1 0.01 4 force", p[0], p[1] - 0.5, p[2]));
            if f.t >= f.total {
                lines.push(format!("kill @e[tag=voltz_m{}]", f.id));
                landed.push((f.to, f.warhead));
            }
        }
        flights.retain(|f| f.t < f.total);
        drop(flights);
        queue(lines);
    }
    for (to, warhead) in landed {
        let Some(blast) = explosives::blast(warhead) else { continue };
        let kind = if blast.breaks_blocks { NativeBlast::Tnt } else { NativeBlast::Keep };
        let dimension = game.dim.id().to_string();
        if blast.radius > 0.0 {
            for d in 0..blast.depth.max(1) {
                game.native_explode(&dimension, [to[0], to[1] - d as f64, to[2]], blast.radius, blast.fire, kind);
            }
        }
        let at = [to[0].floor() as i32, to[1].floor() as i32, to[2].floor() as i32];
        if blast.after == After::Emp {
            emp(game, at);
        }
        queue(explosives::commands(blast.after, at));
    }
}

/// A turret with energy shoots the nearest hostile mob in range now and then.
fn turret_tick(pos: [i32; 3], entity: &mut ModBlockEntity, state: &mut State) {
    let Some((range, damage, cost, interval, kind)) = weapons::turret(&entity.id) else { return };
    let t = nbt_int(&entity.nbt, "t") + 1;
    entity.nbt.insert(JavaString::from("t"), Tag::Int(t % interval));
    if t < interval || state.energy < cost {
        return;
    }
    state.energy -= cost;
    let (x, y, z) = (pos[0] as f64 + 0.5, pos[1] as f64 + 1.0, pos[2] as f64 + 0.5);
    let target = weapons::target_selector(range as f64);
    let lines = vec![
        format!("execute positioned {x} {y} {z} run damage {target} {damage} {kind}"),
        format!("execute positioned {x} {y} {z} at {target} run particle minecraft:crit ~ ~1 ~ 0.2 0.4 0.2 0.3 8"),
        format!("particle minecraft:smoke {x} {y} {z} 0.1 0.1 0.1 0.01 3"),
    ];
    queue(lines);
}

struct Shot {
    uuid: u128,
    gun: &'static str,
}

static SHOTS: Mutex<Vec<Shot>> = Mutex::new(Vec::new());
static LAST_SHOT: Mutex<Option<HashMap<u128, u64>>> = Mutex::new(None);
static SERVER_TICKS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// A right click with a gun in the air: it fires at the end of the tick.
pub(super) fn use_item(game: &mut Game<'_>, item: &str, claimed: &mut bool) {
    let Some(gun) = weapons::gun(item) else { return };
    let Some(player) = game.native_acting_player() else { return };
    *claimed = true;
    if let Ok(mut s) = SHOTS.lock() {
        s.push(Shot { uuid: player.uuid, gun: gun.id });
    }
}

fn fire_shots(game: &mut Game<'_>) {
    let now = SERVER_TICKS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let shots: Vec<Shot> = match SHOTS.lock() {
        Ok(mut s) if !s.is_empty() => std::mem::take(&mut *s),
        _ => return,
    };
    let players = game.native_server_players();
    for shot in shots {
        let Some(gun) = weapons::gun(shot.gun) else { continue };
        let Some(p) = players.iter().find(|p| p.uuid == shot.uuid) else { continue };
        if let Ok(mut l) = LAST_SHOT.lock() {
            let m = l.get_or_insert_with(HashMap::new);
            if m.get(&shot.uuid).is_some_and(|t| now < t + gun.cooldown as u64) {
                continue;
            }
            m.insert(shot.uuid, now);
        }
        // Creative players shoot for free; the rest spend one ammo item.
        if p.game_mode != 1 {
            let has = game
                .native_player_stacks(shot.uuid)
                .is_some_and(|s| s.iter().flatten().any(|st| &*st.item == gun.ammo));
            if !has {
                game.native_action_bar(shot.uuid, horizon_client::text::Text::literal("Out of ammo"));
                continue;
            }
            game.native_run_command(&format!("clear {} {} 1", p.name, gun.ammo));
        }
        let d = weapons::look(p.yaw, p.pitch);
        let (ex, ey, ez) = (p.position[0], p.position[1] + 1.62, p.position[2]);
        match gun.projectile {
            weapons::Projectile::Arrow => {
                // A bullet: look for a target every couple of blocks along the line of sight.
                let near = weapons::target_selector(1.3);
                let from = format!("execute positioned {ex:.2} {ey:.2} {ez:.2} rotated {:.1} {:.1} positioned", p.yaw, p.pitch);
                for step in (weapons::BULLET_STEP..=weapons::BULLET_RANGE).step_by(weapons::BULLET_STEP as usize) {
                    game.native_run_command(&format!("{from} ^ ^ ^{step} run damage {near} {:.1} minecraft:arrow", gun.damage));
                    if step % 6 == 0 {
                        game.native_run_command(&format!("{from} ^ ^ ^{step} run particle minecraft:crit ~ ~ ~ 0 0 0 0 1"));
                    }
                }
            }
            weapons::Projectile::Rocket(power) => {
                let m = [d[0] * gun.speed, d[1] * gun.speed, d[2] * gun.speed];
                let (x, y, z) = (ex + d[0] * 1.5, ey + d[1] * 1.5, ez + d[2] * 1.5);
                game.native_run_command(&format!(
                    "summon minecraft:fireball {x:.2} {y:.2} {z:.2} {{Motion:[{:.3},{:.3},{:.3}],acceleration_power:0.1d,ExplosionPower:{power}b}}",
                    m[0], m[1], m[2]
                ));
            }
        }
        let (x, y, z) = (ex, ey, ez);
        game.native_run_command(&format!("playsound minecraft:entity.generic.explode player @a {x:.1} {y:.1} {z:.1} 0.4 2"));
    }
}
