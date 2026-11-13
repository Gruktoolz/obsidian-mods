// Deepbore Quarry in the game: the edges between the game's types and the rules
// (`game::deepbore_quarry`, runtime.rs). Added to the game as
// `game::deepbore_quarry_glue`; each small helper file calls it from a hook point.
//
// A quarry's state lives in its block entity, so it moves with the chunk's save:
// `nbt` holds the rules' `Quarry`, `items` the slots, `data` what its screen shows.
// What belongs to the world (the server's limit, the chunks held) lives in files
// in the world's folder.

use crate::game::{
    Game, GameLevel, LevelDimension, deepbore_quarry as rules,
    native_block_entity_draw::{BlockEntityDraw, CubeFace},
};
use horizon_client::{
    blocks::Fluid,
    containers::{BlockEntity, ModBlockEntity, ModContainer},
    hud::HudBatch,
    items::ItemStack,
    lang,
    mod_api::{NativeCommand, NativeMod, NativeScreen, ServerTickPhase},
    particles::{ParticleOptions, ParticleType},
    text::{self, ClickEvent, Style, Text},
};
use horizon_core::{geometry::Direction, position::BlockPos};
use horizon_mods::content::{Block, BlockProperties, Item, Rarity};
use horizon_world::{
    nbt::{Compound, JavaString, Tag},
    registry::BlockStateId,
};
use rules::{Action, Cell, Input, Quarry, Upgrade, Upgrades, store};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard},
};

// ---- the world's files -----------------------------------------------------

struct WorldState {
    dir: PathBuf,
    config: rules::Config,
    loads: rules::Loads,
    /// Not yet saved. Cleared only by a save that succeeded, so a failed one is tried again.
    loads_changed: bool,
    /// A damaged loads.json couldn't be moved aside: it isn't written over in this session.
    loads_blocked: bool,
    /// The last save of loads.json failed (logged once, not every tick).
    loads_failing: bool,
    /// Said in chat at the next server tick: a world file was damaged.
    notices: Vec<String>,
    /// Forced chunks to change at the end of the server tick: a player's click
    /// and a block entity's tick can't run console commands.
    commands: Vec<Force>,
    /// The first tick of this world has run.
    woken: bool,
}

struct Force {
    dimension: String,
    add: bool,
    chunk: [i32; 2],
}

static WORLD: Mutex<Option<WorldState>> = Mutex::new(None);

/// A world file, or an empty one when there is none. A damaged file is moved aside
/// (`<name>.corrupt-<seconds>`), logged as an error and said in chat; the answer's
/// `false` means it couldn't be moved, so it must not be written over.
fn read_json<T: serde::de::DeserializeOwned + Default>(path: &Path, notices: &mut Vec<String>) -> (T, bool) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    match store::read_json(path, now) {
        Ok(value) => (value, true),
        Err(store::Damaged { error, kept }) => {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("file");
            let (line, writable) = match &kept {
                Some(kept) => {
                    let kept = kept.file_name().and_then(|n| n.to_str()).unwrap_or("?");
                    (format!("{name} could not be read ({error}); it was kept as {kept} and a new one is started."), true)
                }
                None => (format!("{name} could not be read ({error}) nor moved aside; it is left as it is and not saved over until the world is reopened."), false),
            };
            log::error!("[deepbore_quarry] {}: {line}", path.display());
            notices.push(format!("Deepbore Quarry: {line}"));
            (T::default(), writable)
        }
    }
}

/// Writes a world file and logs a failure; the caller keeps what it couldn't save.
fn write_json<T: serde::Serialize>(path: &Path, value: &T) -> std::io::Result<()> {
    store::write_json(path, value).inspect_err(|e| {
        log::error!("[deepbore_quarry] {} was not saved: {e}", path.display());
    })
}

/// The state of the open world, read on first use and again when another world opens.
fn world(game: &Game<'_>) -> Option<MutexGuard<'static, Option<WorldState>>> {
    let dir = game.native_world_dir()?.join(rules::DIR);
    let mut state = WORLD.lock().unwrap_or_else(|e| e.into_inner());
    if state.as_ref().is_none_or(|s| s.dir != dir) {
        let mut notices = Vec::new();
        let (config, _) = read_json(&dir.join("config.json"), &mut notices);
        let (loads, loads_writable) = read_json(&dir.join("loads.json"), &mut notices);
        *state = Some(WorldState {
            config,
            loads,
            loads_changed: false,
            loads_blocked: !loads_writable,
            loads_failing: false,
            notices,
            commands: Vec::new(),
            woken: false,
            dir,
        });
    }
    Some(state)
}

fn forceload(commands: &mut Vec<Force>, dimension: &str, change: rules::Change) {
    for (add, chunks) in [(true, change.add), (false, change.remove)] {
        for chunk in chunks {
            commands.push(Force {
                dimension: dimension.to_owned(),
                add,
                chunk,
            });
        }
    }
}

/// The end of each server tick: what had to wait for it.
pub(in crate::game) fn server_tick(game: &mut Game<'_>, phase: ServerTickPhase) {
    if !matches!(phase, ServerTickPhase::ServerTickEnd { .. }) {
        return;
    }
    let (commands, wake) = {
        let Some(mut guard) = world(game) else {
            return;
        };
        let st = guard.as_mut().expect("opened");
        let wake = if st.woken {
            Vec::new()
        } else {
            st.loads.dimensions()
        };
        st.woken = true;
        (std::mem::take(&mut st.commands), wake)
    };
    // A level no player has entered isn't open, so the chunks it forces wouldn't
    // tick and its quarries would wait for a visitor. A command in it opens it.
    for dimension in wake {
        game.native_run_command(&format!("execute in {dimension} run forceload query"));
    }
    for force in commands {
        let [x, z] = force.chunk;
        let dimension = &force.dimension;
        if !force.add {
            let mut guard = WORLD.lock().unwrap_or_else(|e| e.into_inner());
            let ours = guard
                .as_mut()
                .is_none_or(|st| st.loads.take_ours(dimension, force.chunk));
            drop(guard);
            if ours {
                game.native_run_command(&format!(
                    "execute in {dimension} run forceload remove {} {}",
                    x * 16,
                    z * 16
                ));
            }
        } else if !game.native_run_command(&format!(
            "execute in {dimension} run forceload add {} {}",
            x * 16,
            z * 16
        )) {
            // Already forced, by an operator or a command block: it stays when we let go.
            if let Some(st) = WORLD.lock().unwrap_or_else(|e| e.into_inner()).as_mut() {
                st.loads.not_ours(dimension, force.chunk);
                st.loads_changed = true;
            }
        }
    }
    if let Some(st) = WORLD.lock().unwrap_or_else(|e| e.into_inner()).as_mut() {
        if !st.loads_blocked {
            let path = st.dir.join("loads.json");
            // A failure leaves loads_changed set: the next tick tries again. It is
            // logged once, when saving starts failing, and again when it works.
            match store::save_if_changed(&mut st.loads_changed, || store::write_json(&path, &st.loads)) {
                Err(e) if !st.loads_failing => {
                    log::error!("[deepbore_quarry] {} was not saved, trying again each tick: {e}", path.display());
                    st.loads_failing = true;
                }
                Ok(()) if st.loads_failing => {
                    log::info!("[deepbore_quarry] {} is saved again", path.display());
                    st.loads_failing = false;
                }
                _ => {}
            }
        }
    }
}

// ---- declarations ----------------------------------------------------------

/// The quarry. A furnace is its template: it opens a menu when used, and its
/// `facing` is the side towards the player who placed it.
pub(in crate::game) fn blocks(blocks: &mut Vec<Block>) {
    blocks.push(Block {
        id: rules::BLOCK.into(),
        template: "minecraft:furnace".into(),
        properties: BlockProperties {
            hardness: Some(3.5),
            explosion_resistance: Some(6.0),
            requires_tool: Some(true),
            sounds: Some("minecraft:netherite_block".into()),
            ..Default::default()
        },
        item: true,
        ..Default::default()
    });
}

pub(in crate::game) fn items(items: &mut Vec<Item>) {
    for id in [
        "deepbore_quarry:area_upgrade",
        "deepbore_quarry:speed_upgrade",
        "deepbore_quarry:fluids_upgrade",
        "deepbore_quarry:chunk_upgrade",
    ] {
        items.push(Item {
            id: id.into(),
            template: "minecraft:paper".into(),
            max_stack_size: Some(16),
            rarity: Some(Rarity::Uncommon),
            ..Default::default()
        });
    }
}

/// A player joins. The assets half, if it's there, reads the tag to know this
/// half is too (content/data/deepbore_quarry/function/check.mcfunction). Without the
/// assets half, which brings the block's loot table, they hear what's missing and
/// where to get it, in plain text, as the language files are gone too.
pub(in crate::game) fn logged_in(game: &mut Game<'_>, player: u128, name: &str) {
    // A damaged world file, moved aside when the world opened: everyone who joins hears it.
    let notices = world(game).and_then(|guard| guard.as_ref().map(|st| st.notices.clone()));
    let red = Style {
        color: Some(text::RED),
        ..Style::default()
    };
    for line in notices.unwrap_or_default() {
        game.native_tell(player, Text::styled(line, red.clone()));
    }
    let registry = &game.pack.assets.registry;
    let assets = registry.default_state(rules::BLOCK).is_some_and(|state| {
        world_items!(game)
            .block_loot_table(registry, state)
            .is_some()
    });
    if assets {
        game.native_run_command(&format!("tag {name} add deepbore_quarry.native"));
        return;
    }
    const ASSETS_PAGE: &str = "https://playhorizon.dev/mods/deepbore_quarry";
    let link = Style {
        color: Some(text::AQUA),
        underlined: Some(true),
        click: Some(ClickEvent::OpenUrl(ASSETS_PAGE.to_owned())),
        ..Style::default()
    };
    for line in [
        "Deepbore Quarry: the assets mod is missing. Install deepbore_quarry too: ",
        "Deepbore Quarry: не установлен мод с ассетами. Поставьте ещё deepbore_quarry: ",
    ] {
        let line = Text::literal(line).append(Text::styled(ASSETS_PAGE, link.clone()));
        game.native_tell(player, line);
    }
}

/// The row in Options > Mods.
pub(in crate::game) fn listed(mods: &mut Vec<NativeMod>) {
    mods.push(NativeMod {
        id: rules::ID.into(),
        title: format!("Deepbore Quarry {}", rules::VERSION),
        description: lang::get_or(
            "deepbore_quarry.mod.description",
            "A machine that digs a square pit down to bedrock and fills a chest.",
        ),
        keys: Vec::new(),
    });
}

pub(in crate::game) fn block_entity(block: &str, entity: &mut Option<(String, Compound)>) {
    if block == rules::BLOCK && entity.is_none() {
        *entity = Some((rules::BLOCK.to_owned(), Compound::default()));
    }
}

pub(in crate::game) fn block_entity_ticks(block: &str, ticks: &mut bool) {
    if block == rules::BLOCK {
        *ticks = true;
    }
}

/// Hoppers put fuel in, from any side, and take nothing out.
pub(in crate::game) fn container(block: &str, container: &mut Option<ModContainer>) {
    if block == rules::BLOCK {
        *container = Some(ModContainer {
            slots: rules::SLOTS,
            faces: std::array::from_fn(|_| vec![rules::FUEL_SLOT]),
            inputs: vec![rules::FUEL_SLOT],
            outputs: Vec::new(),
            menu: Some(rules::MENU.to_owned()),
        });
    }
}

pub(in crate::game) fn commands(commands: &mut Vec<NativeCommand>) {
    commands.push(NativeCommand::op("deepbore maxsize", 2));
    commands.push(NativeCommand::op("deepbore maxsize <size:text>", 2));
}

pub(in crate::game) fn perform(game: &mut Game<'_>, line: &str, ran: &mut bool) {
    let mut words = line.split(' ');
    if *ran || words.next() != Some("deepbore") {
        return;
    }
    *ran = true;
    let Some(mut guard) = world(game) else {
        return;
    };
    let st = guard.as_mut().expect("opened");
    let reply = match (words.next(), words.next()) {
        (Some("maxsize"), Some(arg)) => match rules::parse_max_side(arg) {
            Some(side) => {
                st.config.max_side = side;
                match write_json(&st.dir.join("config.json"), &st.config) {
                    Ok(()) => size_line(side),
                    Err(e) => size_line(side).append(Text::literal(format!(" (not saved: {e})"))),
                }
            }
            None => Text::translate("deepbore_quarry.maxsize.usage", &[]),
        },
        (Some("maxsize"), None) => size_line(st.config.max_side),
        _ => Text::translate("deepbore_quarry.maxsize.usage", &[]),
    };
    drop(guard);
    game.native_feedback(reply);
}

fn size_line(side: i32) -> Text {
    let n = Text::literal(side.to_string());
    Text::translate("deepbore_quarry.maxsize.set", &[n.clone(), n])
}

// ---- a quarry's state ------------------------------------------------------

fn key_name() -> JavaString {
    JavaString::from("quarry")
}

fn load(entity: &ModBlockEntity) -> Option<Quarry> {
    match entity.nbt.get(&key_name()) {
        Some(Tag::ByteArray(bytes)) => serde_json::from_slice(bytes).ok(),
        _ => None,
    }
}

fn store(entity: &mut ModBlockEntity, quarry: &Quarry) {
    if let Ok(bytes) = serde_json::to_vec(quarry) {
        entity.nbt.insert(key_name(), Tag::ByteArray(bytes));
    }
}

fn dig_direction(game: &Game<'_>, pos: [i32; 3]) -> rules::Facing {
    game.native_block_property(pos, "facing")
        .and_then(|f| rules::Facing::away_from(&f))
        .unwrap_or(rules::Facing::North)
}

fn upgrade_slots(entity: &ModBlockEntity) -> [Option<Upgrade>; rules::UPGRADE_SLOTS] {
    std::array::from_fn(|i| {
        entity
            .items
            .get(rules::FIRST_UPGRADE + i)
            .and_then(Option::as_ref)
            .and_then(|s| rules::upgrade_of(&s.item))
    })
}

/// A quarry placed from its item is its placer's.
pub(in crate::game) fn placed(game: &mut Game<'_>, stack: &ItemStack, pos: [i32; 3]) {
    if &*stack.item != rules::BLOCK {
        return;
    }
    let Some(who) = game.native_acting_player() else {
        return;
    };
    let quarry = Quarry::new(who.uuid, dig_direction(game, pos), pos);
    game.native_with_mod_block_entity(pos, |entity| store(entity, &quarry));
}

/// Whether the acting player is kept from this quarry, and told so.
fn refused(game: &mut Game<'_>, pos: [i32; 3]) -> bool {
    if game.native_block(pos) != rules::BLOCK {
        return false;
    }
    let Some(who) = game.native_acting_player() else {
        return false;
    };
    let owner = match game.world.block_entities.get(&(pos[0], pos[1], pos[2])) {
        Some(BlockEntity::Mod(m)) => load(m).map_or(0, |q| q.owner),
        _ => 0,
    };
    if owner == 0 || owner == who.uuid || who.permission >= 2 {
        return false;
    }
    game.native_tell(who.uuid, Text::translate("deepbore_quarry.not_owner", &[]));
    true
}

pub(in crate::game) fn right_click(game: &mut Game<'_>, pos: [i32; 3], canceled: &mut bool) {
    if !*canceled && refused(game, pos) {
        *canceled = true;
    }
}

pub(in crate::game) fn destroyed(game: &mut Game<'_>, pos: [i32; 3], canceled: &mut bool) {
    if !*canceled && refused(game, pos) {
        *canceled = true;
    }
}

/// Any way a quarry goes: its chunks are let go.
pub(in crate::game) fn block_changed(
    level: &GameLevel<'_>,
    dimension: LevelDimension,
    pos: BlockPos,
    old: BlockStateId,
    new: BlockStateId,
) {
    if &*level.facts.get(old).name != rules::BLOCK || &*level.facts.get(new).name == rules::BLOCK {
        return;
    }
    let dimension = match dimension {
        LevelDimension::Overworld => "minecraft:overworld",
        LevelDimension::Nether => "minecraft:the_nether",
        LevelDimension::End => "minecraft:the_end",
    };
    let mut state = WORLD.lock().unwrap_or_else(|e| e.into_inner());
    let Some(st) = state.as_mut() else {
        return;
    };
    let key = (dimension.to_owned(), [pos.x, pos.y, pos.z]);
    if st.loads.holds(&key) {
        let change = st.loads.release(&key);
        forceload(&mut st.commands, dimension, change);
        st.loads_changed = true;
    }
}

// ---- the tick --------------------------------------------------------------

/// The sides a chest may be on, and the chest's side that faces the quarry.
const AROUND: [([i32; 3], Direction); 6] = [
    ([0, -1, 0], Direction::Up),
    ([0, 1, 0], Direction::Down),
    ([0, 0, -1], Direction::South),
    ([0, 0, 1], Direction::North),
    ([-1, 0, 0], Direction::East),
    ([1, 0, 0], Direction::West),
];

fn find_chest(game: &Game<'_>, dimension: &str, pos: [i32; 3]) -> Option<([i32; 3], Direction)> {
    AROUND.iter().find_map(|(offset, face)| {
        let at = [pos[0] + offset[0], pos[1] + offset[1], pos[2] + offset[2]];
        (rules::is_storage(&game.native_block(at))
            && game
                .native_container_slots(dimension, at, *face)
                .is_some_and(|slots| !slots.is_empty()))
        .then_some((at, *face))
    })
}

/// Sends what waits in the hidden slots to the chest, as far as it has room.
fn flush(game: &mut Game<'_>, entity: &mut ModBlockEntity, chest: ([i32; 3], Direction)) {
    for slot in rules::FIRST_BUFFER..rules::SLOTS {
        if let Some(stack) = entity.items.get(slot).cloned().flatten() {
            let rest = game.native_insert_item(chest.0, chest.1, stack.clone());
            if rest.as_ref() != Some(&stack) {
                entity.items[slot] = rest;
            }
        }
    }
}

fn keep(entity: &mut ModBlockEntity, stack: ItemStack) {
    let free = (rules::FIRST_BUFFER..rules::SLOTS).find(|s| entity.items[*s].is_none());
    match free {
        Some(slot) => entity.items[slot] = Some(stack),
        // One block never drops more kinds than there are slots.
        None => log::warn!("[deepbore_quarry] no room for {}", stack.item),
    }
}

// --- internals ---
// The game's own tables, which no hook point offers: they may change with a
// new Horizon revision, so they stay here.

fn burn_time(game: &Game<'_>, stack: &ItemStack) -> i32 {
    game.res
        .providers()
        .burn_time(world_items!(game), stack, false)
}

/// What a block drops under an unenchanted diamond pickaxe, which harvests
/// everything breakable a pickaxe can, obsidian too.
fn drops(game: &mut Game<'_>, state: BlockStateId) -> Vec<ItemStack> {
    let tool = ItemStack {
        item: "minecraft:diamond_pickaxe".into(),
        count: 1,
        damage: 0,
        extra: None,
    };
    world_items!(game).block_drops(
        &game.pack.assets.registry,
        state,
        Some(&tool),
        &mut game.world.random,
    )
}

fn classify(game: &Game<'_>, at: [i32; 3]) -> Cell {
    if !game.world.loaded(at[0], at[2]) {
        return Cell::Unloaded;
    }
    let facts = game.res.facts().get(game.world.block(at[0], at[1], at[2]));
    let name = &*facts.name;
    if matches!(
        name,
        "minecraft:air" | "minecraft:cave_air" | "minecraft:void_air"
    ) {
        return Cell::Air;
    }
    if !matches!(facts.fluid, Fluid::None) && !facts.waterlogged {
        return Cell::Fluid;
    }
    if facts.hardness < 0.0 {
        return Cell::Unbreakable;
    }
    let contents = game
        .world
        .block_entities
        .get(&(at[0], at[1], at[2]))
        .is_some();
    if name == rules::BLOCK || (contents && name != "minecraft:spawner") {
        return Cell::Kept;
    }
    Cell::Solid
}
// --- end of internals ---

fn dig(
    game: &mut Game<'_>,
    entity: &mut ModBlockEntity,
    dimension: &str,
    at: [i32; 3],
    fluid: bool,
    chest: Option<([i32; 3], Direction)>,
) {
    let state = game.world.block(at[0], at[1], at[2]);
    let center = [
        f64::from(at[0]) + 0.5,
        f64::from(at[1]) + 0.5,
        f64::from(at[2]) + 0.5,
    ];
    // A fluid is plugged with cobblestone, so that it can't run into the pit.
    let (place, loot) = if fluid {
        ("minecraft:cobblestone", Vec::new())
    } else {
        ("minecraft:air", drops(game, state))
    };
    let Some(new) = game.pack.assets.registry.default_state(place) else {
        return;
    };
    game.native_set_block(dimension, at, new, 3);
    for stack in loot {
        let rest = match chest {
            Some((pos, face)) => game.native_insert_item(pos, face, stack),
            None => Some(stack),
        };
        if let Some(rest) = rest {
            keep(entity, rest);
        }
    }
    game.with_level(|_, level, _| {
        level.native_send_particles(
            &ParticleOptions::simple(ParticleType::Block),
            Some(state),
            center,
            [0.25; 3],
            0.1,
            8,
        );
        level.native_play_sound(
            "minecraft:block.stone.break",
            center,
            BLOCK_SOUNDS,
            0.6,
            1.0,
        );
    });
}

/// The "blocks" sound source (`SOUND_SOURCES`).
const BLOCK_SOUNDS: usize = 4;

/// What can't burn leaves the fuel slot for the chest: lava's empty bucket, and
/// whatever a hopper or a click put there, which would block the fuel behind it.
fn clear_fuel_slot(game: &Game<'_>, entity: &mut ModBlockEntity) {
    let useless = entity.items[rules::FUEL_SLOT]
        .as_ref()
        .is_some_and(|s| burn_time(game, s) == 0);
    let room = entity.items[rules::FIRST_BUFFER..]
        .iter()
        .any(Option::is_none);
    if useless && room {
        if let Some(stack) = entity.items[rules::FUEL_SLOT].take() {
            keep(entity, stack);
        }
    }
}

fn take_fuel(entity: &mut ModBlockEntity) {
    let Some(stack) = entity.items[rules::FUEL_SLOT].as_mut() else {
        return;
    };
    let lava = &*stack.item == "minecraft:lava_bucket";
    stack.count -= 1;
    if stack.count == 0 {
        entity.items[rules::FUEL_SLOT] = lava.then(|| ItemStack {
            item: "minecraft:bucket".into(),
            count: 1,
            damage: 0,
            extra: None,
        });
    }
}

/// Holds or lets go of the quarry's chunks; true when another quarry of its
/// owner holds them instead.
fn hold_chunks(
    game: &Game<'_>,
    quarry: &Quarry,
    upgrades: Upgrades,
    dimension: &str,
    pos: [i32; 3],
    chest: Option<([i32; 3], Direction)>,
) -> bool {
    let Some(mut guard) = world(game) else {
        return false;
    };
    let st = guard.as_mut().expect("opened");
    let key = (dimension.to_owned(), pos);
    if !(quarry.on && upgrades.chunk && quarry.cursor.is_some()) {
        if st.loads.holds(&key) {
            let change = st.loads.release(&key);
            forceload(&mut st.commands, dimension, change);
            st.loads_changed = true;
        }
        return false;
    }
    let chunks = quarry.area(pos).chunks(pos, chest.map(|c| c.0));
    match st.loads.hold(quarry.owner, &key, chunks) {
        Some(change) => {
            if !change.add.is_empty() || !change.remove.is_empty() {
                forceload(&mut st.commands, dimension, change);
                st.loads_changed = true;
            }
            false
        }
        None => true,
    }
}

pub(in crate::game) fn tick(game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity) {
    if entity.id != rules::BLOCK || !game.native_on(rules::ID) {
        return;
    }
    if entity.items.len() < rules::SLOTS {
        entity.items.resize(rules::SLOTS, None);
    }
    let was = load(entity);
    // Placed some other way than from its item: nobody's.
    let mut quarry = was
        .clone()
        .unwrap_or_else(|| Quarry::new(0, dig_direction(game, pos), pos));
    let upgrades = Upgrades::from_slots(&upgrade_slots(entity));
    quarry.resize(pos, rules::side(upgrades.area));

    let dimension = game.native_dimension();
    let chest = find_chest(game, dimension, pos);
    if let Some(chest) = chest {
        clear_fuel_slot(game, entity);
        flush(game, entity, chest);
    }
    let pending = entity.items[rules::FIRST_BUFFER..]
        .iter()
        .any(Option::is_some);
    let powered = game.native_neighbor_signal(dimension, pos) > 0;
    let (min_y, _) = game.native_world_height();
    let max_side = world(game).map_or(rules::side(3), |g| {
        g.as_ref().expect("opened").config.max_side
    });

    let mut action = Action::Wait;
    let mut broke = false;
    for _ in 0..rules::SKIPS_PER_TICK {
        let cell = quarry
            .target(pos)
            .map_or(Cell::Air, |at| classify(game, at));
        let fuel = entity.items[rules::FUEL_SLOT]
            .as_ref()
            .map(|s| burn_time(game, s))
            .filter(|t| *t > 0);
        let input = Input {
            upgrades,
            powered,
            max_side,
            min_y,
            chest: chest.is_some(),
            pending,
            fuel,
            cell,
        };
        action = rules::step(&mut quarry, pos, &input);
        match action {
            Action::Skip => {}
            Action::Burn => {
                quarry.burn += fuel.unwrap_or(0);
                take_fuel(entity);
            }
            Action::Break { pos: at, fluid } => {
                dig(game, entity, dimension, at, fluid, chest);
                broke = true;
                break;
            }
            Action::Stopped(_) | Action::Wait => break,
        }
    }

    let refused = hold_chunks(game, &quarry, upgrades, dimension, pos, chest);
    let status = rules::status_code(&action);
    let in_slot = entity.items[rules::FUEL_SLOT]
        .as_ref()
        .map_or(0, |s| i64::from(burn_time(game, s)) * s.count as i64);
    let data = vec![
        status,
        ((i64::from(quarry.burn) + in_slot) / i64::from(rules::BURN_PER_BLOCK)).min(32767) as i32,
        quarry.cursor.map_or(i32::from(i16::MIN), |c| c.y),
        quarry.side,
        i32::from(quarry.on),
        i32::from(refused),
    ];
    // What clients draw changes with the target and with working or not.
    let working = |d: &[i32]| d.first() == Some(&0);
    if broke
        || working(&data) != working(&entity.data)
        || was.as_ref().is_none_or(|w| w.side != quarry.side)
    {
        game.native_block_entity_sync(pos);
    }
    if entity.data != data {
        entity.data = data;
    }
    if was.as_ref() != Some(&quarry) {
        store(entity, &quarry);
    }
}

pub(in crate::game) fn menu_button(entity: &mut ModBlockEntity, id: i32) {
    if entity.id == rules::BLOCK && id == TOGGLE {
        if let Some(mut quarry) = load(entity) {
            quarry.on = !quarry.on;
            store(entity, &quarry);
        }
    }
}

/// A shift-click from the inventory: upgrades to a slot that takes them, fuel to
/// its slot, nothing else. One from the quarry goes the game's way, to the inventory.
pub(in crate::game) fn quick_move(
    game: &Game<'_>,
    entity: &mut ModBlockEntity,
    container_slots: usize,
    slot: usize,
    stack: &mut ItemStack,
    handled: &mut bool,
) {
    if entity.id != rules::BLOCK || slot < container_slots {
        return;
    }
    *handled = true;
    if let Some(kind) = rules::upgrade_of(&stack.item) {
        let slots = upgrade_slots(entity);
        let free = (0..rules::UPGRADE_SLOTS).find(|i| {
            entity.items[rules::FIRST_UPGRADE + i].is_none() && rules::accepts(&slots, *i, kind)
        });
        if let Some(i) = free {
            let mut one = stack.clone();
            one.count = 1;
            entity.items[rules::FIRST_UPGRADE + i] = Some(one);
            stack.count -= 1;
        }
    } else if burn_time(game, stack) > 0 {
        let most = game.native_max_stack_size(&stack.item).unwrap_or(64);
        match entity.items[rules::FUEL_SLOT].as_mut() {
            None => {
                entity.items[rules::FUEL_SLOT] = Some(stack.clone());
                stack.count = 0;
            }
            Some(have) if have.item == stack.item => {
                let room = (i64::from(most) - have.count as i64).clamp(0, stack.count as i64);
                have.count = (have.count as i64 + room).try_into().unwrap_or(have.count);
                stack.count = (stack.count as i64 - room)
                    .try_into()
                    .unwrap_or(stack.count);
            }
            Some(_) => {}
        }
    }
}

// ---- the screen ------------------------------------------------------------

const TOGGLE: i32 = 0;
/// `[x, y, width, height]`, from the screen's top left.
const BUTTON: [f32; 4] = [136.0, 34.0, 32.0, 18.0];

pub(in crate::game) fn screen(block: &str, screen: &mut Option<NativeScreen>) {
    if block == rules::BLOCK {
        // tools/make_textures.py draws the frames in the same places; the slots
        // past this list are hidden, and the server's menu leaves them out.
        *screen = Some(NativeScreen {
            width: 176.0,
            height: 166.0,
            slots: vec![
                Some([17.0, 35.0]),
                Some([62.0, 35.0]),
                Some([80.0, 35.0]),
                Some([98.0, 35.0]),
                Some([116.0, 35.0]),
            ],
            inventory_y: 84.0,
            inventory_x: 8.0,
            own_labels: true,
            own_background: true,
        });
    }
}

/// A language file's line with its `%s` filled in.
fn filled(key: &str, args: &[String]) -> String {
    let mut line = lang::get(key);
    for arg in args {
        line = line.replacen("%s", arg, 1);
    }
    line
}

pub(in crate::game) fn screen_draw(
    game: &Game<'_>,
    block: &str,
    data: &[i32],
    batch: &mut HudBatch,
    origin: [f32; 2],
) {
    if block != rules::BLOCK {
        return;
    }
    let [x, y] = origin;
    game.native_sprite(
        batch,
        "deepbore_quarry:gui/quarry",
        [0.0, 0.0, 176.0, 166.0],
        [x, y, 176.0, 166.0],
    );
    const LABEL: u32 = 0x40_4040;
    let get = |i: usize| data.get(i).copied().unwrap_or(0);
    let text = |batch: &mut HudBatch, line: &str, at: [f32; 2], rgb: u32| {
        game.native_text(batch, line, [x + at[0], y + at[1]], rgb, false);
    };
    text(
        batch,
        &lang::get("deepbore_quarry.screen.title"),
        [8.0, 6.0],
        LABEL,
    );
    let status = lang::get(&format!("deepbore_quarry.status.{}", get(0)));
    let color = if get(0) == 0 { 0x20_7020 } else { 0xA0_3020 };
    text(batch, &status, [8.0, 20.0], color);
    let side = get(3).to_string();
    let lines = [
        (
            filled("deepbore_quarry.screen.fuel", &[get(1).to_string()]),
            [8.0, 55.0],
        ),
        (
            filled("deepbore_quarry.screen.size", &[side.clone(), side]),
            [100.0, 55.0],
        ),
    ];
    for (line, at) in &lines {
        text(batch, line, *at, LABEL);
    }
    let layer = get(2) as i16;
    if layer != i16::MIN {
        text(
            batch,
            &filled("deepbore_quarry.screen.depth", &[layer.to_string()]),
            [8.0, 65.0],
            LABEL,
        );
    }
    if get(5) != 0 {
        text(
            batch,
            &lang::get("deepbore_quarry.chunk.taken"),
            [8.0, 74.0],
            0xA0_3020,
        );
    }
    let [bx, by, bw, bh] = BUTTON;
    game.native_fill(batch, [x + bx, y + by, bw, bh], 0xFF55_5555);
    game.native_fill(
        batch,
        [x + bx + 1.0, y + by + 1.0, bw - 2.0, bh - 2.0],
        0xFF8B_8B8B,
    );
    let label = lang::get(if get(4) != 0 {
        "deepbore_quarry.screen.stop"
    } else {
        "deepbore_quarry.screen.start"
    });
    let width = game.native_text_width(&label);
    text(
        batch,
        &label,
        [bx + (bw - width) / 2.0, by + 5.0],
        0xFF_FFFF,
    );
}

pub(in crate::game) fn screen_click(
    block: &str,
    at: [f32; 2],
    right: bool,
    button: &mut Option<i32>,
) {
    let [bx, by, bw, bh] = BUTTON;
    if block == rules::BLOCK
        && !right
        && (bx..bx + bw).contains(&at[0])
        && (by..by + bh).contains(&at[1])
    {
        *button = Some(TOGGLE);
    }
}

// ---- the drawing -----------------------------------------------------------

/// What clients get of a quarry: the square, whether it works, its pace and its target.
pub(in crate::game) fn update_tag(
    pos: [i32; 3],
    entity: &ModBlockEntity,
    tag: &mut Option<Compound>,
) {
    if entity.id != rules::BLOCK {
        return;
    }
    let Some(quarry) = load(entity) else {
        return;
    };
    let area = quarry.area(pos);
    let upgrades = Upgrades::from_slots(&upgrade_slots(entity));
    let target = quarry.target(pos);
    let [tx, ty, tz] = target.unwrap_or([pos[0], pos[1], pos[2]]);
    let view = vec![
        area.min[0],
        area.min[1],
        area.max[0],
        area.max[1],
        i32::from(entity.data.first() == Some(&0)),
        rules::ticks_per_block(upgrades.speed) as i32,
        i32::from(target.is_some()),
        tx,
        ty,
        tz,
    ];
    let mut compound = Compound::default();
    compound.insert(JavaString::from("view"), Tag::IntArray(view));
    *tag = Some(compound);
}

pub(in crate::game) fn bounds(entity: &ModBlockEntity, bounds: &mut [f32; 6]) {
    if entity.id == rules::BLOCK {
        *bounds = [
            f32::NEG_INFINITY,
            f32::NEG_INFINITY,
            f32::NEG_INFINITY,
            f32::INFINITY,
            f32::INFINITY,
            f32::INFINITY,
        ];
    }
}

/// How far above the quarry the gantry runs.
const GANTRY: f32 = 3.0;
const FRAME: &str = "deepbore_quarry:block/frame";
const DRILL: &str = "deepbore_quarry:block/drill";

struct Head {
    at: [f32; 3],
    /// The frame it was last drawn: game time and partial tick.
    time: (i64, f32),
}

/// Where each quarry's head is, on this client, so it moves between two blocks.
static HEADS: Mutex<Option<HashMap<[i32; 3], Head>>> = Mutex::new(None);

fn part(
    game: &Game<'_>,
    draw: &mut BlockEntityDraw,
    sprite: &str,
    at: [f32; 3],
    from: [f32; 3],
    to: [f32; 3],
) {
    let face = Some(CubeFace { sprite, uv: None });
    draw.push();
    draw.translate(at);
    game.native_draw_cube(draw, from, to, [face; 6], 0xFF_FFFF);
    draw.pop();
}

pub(in crate::game) fn submit(
    game: &Game<'_>,
    pos: [i32; 3],
    entity: &ModBlockEntity,
    draw: &mut BlockEntityDraw,
) {
    if entity.id != rules::BLOCK {
        return;
    }
    let Some(Tag::IntArray(view)) = entity.nbt.get(&JavaString::from("view")) else {
        return;
    };
    let &[
        min_x,
        min_z,
        max_x,
        max_z,
        working,
        ticks,
        has_target,
        tx,
        ty,
        tz,
    ] = view.as_slice()
    else {
        return;
    };
    // The quarry's own block is dark inside: the light is the air's above it.
    let (block_light, sky_light) = game.native_light([pos[0], pos[1] + 1, pos[2]]);
    draw.light = u32::from(sky_light) * 16 | (u32::from(block_light) * 16) << 8;
    let (x0, z0) = ((min_x - pos[0]) as f32, (min_z - pos[2]) as f32);
    let (x1, z1) = ((max_x - pos[0]) as f32, (max_z - pos[2]) as f32);
    let along_x = ([0.0, 6.0, 6.0], [16.0, 10.0, 10.0]);
    let along_z = ([6.0, 6.0, 0.0], [10.0, 10.0, 16.0]);
    let upright = ([6.0, 0.0, 6.0], [10.0, 16.0, 10.0]);

    // The frame around the square, with a post at each corner.
    let mut x = x0;
    while x <= x1 {
        part(game, draw, FRAME, [x, GANTRY, z0], along_x.0, along_x.1);
        part(game, draw, FRAME, [x, GANTRY, z1], along_x.0, along_x.1);
        x += 1.0;
    }
    let mut z = z0;
    while z <= z1 {
        part(game, draw, FRAME, [x0, GANTRY, z], along_z.0, along_z.1);
        part(game, draw, FRAME, [x1, GANTRY, z], along_z.0, along_z.1);
        z += 1.0;
    }
    for corner in [[x0, z0], [x1, z0], [x0, z1], [x1, z1]] {
        let mut y = 0.0;
        while y < GANTRY {
            part(
                game,
                draw,
                FRAME,
                [corner[0], y, corner[1]],
                upright.0,
                upright.1,
            );
            y += 1.0;
        }
    }

    // The head: it moves on towards the block above the target, smoothly.
    let time = (draw.game_time, draw.partial);
    let centre = [(x0 + x1) / 2.0, GANTRY, (z0 + z1) / 2.0];
    let target = if has_target != 0 {
        [
            (tx - pos[0]) as f32,
            (ty - pos[1]) as f32 + 1.0,
            (tz - pos[2]) as f32,
        ]
    } else {
        centre
    };
    let head = {
        let mut heads = HEADS.lock().unwrap_or_else(|e| e.into_inner());
        let head = heads
            .get_or_insert_with(HashMap::new)
            .entry(pos)
            .or_insert(Head { at: centre, time });
        let elapsed = rules::ticks_between(head.time, time).min(20.0);
        head.time = time;
        if working != 0 || has_target == 0 {
            let d = [
                target[0] - head.at[0],
                target[1] - head.at[1],
                target[2] - head.at[2],
            ];
            let distance = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
            let pace = (3.0 / ticks.max(1) as f32).max(distance / 40.0);
            head.at = rules::approach(head.at, target, pace * elapsed);
        }
        head.at
    };

    // The bridge across the square, at the head's z.
    let mut x = x0;
    while x <= x1 {
        part(
            game,
            draw,
            FRAME,
            [x, GANTRY, head[2]],
            along_x.0,
            along_x.1,
        );
        x += 1.0;
    }
    // The carriage on the bridge, the shaft below it, the bit at its end.
    part(
        game,
        draw,
        FRAME,
        [head[0], GANTRY, head[2]],
        [3.0; 3],
        [13.0; 3],
    );
    let mut y = GANTRY;
    while y - 1.0 >= head[1] {
        y -= 1.0;
        part(
            game,
            draw,
            DRILL,
            [head[0], y, head[2]],
            upright.0,
            upright.1,
        );
    }
    if y > head[1] {
        let left = (y - head[1]) * 16.0;
        part(
            game,
            draw,
            DRILL,
            [head[0], head[1], head[2]],
            upright.0,
            [10.0, left, 10.0],
        );
    }
    part(
        game,
        draw,
        DRILL,
        [head[0], head[1], head[2]],
        [4.0, 0.0, 4.0],
        [12.0, 6.0, 12.0],
    );
}
