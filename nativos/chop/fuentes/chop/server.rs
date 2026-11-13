// Chop! (c) 2026 Mayonesu0. All rights reserved, except the grants in LICENSE: https://github.com/Mayonesu0/chop/blob/main/LICENSE

// Chop! — the game's side (game::chop_server).
//
// A player cuts a log: on the next tick of that level the tree is read
// (game::chop_core decides what it is and which way it falls), its loot is rolled
// with the player's tool, it leaves the level and comes back as block displays that
// turn about their hinge under gravity. Each tick every display gets its new pose in
// one entity-data packet; sounds and particles go straight to the players near it;
// what a block gives drops where it breaks. `/chop` toggles it per player and opens
// the settings, a menu of clickable lines in the chat.

use super::Game;
use super::chop_core::{self as core, Cell, Config, Key, Phase, Pos, Section, Unit};
use super::creatures::NativeSound;
use super::lan_particles::QueuedParticle;
use super::native_entities::NativeDisplay;
use super::native_lifecycle::{ServerTickPhase, WorldPhase};
use super::native_server::{NativeCommand, NativeServerPlayer};
use horizon_client::items::ItemStack;
use horizon_client::mobs::{Damage, DamageKind};
use horizon_client::particles::{ParticleOptions, ParticleType};
use horizon_client::text::{ClickEvent, Style, Text, named_color};
use horizon_protocol::entity_data::{Value, serializer};
use horizon_protocol::packets::play::{self, Clientbound, SetEntityData};
use horizon_world::nbt::{Compound, Tag};
use horizon_world::registry::BlockStateId;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

/// The mod's id, as Options > Mods knows it.
const MOD: &str = "chop";
/// Every display of the mod carries it.
const TAG: &str = "chop.d";
/// `Level.setBlock` flags: the client is told, nothing else happens.
const QUIET: i32 = 2 | 816;
/// The usual: neighbours and shapes update.
const NORMAL: i32 = 3;
/// A cut not looked at by then is dropped (the break was cancelled).
const PENDING_TICKS: u32 = 40;
/// A tree whose level hasn't ticked for this long is ended where it is.
const IDLE_TICKS: u64 = 100;
const SAVE_TICKS: u64 = 600;
/// While no tree falls, displays left by a crash are removed this often.
const SWEEP_TICKS: u64 = 600;
/// Sound source `block`.
const BLOCKS: i32 = 4;

/// Mobs that are on the players' side.
const ALLIES: [&str; 6] = [
    "minecraft:iron_golem",
    "minecraft:snow_golem",
    "minecraft:copper_golem",
    "minecraft:villager",
    "minecraft:wandering_trader",
    "minecraft:allay",
];

// ---------------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------------

/// Who gets a tree's loot, and whether into their inventory (else at their feet).
type Owner = Option<(u128, bool)>;

struct Pending {
    dim: String,
    pos: Pos,
    state: BlockStateId,
    player: u128,
    age: u32,
}

/// A tree on its way down.
struct Falling {
    id: u64,
    dim: String,
    /// Who gets the loot at their feet, with that setting.
    owner: Owner,
    fell: core::Fell,
    /// The display showing each part.
    ents: Vec<Option<u32>>,
    states: Vec<BlockStateId>,
    loot: Vec<Vec<ItemStack>>,
    /// Where its displays stand: the cut block's centre.
    anchor: [f64; 3],
    /// Where it touches the ground, and the block it touches.
    contacts: Vec<([f64; 3], BlockStateId)>,
    struck: HashSet<u32>,
    struck_players: HashSet<u128>,
    whoosh: bool,
    /// It started to turn (the creak).
    creaked: bool,
    settled: bool,
    leaf: Option<String>,
    /// The cut block's ghost, shaking before it pops.
    ghost: Option<u32>,
    cut_state: BlockStateId,
    /// The logs of other trees its trunk comes down on (a chain reaction).
    hits: Vec<Pos>,
    /// 0 for the tree a player cut, 1 for one it knocked down, and so on.
    generation: u32,
    /// How often it fell on after knocking a tree down.
    resumes: u32,
    /// What its loot is rolled with, and whether it has loot.
    tool: Option<ItemStack>,
    drops: bool,
    /// Sounds still to come: the tick (`fell.age`), the sound, where, volume, pitch.
    later: Vec<(u32, &'static str, [f64; 3], f64, f64)>,
    /// What to plant where it stood once it is gone, and whether the saplings came
    /// out of its loot.
    replant: Option<(&'static str, Vec<Pos>, bool)>,
    /// Who felled it (a tree knocked down counts for its feller), for the stats.
    credit: Option<(u128, String)>,
    /// Its leaves' tint (the biome's).
    tint: Option<u32>,
    /// Apples that come loose during the fall: the leaf, the apples, and the tick
    /// after the creak they go.
    apples: Vec<(usize, ItemStack, u32)>,
    /// The tick it started to turn.
    creak_age: Option<u32>,
    /// The farthest part and one halfway up the trunk (where sounds come from).
    tip: usize,
    middle: usize,
    leaves: Vec<usize>,
    last: u64,
}

/// A log a player is digging: it trembles until it breaks.
struct Tremble {
    dim: String,
    pos: Pos,
    /// The face dug, pointing out of the block.
    face: Pos,
    state: BlockStateId,
    player: u128,
    /// Ticks the dig should take.
    ticks: u32,
    age: u32,
    display: Option<u32>,
    /// A few of the tree's leaves, which shiver as it is dug.
    leaves: Vec<Pos>,
    checked: bool,
    /// The player let go.
    stopped: bool,
    /// Ticks since the log broke, waiting for its felling to take the display.
    broken: u32,
    /// A tree shaken by hand, not dug: it shivers a moment and nothing breaks.
    shaking: bool,
    /// Its leaves' tint (the biome's).
    tint: Option<u32>,
    /// Where the trunk will lie: the ground under each log once it has fallen.
    preview: Vec<[f64; 3]>,
}

/// A player's own settings; -1 follows the world's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Perso {
    /// Sound volume, percent.
    volume: i32,
    /// 0 to 3, as the world's setting.
    particles: i32,
    /// The fall's preview, 0 or 1.
    preview: i32,
}

impl Default for Perso {
    fn default() -> Self {
        Perso {
            volume: -1,
            particles: -1,
            preview: -1,
        }
    }
}

/// What a player has felled.
#[derive(Clone, Debug, Default)]
struct Stats {
    name: String,
    trees: u64,
    logs: u64,
    blocks: u64,
    biggest: u64,
    chain: u32,
}

/// A player shook a tree (a right click with an empty hand on its trunk).
struct ShakeReq {
    dim: String,
    pos: Pos,
    face: Pos,
    player: u128,
}

struct State {
    world: PathBuf,
    config: Config,
    perso: HashMap<u128, Perso>,
    stats: HashMap<u128, Stats>,
    /// Hunger owed by players for the trees they felled.
    fatigue: HashMap<u128, f32>,
    shakes: Vec<ShakeReq>,
    /// When each tree (its foot) and each player last shook.
    shaken: HashMap<(String, Pos), u64>,
    shook: HashMap<u128, u64>,
    placed: HashMap<String, HashSet<Pos>>,
    /// Players who turned Chop! off for themselves.
    off: HashSet<u128>,
    pending: Vec<Pending>,
    trees: Vec<Falling>,
    trembles: Vec<Tremble>,
    rng: core::Rng,
    session: u64,
    next_id: u64,
    tick: u64,
    swept: HashMap<String, u64>,
    dirty: bool,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);
/// Set while a hook works on the state, so a hook the work sets off waits its turn.
static BUSY: AtomicBool = AtomicBool::new(false);
/// Whether a log trembles somewhere (the players' ticks look only then).
static TREMBLING: AtomicBool = AtomicBool::new(false);
/// Whether a player owes hunger for a felling.
static TIRED: AtomicBool = AtomicBool::new(false);
/// The players' own settings, as the effects read them.
static PERSO: Mutex<Option<HashMap<u128, Perso>>> = Mutex::new(None);

fn perso_snapshot(st: &State) {
    *PERSO.lock().unwrap_or_else(|e| e.into_inner()) = Some(st.perso.clone());
}

fn perso_of(uuid: u128) -> Perso {
    PERSO
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .as_ref()
        .and_then(|m| m.get(&uuid).copied())
        .unwrap_or_default()
}

fn seed() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0xC40B, |d| d.as_nanos() as u64)
}

fn data_file(world: &Path) -> PathBuf {
    world.join("data").join("chop.txt")
}

fn load(world: PathBuf) -> State {
    let text = std::fs::read_to_string(data_file(&world)).unwrap_or_default();
    let mut config = Config::default();
    let mut off = HashSet::new();
    let mut placed: HashMap<String, HashSet<Pos>> = HashMap::new();
    let mut perso: HashMap<u128, Perso> = HashMap::new();
    let mut stats: HashMap<u128, Stats> = HashMap::new();
    let mut section = String::new();
    for line in text.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(head) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            section = head.trim().to_owned();
            continue;
        }
        if section == "reglages" {
            config.read_line(line);
        } else if section == "sans_chop" {
            if let Ok(u) = u128::from_str_radix(line, 16) {
                off.insert(u);
            }
        } else if let Some(u) = section
            .strip_prefix("perso ")
            .and_then(|h| u128::from_str_radix(h.trim(), 16).ok())
        {
            let p = perso.entry(u).or_default();
            if let Some((k, v)) = line.split_once('=')
                && let Ok(v) = v.trim().parse::<i32>()
            {
                match k.trim() {
                    "sons" => p.volume = v.clamp(-1, 200),
                    "particules" => p.particles = v.clamp(-1, 3),
                    "apercu" => p.preview = v.clamp(-1, 1),
                    _ => {}
                }
            }
        } else if let Some(u) = section
            .strip_prefix("stats ")
            .and_then(|h| u128::from_str_radix(h.trim(), 16).ok())
        {
            let s = stats.entry(u).or_default();
            if let Some((k, v)) = line.split_once('=') {
                let (k, v) = (k.trim(), v.trim());
                let n = v.parse::<u64>().unwrap_or(0);
                match k {
                    "nom" => s.name = v.to_owned(),
                    "arbres" => s.trees = n,
                    "buches" => s.logs = n,
                    "blocs" => s.blocks = n,
                    "record" => s.biggest = n,
                    "chaine" => s.chain = n as u32,
                    _ => {}
                }
            }
        } else if let Some(dim) = section.strip_prefix("posees ") {
            let n: Vec<i32> = line
                .split_whitespace()
                .filter_map(|w| w.parse().ok())
                .collect();
            if let [x, y, z] = n[..] {
                placed
                    .entry(dim.trim().to_owned())
                    .or_default()
                    .insert([x, y, z]);
            }
        }
    }
    let seed = seed();
    *PERSO.lock().unwrap_or_else(|e| e.into_inner()) = Some(perso.clone());
    State {
        world,
        config,
        perso,
        stats,
        fatigue: HashMap::new(),
        shakes: Vec::new(),
        shaken: HashMap::new(),
        shook: HashMap::new(),
        placed,
        off,
        pending: Vec::new(),
        trees: Vec::new(),
        trembles: Vec::new(),
        rng: core::Rng::new(seed),
        session: seed & 0xFFFF_FFFF,
        next_id: 1,
        tick: 0,
        swept: HashMap::new(),
        dirty: false,
    }
}

fn save(st: &mut State) {
    let mut out = String::from("# Chop! : les données de ce monde\n[reglages]\n");
    for line in st.config.to_lines() {
        out.push_str(&line);
        out.push('\n');
    }
    out.push_str("[sans_chop]\n");
    let mut off: Vec<&u128> = st.off.iter().collect();
    off.sort();
    for u in off {
        out.push_str(&format!("{u:032x}\n"));
    }
    let mut players: Vec<&u128> = st.perso.keys().collect();
    players.sort();
    for u in players {
        let p = st.perso[u];
        if p == Perso::default() {
            continue;
        }
        out.push_str(&format!(
            "[perso {u:032x}]\nsons = {}\nparticules = {}\napercu = {}\n",
            p.volume, p.particles, p.preview
        ));
    }
    let mut players: Vec<&u128> = st.stats.keys().collect();
    players.sort();
    for u in players {
        let s = &st.stats[u];
        out.push_str(&format!(
            "[stats {u:032x}]\nnom = {}\narbres = {}\nbuches = {}\nblocs = {}\nrecord = {}\nchaine = {}\n",
            s.name, s.trees, s.logs, s.blocks, s.biggest, s.chain
        ));
    }
    let mut dims: Vec<&String> = st.placed.keys().collect();
    dims.sort();
    for dim in dims {
        let mut list: Vec<&Pos> = st.placed[dim].iter().collect();
        if list.is_empty() {
            continue;
        }
        list.sort();
        out.push_str(&format!("[posees {dim}]\n"));
        for p in list {
            out.push_str(&format!("{} {} {}\n", p[0], p[1], p[2]));
        }
    }
    let file = data_file(&st.world);
    if let Some(dir) = file.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let tmp = file.with_extension("txt.tmp");
    if std::fs::write(&tmp, out).is_ok() && std::fs::rename(&tmp, &file).is_ok() {
        st.dirty = false;
    }
}

/// Runs `f` on the open world's state (loaded on first use), unless another hook is
/// already working on it.
fn with_state<R>(game: &mut Game<'_>, f: impl FnOnce(&mut Game<'_>, &mut State) -> R) -> Option<R> {
    let world = game.native_world_dir()?;
    if BUSY.swap(true, Ordering::Acquire) {
        return None;
    }
    let mut slot = STATE.lock().unwrap_or_else(|e| e.into_inner());
    let mut st = match slot.take() {
        Some(st) if st.world == world => st,
        Some(mut old) => {
            save(&mut old);
            load(world)
        }
        None => load(world),
    };
    drop(slot);
    let r = f(game, &mut st);
    *STATE.lock().unwrap_or_else(|e| e.into_inner()) = Some(st);
    BUSY.store(false, Ordering::Release);
    Some(r)
}

// ---------------------------------------------------------------------------------
// Reading the level in place
// ---------------------------------------------------------------------------------

fn state_at(game: &Game<'_>, p: Pos) -> Option<BlockStateId> {
    let w = &game.world;
    if p[0].abs() >= 30_000_000
        || p[2].abs() >= 30_000_000
        || !(w.shape.min_y..w.shape.max_y()).contains(&p[1])
        || !w.loaded(p[0], p[2])
    {
        return None;
    }
    Some(w.block(p[0], p[1], p[2]))
}

fn in_tag(game: &Game<'_>, id: BlockStateId, tag: &str) -> bool {
    game.pack
        .assets
        .registry
        .state(id)
        .is_some_and(|s| game.native_server_block_in_tag(s, tag).unwrap_or(false))
}

fn name_of(game: &Game<'_>, id: BlockStateId) -> String {
    game.pack
        .assets
        .registry
        .state(id)
        .map(|s| s.name.to_string())
        .unwrap_or_default()
}

fn prop(game: &Game<'_>, id: BlockStateId, key: &str) -> Option<String> {
    let s = game.pack.assets.registry.state(id)?;
    s.properties
        .iter()
        .find(|(k, _)| k.to_string() == key)
        .map(|(_, v)| v.to_string())
}

/// `oak` of `minecraft:stripped_oak_log`.
fn species(name: &str) -> &str {
    let n = name.trim_start_matches("minecraft:");
    let n = n.strip_prefix("stripped_").unwrap_or(n);
    ["_log", "_wood", "_stem", "_hyphae", "_leaves"]
        .iter()
        .find_map(|s| n.strip_suffix(s))
        .unwrap_or(n)
}

/// What hangs on trees, and how.
fn hanger(game: &Game<'_>, id: BlockStateId, name: &str) -> Option<core::Attach> {
    use core::Attach::*;
    Some(match name.trim_start_matches("minecraft:") {
        "vine" | "cocoa" | "bee_nest" | "glow_lichen" | "resin_clump" | "creaking_heart" => Side,
        "pale_hanging_moss" | "weeping_vines" | "weeping_vines_plant" => Below,
        "mangrove_propagule" if prop(game, id, "hanging").as_deref() == Some("true") => Below,
        "snow" => Above,
        _ => return None,
    })
}

/// The rules' view of the level in place.
struct Reader<'g, 'a> {
    game: &'g Game<'a>,
    cells: HashMap<BlockStateId, Cell>,
    woods: HashMap<String, u16>,
    /// Huge fungi and mushrooms are trees too.
    fungi: bool,
}

/// The blocks of a fungus's or a mushroom's crown.
fn is_cap(name: &str) -> bool {
    matches!(
        name.trim_start_matches("minecraft:"),
        "nether_wart_block"
            | "warped_wart_block"
            | "shroomlight"
            | "red_mushroom_block"
            | "brown_mushroom_block"
    )
}

/// A trunk block: a log, wood, a stem or hyphae, or a mushroom's stem.
fn is_trunk(game: &Game<'_>, id: BlockStateId) -> bool {
    in_tag(game, id, "minecraft:logs") || name_of(game, id) == "minecraft:mushroom_stem"
}

impl<'g, 'a> Reader<'g, 'a> {
    fn new(game: &'g Game<'a>, config: &Config) -> Self {
        Reader {
            game,
            cells: HashMap::new(),
            woods: HashMap::new(),
            fungi: config.on(Key::Fungi),
        }
    }
    fn wood(&mut self, name: &str) -> u16 {
        let n = self.woods.len() as u16;
        *self.woods.entry(species(name).to_owned()).or_insert(n + 1)
    }
    fn classify(&mut self, id: BlockStateId) -> Cell {
        if let Some(c) = self.cells.get(&id) {
            return *c;
        }
        let game = self.game;
        let name = name_of(game, id);
        let b = game.res.behaviours();
        let cell = if b.is_air(id) {
            Cell::Open
        } else if in_tag(game, id, "minecraft:logs")
            || (self.fungi && name == "minecraft:mushroom_stem")
        {
            Cell::Log {
                wood: self.wood(&name),
            }
        } else if self.fungi && is_cap(&name) {
            Cell::Cap
        } else if in_tag(game, id, "minecraft:leaves") {
            Cell::Leaves {
                distance: prop(game, id, "distance")
                    .and_then(|d| d.parse().ok())
                    .unwrap_or(7),
                persistent: prop(game, id, "persistent").as_deref() == Some("true"),
            }
        } else if let Some(a) = hanger(game, id, &name) {
            Cell::Hanger(a)
        } else if b.is_replaceable(id) {
            Cell::Open
        } else {
            match b.collision_shape(id) {
                Some(s) if s.is_empty() => Cell::Open,
                // carpets, rails, pressure plates: the tree lies on top of them
                Some(s) if s.max(horizon_core::geometry::Axis::Y) <= 0.2 => Cell::Open,
                _ => Cell::Solid,
            }
        };
        self.cells.insert(id, cell);
        cell
    }
}

impl core::World for Reader<'_, '_> {
    fn cell(&mut self, p: Pos) -> Option<Cell> {
        let w = &self.game.world;
        if p[0].abs() >= 30_000_000 || p[2].abs() >= 30_000_000 || !w.loaded(p[0], p[2]) {
            return None;
        }
        if !(w.shape.min_y..w.shape.max_y()).contains(&p[1]) {
            return Some(Cell::Open);
        }
        Some(self.classify(w.block(p[0], p[1], p[2])))
    }
}

// ---------------------------------------------------------------------------------
// The players
// ---------------------------------------------------------------------------------

struct Cutter {
    name: String,
    dim: String,
    yaw: f32,
    game_mode: i32,
    sneaking: bool,
    held: Option<ItemStack>,
}

fn cutter(game: &Game<'_>, uuid: u128) -> Option<Cutter> {
    let host = game.lan.as_ref()?;
    let g = host
        .hub
        .guests
        .iter()
        .find(|g| g.uuid == uuid && g.placed && !g.dead)?;
    Some(Cutter {
        name: g.name.clone(),
        dim: g.dimension.clone(),
        yaw: g.yaw,
        game_mode: g.game_mode,
        sneaking: g.sneaking,
        held: g
            .inventory
            .get(g.selected)
            .cloned()
            .flatten()
            .filter(|s| s.count > 0),
    })
}

/// The tool's durability, unless it has none or can't break.
fn max_damage(game: &Game<'_>, stack: &ItemStack) -> Option<u32> {
    let components = horizon_client::item_data::encode_components(stack);
    let get = |k: &str| components.as_ref().and_then(|c| c.get(k));
    if get("minecraft:unbreakable").is_some() {
        return None;
    }
    let max = match get("minecraft:max_damage") {
        Some(Tag::Int(v)) if *v > 0 => *v as u32,
        _ => game
            .res
            .items()
            .get(&stack.item)
            .map_or(0, |i| i.max_damage),
    };
    (max > 0).then_some(max)
}

/// Whether `uuid` owns this single player world (cheats or not).
fn owner(game: &Game<'_>, uuid: u128) -> bool {
    game.lan
        .as_ref()
        .and_then(|h| h.hub.singleplayer_owner)
        .is_some_and(|o| o == uuid)
}

// ---------------------------------------------------------------------------------
// Effects
// ---------------------------------------------------------------------------------

/// A sound of the game at `at`, to each player near enough, at their own volume
/// (or the world's).
fn sound(game: &mut Game<'_>, config: &Config, name: &str, at: [f64; 3], volume: f64, pitch: f64) {
    let world = f64::from(config.get(Key::Volume)) / 100.0;
    if volume <= 0.0 {
        return;
    }
    let dimension = game.dim.id();
    let pitch = pitch.clamp(0.5, 2.0) as f32;
    let Some(host) = &game.lan else {
        if world > 0.0 {
            game.creatures.lan_sounds.push(NativeSound {
                dimension,
                sound: name.to_owned(),
                at,
                source: BLOCKS,
                volume: (volume * world) as f32,
                pitch,
            });
        }
        return;
    };
    if name.starts_with("minecraft:")
        && host
            .hub
            .registry_id("minecraft:sound_event", name)
            .is_none()
    {
        return;
    }
    for g in host
        .hub
        .guests
        .iter()
        .filter(|g| g.placed && g.dimension == dimension)
    {
        let own = perso_of(g.uuid).volume;
        let scale = if own >= 0 {
            f64::from(own) / 100.0
        } else {
            world
        };
        let v = (volume * scale) as f32;
        if v <= 0.0 {
            continue;
        }
        let range = f64::from(v.max(1.0)) * 16.0;
        let d: f64 = (0..3).map(|k| (g.position[k] - at[k]).powi(2)).sum();
        if d > range * range {
            continue;
        }
        let packet = host
            .hub
            .sound_packet(name, at, v, pitch, BLOCKS, host.hub.next_sound_seed());
        g.send(&packet);
    }
}

/// `base` particles at the "normales" setting, to each player within 32 blocks,
/// as many as their own setting (or the world's) says. `only` picks the players.
#[allow(clippy::too_many_arguments)]
fn particles_to(
    game: &mut Game<'_>,
    config: &Config,
    options: ParticleOptions,
    state: Option<BlockStateId>,
    at: [f64; 3],
    spread: [f64; 3],
    speed: f64,
    base: f64,
    only: &dyn Fn(u128) -> bool,
) {
    const LEVELS: [f64; 4] = [0.0, 0.4, 1.0, 2.0];
    let world = config.get(Key::Particles).clamp(0, 3);
    let dimension = game.dim.id();
    let Some(host) = &game.lan else {
        let k = LEVELS[world as usize];
        if k > 0.0 {
            game.lan_send_particles(QueuedParticle {
                dimension,
                options,
                state,
                at,
                spread,
                speed,
                count: ((base * k).round() as i32).max(1),
                randomization: 0,
            });
        }
        return;
    };
    let Some(particle) = Game::hub_particle(&host.hub, &options, state) else {
        return;
    };
    for g in host
        .hub
        .guests
        .iter()
        .filter(|g| g.placed && g.dimension == dimension)
    {
        if !only(g.uuid) {
            continue;
        }
        // BlockPos.closerToCenterThan(at, 32): from the centre of the player's block
        let d: f64 = (0..3)
            .map(|k| (g.position[k].floor() + 0.5 - at[k]).powi(2))
            .sum();
        if d >= 32.0 * 32.0 {
            continue;
        }
        let own = perso_of(g.uuid).particles;
        let level = if own >= 0 { own.min(3) } else { world };
        let k = LEVELS[level as usize];
        if k == 0.0 {
            continue;
        }
        let packet = Clientbound::LevelParticles(play::LevelParticles {
            particle: particle.clone(),
            override_limiter: false,
            always_show: false,
            pos: at,
            dist: spread.map(|v| v as f32),
            max_speed: [speed as f32; 3],
            count: ((base * k).round() as i32).max(1),
            randomization: 0,
        });
        g.send(&packet);
    }
}

#[allow(clippy::too_many_arguments)]
fn particles(
    game: &mut Game<'_>,
    config: &Config,
    options: ParticleOptions,
    state: Option<BlockStateId>,
    at: [f64; 3],
    spread: [f64; 3],
    speed: f64,
    base: f64,
) {
    particles_to(
        game,
        config,
        options,
        state,
        at,
        spread,
        speed,
        base,
        &|_| true,
    );
}

fn block_bits(game: &mut Game<'_>, config: &Config, state: BlockStateId, at: [f64; 3], base: f64) {
    particles(
        game,
        config,
        ParticleOptions::simple(ParticleType::Block),
        Some(state),
        at,
        [0.25, 0.25, 0.25],
        0.1,
        base,
    );
}

fn break_sound(name: &str) -> &'static str {
    let s = species(name);
    let bare = name.trim_start_matches("minecraft:");
    if bare.ends_with("wart_block") {
        "minecraft:block.wart_block.break"
    } else if bare == "shroomlight" {
        "minecraft:block.shroomlight.break"
    } else if bare.starts_with("weeping_vines") {
        "minecraft:block.weeping_vines.break"
    } else if bare.ends_with("mushroom_block") || bare == "mushroom_stem" {
        "minecraft:block.wood.break"
    } else if name.ends_with("_leaves") {
        match s {
            "cherry" => "minecraft:block.cherry_leaves.break",
            "azalea" | "flowering_azalea" => "minecraft:block.azalea_leaves.break",
            _ => "minecraft:block.grass.break",
        }
    } else if s == "cherry" {
        "minecraft:block.cherry_wood.break"
    } else if s == "crimson" || s == "warped" {
        "minecraft:block.stem.break"
    } else if name.ends_with("snow") {
        "minecraft:block.snow.break"
    } else if name.contains("vine") || name.contains("moss") || name.contains("lichen") {
        "minecraft:block.vine.break"
    } else {
        "minecraft:block.wood.break"
    }
}

fn step_sound(leaf: &str) -> &'static str {
    let bare = leaf.trim_start_matches("minecraft:");
    if bare.ends_with("wart_block") {
        return "minecraft:block.wart_block.step";
    }
    if bare.ends_with("mushroom_block") {
        return "minecraft:block.wood.step";
    }
    match species(leaf) {
        "cherry" => "minecraft:block.cherry_leaves.step",
        "azalea" | "flowering_azalea" => "minecraft:block.azalea_leaves.step",
        _ => "minecraft:block.grass.step",
    }
}

// The game's own sounds, layered and tuned: wood that creaks, cracks and thuds,
// leaves that rustle and crash, the soft thud of wool, and bubbly pops.
const WOOL: &str = "minecraft:block.wool.fall";
const PLOP: &str = "minecraft:entity.chicken.egg";
const BUBBLE: &str = "minecraft:block.bubble_column.bubble_pop";
const CREAK: &str = "minecraft:block.wooden_door.open";
const CRACK: &str = "minecraft:block.wood.break";
const THUD: &str = "minecraft:block.wood.fall";

/// Whether a leaf block's colour is its biome's (oak, jungle, acacia, dark oak).
fn biome_tinted(leaf: &str) -> bool {
    matches!(species(leaf), "oak" | "jungle" | "acacia" | "dark_oak")
}

/// The foliage colour of the biome at `p`.
fn foliage_at(game: &Game<'_>, p: Pos) -> Option<u32> {
    let chunk = game.world.chunks.get(&(p[0] >> 4, p[2] >> 4))?;
    let biome = chunk.biome(p[0], p[1], p[2]);
    Some(game.res.biomes().get(biome).foliage)
}

/// The tint of the leaves of a tree at `p`: its biome's for the leaves that take
/// it, else none (their own colour).
fn tint_for(game: &Game<'_>, leaf: Option<&str>, p: Pos) -> Option<u32> {
    leaf.filter(|l| biome_tinted(l))
        .and_then(|_| foliage_at(game, p))
}

/// The falling-leaf particle of a leaf block, tinted as its foliage is (`tint`,
/// the biome's, for the leaves that take it); spores for a fungus's wart, none
/// for a mushroom's cap.
fn leaf_particle(leaf: &str, tint: Option<u32>) -> Option<ParticleOptions> {
    let bare = leaf.trim_start_matches("minecraft:");
    if bare == "nether_wart_block" {
        return Some(ParticleOptions::simple(ParticleType::CrimsonSpore));
    }
    if bare == "warped_wart_block" {
        return Some(ParticleOptions::simple(ParticleType::WarpedSpore));
    }
    if is_cap(bare) {
        return None;
    }
    let fixed = match species(leaf) {
        "cherry" => return Some(ParticleOptions::simple(ParticleType::CherryLeaves)),
        "pale_oak" => return Some(ParticleOptions::simple(ParticleType::PaleOakLeaves)),
        "spruce" => 0x619961,
        "birch" => 0x80A755,
        "jungle" => 0x30BB0B,
        "acacia" => 0xAEA42A,
        "mangrove" => 0x8DB127,
        "azalea" | "flowering_azalea" => 0x6E9A35,
        _ => 0x59AE30,
    };
    let rgb = tint.filter(|_| biome_tinted(leaf)).unwrap_or(fixed);
    let mut o = ParticleOptions::simple(ParticleType::TintedLeaves);
    o.color = [
        ((rgb >> 16) & 0xFF) as f32 / 255.0,
        ((rgb >> 8) & 0xFF) as f32 / 255.0,
        (rgb & 0xFF) as f32 / 255.0,
        1.0,
    ];
    Some(o)
}

// ---------------------------------------------------------------------------------
// Displays
// ---------------------------------------------------------------------------------

fn floats(v: &[f32]) -> Tag {
    Tag::List(v.iter().map(|x| Tag::Float(x.to_bits())).collect())
}

#[allow(clippy::too_many_arguments)]
fn summon_display(
    game: &mut Game<'_>,
    dim: &str,
    anchor: [f64; 3],
    state: BlockStateId,
    translation: [f32; 3],
    scale: f32,
    session: u64,
    tree: u64,
) -> Option<u32> {
    let s = game.pack.assets.registry.state(state)?.clone();
    let mut block = Compound::new();
    block.insert("Name".into(), Tag::String(s.name.as_str().into()));
    let mut props = Compound::new();
    for (k, v) in s.properties.iter() {
        props.insert(
            k.to_string().as_str().into(),
            Tag::String(v.to_string().as_str().into()),
        );
    }
    block.insert("Properties".into(), Tag::Compound(props));
    let mut t = Compound::new();
    t.insert("translation".into(), floats(&translation));
    t.insert("left_rotation".into(), floats(&[0.0, 0.0, 0.0, 1.0]));
    t.insert("scale".into(), floats(&[scale; 3]));
    t.insert("right_rotation".into(), floats(&[0.0, 0.0, 0.0, 1.0]));
    let mut c = Compound::new();
    c.insert("block_state".into(), Tag::Compound(block));
    c.insert("transformation".into(), Tag::Compound(t));
    c.insert("interpolation_duration".into(), Tag::Int(0));
    c.insert("start_interpolation".into(), Tag::Int(0));
    c.insert("teleport_duration".into(), Tag::Int(0));
    c.insert("view_range".into(), Tag::Float(1.5f32.to_bits()));
    let tags = [
        TAG.to_owned(),
        format!("chop.s{session}"),
        format!("chop.t{tree}"),
    ];
    c.insert(
        "Tags".into(),
        Tag::List(
            tags.iter()
                .map(|t| Tag::String(t.as_str().into()))
                .collect(),
        ),
    );
    game.native_summon(dim, "minecraft:block_display", anchor, c)
}

/// A display's next pose, eased over one tick, sent to the players who see it: the
/// interpolation restarts (start 0) toward the new translation and rotation.
fn pose(game: &Game<'_>, id: u32, translation: [f32; 3], rotation: [f32; 4]) {
    let Some(host) = &game.lan else {
        return;
    };
    let packet = Clientbound::SetEntityData(SetEntityData {
        id: id as i32,
        values: vec![
            (8, serializer::INT, Value::Int(0)),
            (9, serializer::INT, Value::Int(1)),
            (11, serializer::VECTOR3, Value::Floats(translation.to_vec())),
            (13, serializer::QUATERNION, Value::Floats(rotation.to_vec())),
        ],
    });
    host.hub.send_to_tracking(id as i32, &packet);
}

/// `pose`, also scaled (a block puffing up or the cut block's ghost).
fn pose_scaled(game: &Game<'_>, id: u32, translation: [f32; 3], rotation: [f32; 4], scale: f32) {
    let Some(host) = &game.lan else {
        return;
    };
    let packet = Clientbound::SetEntityData(SetEntityData {
        id: id as i32,
        values: vec![
            (8, serializer::INT, Value::Int(0)),
            (9, serializer::INT, Value::Int(1)),
            (11, serializer::VECTOR3, Value::Floats(translation.to_vec())),
            (12, serializer::VECTOR3, Value::Floats(vec![scale; 3])),
            (13, serializer::QUATERNION, Value::Floats(rotation.to_vec())),
        ],
    });
    host.hub.send_to_tracking(id as i32, &packet);
}

// ---------------------------------------------------------------------------------
// Hooks
// ---------------------------------------------------------------------------------

/// `native_destroy_block`: a player breaks a block. A log is looked at on the next
/// tick of its level, once the game broke it.
pub(super) fn destroy(game: &mut Game<'_>, pos: [i32; 3], state: BlockStateId) {
    if !game.native_on(MOD) || !is_trunk(game, state) {
        return;
    }
    let Some(player) = game.native_acting_player() else {
        return;
    };
    with_state(game, |_, st| {
        if let Some(set) = st.placed.get_mut(&player.dimension)
            && set.remove(&pos)
        {
            // a placed log goes: it is never a tree's
            st.dirty = true;
            return;
        }
        st.pending.push(Pending {
            dim: player.dimension,
            pos,
            state,
            player: player.uuid,
            age: 0,
        });
    });
}

/// `native_left_click_block`: a player starts digging a block. A log trembles
/// while it is dug, unless it breaks at the first blow.
pub(super) fn left_click(game: &mut Game<'_>, pos: [i32; 3], face: [i32; 3], state: BlockStateId) {
    if !game.native_on(MOD) || !is_trunk(game, state) {
        return;
    }
    let Some(player) = game.native_acting_player() else {
        return;
    };
    // the dig's progress each tick, with this player's tool and effects
    let progress = game.destroy_progress(pos);
    if !(progress > 0.0 && progress < 0.5) {
        return;
    }
    let ticks = (1.0 / progress).ceil() as u32;
    with_state(game, |_, st| {
        if !st.config.on(Key::Shake) || st.off.contains(&player.uuid) {
            return;
        }
        for t in st.trembles.iter_mut().filter(|t| t.player == player.uuid) {
            t.stopped = true;
        }
        st.trembles.push(Tremble {
            dim: player.dimension,
            pos,
            face,
            state,
            player: player.uuid,
            ticks,
            age: 0,
            display: None,
            leaves: Vec::new(),
            checked: false,
            stopped: false,
            broken: 0,
            shaking: false,
            tint: None,
            preview: Vec::new(),
        });
        TREMBLING.store(true, Ordering::Relaxed);
    });
}

/// `native_player_tick`: a player who stopped digging lets their log be.
pub(super) fn player_tick(game: &mut Game<'_>) {
    let digging_stopped = TREMBLING.load(Ordering::Relaxed) && !game.left;
    let tired = TIRED.load(Ordering::Relaxed);
    if !digging_stopped && !tired {
        return;
    }
    let Some(player) = game.native_acting_player() else {
        return;
    };
    let owed = with_state(game, |_, st| {
        if digging_stopped {
            // a dig that only just started may not show as one yet
            for t in st
                .trembles
                .iter_mut()
                .filter(|t| t.player == player.uuid && t.age >= 2 && !t.shaking)
            {
                t.stopped = true;
            }
        }
        let owed = st.fatigue.remove(&player.uuid);
        TIRED.store(!st.fatigue.is_empty(), Ordering::Relaxed);
        owed
    })
    .flatten();
    // the player's own state is in place: they get hungrier
    if let Some(exhaustion) = owed
        && let Some(p) = game.player.as_mut()
    {
        p.cause_food_exhaustion(exhaustion);
    }
}

/// `native_right_click_block`: a right click with an empty hand on a trunk
/// shakes the tree.
pub(super) fn right_click(game: &mut Game<'_>, pos: [i32; 3], face: [i32; 3]) {
    if !game.native_on(MOD) {
        return;
    }
    let empty = game
        .inventory
        .get(game.selected)
        .is_none_or(|s| s.as_ref().is_none_or(|s| s.count == 0));
    if !empty || !state_at(game, pos).is_some_and(|id| is_trunk(game, id)) {
        return;
    }
    let Some(player) = game.native_acting_player() else {
        return;
    };
    with_state(game, |_, st| {
        if st.config.on(Key::ShakeTree) && st.config.on(Key::Active) {
            st.shakes.push(ShakeReq {
                dim: player.dimension,
                pos,
                face,
                player: player.uuid,
            });
        }
    });
}

/// `native_place_block`: logs a player places are remembered and never felled.
pub(super) fn place(game: &mut Game<'_>, pos: [i32; 3], state: BlockStateId) {
    if !game.native_on(MOD) || !is_trunk(game, state) {
        return;
    }
    let Some(player) = game.native_acting_player() else {
        return;
    };
    with_state(game, |_, st| {
        if st.config.on(Key::Protect) && st.placed.entry(player.dimension).or_default().insert(pos)
        {
            st.dirty = true;
        }
    });
}

/// `native_server_tick`.
pub(super) fn server_tick(game: &mut Game<'_>, phase: ServerTickPhase) {
    let ServerTickPhase::ServerTickStart { world_running } = phase else {
        return;
    };
    with_state(game, |game, st| {
        st.tick += 1;
        for p in st.pending.iter_mut() {
            p.age += 1;
        }
        st.pending.retain(|p| p.age <= PENDING_TICKS);
        if world_running {
            // a level nobody is in any more stops ticking: its trees end where they are
            let mut i = 0;
            while i < st.trees.len() {
                if st.tick.saturating_sub(st.trees[i].last) > IDLE_TICKS {
                    let mut tree = st.trees.remove(i);
                    end_tree(game, &mut tree);
                } else {
                    i += 1;
                }
            }
        } else {
            // a frozen world holds them
            let now = st.tick;
            for t in st.trees.iter_mut() {
                t.last = now;
            }
        }
        if st.dirty && st.tick % SAVE_TICKS == 0 {
            save(st);
        }
    });
}

/// `native_level_tick_pre`: the level `dimension` is in place. Its trees move a
/// tick, then the cuts made in it are looked at.
pub(super) fn level_tick(game: &mut Game<'_>, dimension: &str) {
    let enabled = game.native_on(MOD);
    with_state(game, |game, st| {
        if !enabled {
            // switched off in Options > Mods: what falls here ends at once
            let (here, rest): (Vec<Falling>, Vec<Falling>) = std::mem::take(&mut st.trees)
                .into_iter()
                .partition(|t| t.dim == dimension);
            st.trees = rest;
            for mut t in here {
                end_tree(game, &mut t);
            }
            st.pending.retain(|p| p.dim != dimension);
            return;
        }
        let config = st.config.clone();
        let mut knocks = Vec::new();
        let mut i = 0;
        while i < st.trees.len() {
            if st.trees[i].dim != dimension {
                i += 1;
                continue;
            }
            st.trees[i].last = st.tick;
            let (done, knock) = tick_tree(game, &config, &mut st.rng, &mut st.trees[i]);
            knocks.extend(knock);
            if done {
                st.trees.remove(i);
            } else {
                i += 1;
            }
        }
        for k in knocks {
            chain(game, st, dimension, k);
        }
        let mut started = 0;
        let mut j = 0;
        while j < st.pending.len() && started < 3 {
            if st.pending[j].dim != dimension {
                j += 1;
                continue;
            }
            let p = st.pending.remove(j);
            match job(game, st, &p) {
                Job::Retry => {
                    st.pending.insert(j, p);
                    j += 1;
                }
                Job::Done => started += 1,
            }
        }
        tick_shakes(game, st, dimension);
        tick_trembles(game, st, dimension);
        // displays a crash left behind, while nothing falls
        let last = st.swept.get(dimension).copied();
        if st.trees.is_empty()
            && st.trembles.is_empty()
            && last.is_none_or(|t| st.tick.saturating_sub(t) >= SWEEP_TICKS)
        {
            st.swept.insert(dimension.to_owned(), st.tick);
            game.native_run_command(&format!(
                "execute in {dimension} run kill @e[type=minecraft:block_display,tag={TAG}]"
            ));
        }
        game.flush_native_sounds();
    });
}

/// `native_server_stopping`: what still falls drops at once.
pub(super) fn stopping(game: &mut Game<'_>) {
    with_state(game, |game, st| {
        for mut t in std::mem::take(&mut st.trees) {
            end_tree(game, &mut t);
        }
        st.pending.clear();
        save(st);
    });
}

/// `native_world_lifecycle`: a world opened (its data loads) or closed (saved).
pub(super) fn lifecycle(game: &mut Game<'_>, phase: WorldPhase) {
    match phase {
        WorldPhase::WorldOpenReady => {
            let Some(world) = game.native_world_dir() else {
                return;
            };
            let mut slot = STATE.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(mut old) = slot.take() {
                save(&mut old);
            }
            *slot = Some(load(world));
        }
        WorldPhase::WorldCloseFinal
        | WorldPhase::WorldCloseFailed
        | WorldPhase::WorldOpenFailed => {
            let mut slot = STATE.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(mut old) = slot.take() {
                save(&mut old);
            }
        }
    }
}

// ---------------------------------------------------------------------------------
// A cut
// ---------------------------------------------------------------------------------

enum Job {
    Done,
    /// The log is still there: the break hasn't happened yet.
    Retry,
}

fn action_bar(game: &mut Game<'_>, player: u128, text: &str) {
    game.native_action_bar(player, span(text, "gold"));
}

/// Where a felling starts from: a player's cut, or a trunk knocked down.
struct Start {
    dim: String,
    cut: Pos,
    cut_state: BlockStateId,
    /// Which way it should fall (degrees), and how far it may turn from there.
    yaw: f64,
    aim: bool,
    cone: i32,
    tool: Option<ItemStack>,
    drops: bool,
    /// Who gets the loot at their feet, with that setting.
    owner: Owner,
    /// The player who cut it (their uuid and name), for messages and the wear.
    cutter: Option<(u128, String)>,
    /// Who the felling counts for: the cutter, or who felled the tree that
    /// knocked this one down.
    credit: Option<(u128, String)>,
    wear: bool,
    /// A player's own cut out of creative: it tires them (with that setting).
    tired: bool,
    /// The display of the log that trembled as it was dug: it puffs up and pops.
    ghost: Option<(u32, Pos)>,
    generation: u32,
    /// The cut log is still in the level (a trunk knocked down): it breaks once
    /// the tree behind it is known.
    standing: bool,
}

fn job(game: &mut Game<'_>, st: &mut State, p: &Pending) -> Job {
    if state_at(game, p.pos).is_some_and(|id| is_trunk(game, id)) {
        return Job::Retry;
    }
    // the log that trembled as it was dug, if it did
    let ghost = st
        .trembles
        .iter()
        .position(|t| t.dim == p.dim && t.pos == p.pos)
        .map(|i| st.trembles.remove(i))
        .and_then(|t| t.display.map(|d| (d, t.face)));
    if !felled(game, st, p, ghost)
        && let Some((d, _)) = ghost
    {
        game.native_remove_entity(&p.dim, d);
    }
    Job::Done
}

/// Fells the tree of the log `p` broke, when its cutter may: whether it did.
fn felled(game: &mut Game<'_>, st: &mut State, p: &Pending, ghost: Option<(u32, Pos)>) -> bool {
    if st.off.contains(&p.player) {
        return false;
    }
    let Some(who) = cutter(game, p.player) else {
        return false;
    };
    if who.dim != p.dim {
        return false;
    }
    let axe = who.held.as_ref().is_some_and(|s| {
        game.native_server_item_in_tag(s, "minecraft:axes")
            .unwrap_or(false)
    });
    let config = &st.config;
    if !core::fells(config, who.game_mode, who.sneaking, axe) {
        return false;
    }
    let creative = who.game_mode == 1;
    let start = Start {
        dim: p.dim.clone(),
        cut: p.pos,
        cut_state: p.state,
        yaw: f64::from(who.yaw),
        aim: config.on(Key::Aim),
        cone: config.get(Key::Cone),
        tool: who.held.clone(),
        drops: !creative && game.ui.rules.bool("block_drops"),
        owner: match config.get(Key::Loot) {
            1 => Some((p.player, false)),
            2 => Some((p.player, true)),
            _ => None,
        },
        cutter: Some((p.player, who.name.clone())),
        credit: Some((p.player, who.name.clone())),
        wear: config.on(Key::Wear) && !creative,
        tired: !creative,
        ghost,
        generation: 0,
        standing: false,
    };
    fell_tree(game, st, start)
}

/// Reads the tree behind `s.cut`, takes it out of the level and starts its fall:
/// false when there is no tree to fell there.
fn fell_tree(game: &mut Game<'_>, st: &mut State, s: Start) -> bool {
    let config = st.config.clone();
    let cut_name = name_of(game, s.cut_state);
    let tree = {
        let placed = st.placed.get(&s.dim);
        let protect = config.on(Key::Protect);
        let is_placed = |q: Pos| protect && placed.is_some_and(|set| set.contains(&q));
        let mut reader = Reader::new(game, &config);
        let wood = reader.wood(&cut_name);
        let mut look = core::Look::new(&mut reader);
        core::scan(
            &mut look,
            &is_placed,
            s.cut,
            wood,
            &core::Limits::of(&config),
        )
    };
    let tree = match tree {
        Ok(t) => t,
        Err(core::Refusal::TooBig) => {
            if let Some((uuid, _)) = &s.cutter {
                action_bar(
                    game,
                    *uuid,
                    "🪓 Chop! Cet arbre est trop grand : il se coupe bûche par bûche.",
                );
            }
            return false;
        }
        Err(_) => return false,
    };

    // the axe pays for every log
    if s.wear
        && let Some((uuid, name)) = &s.cutter
        && let Some(stack) = &s.tool
        && let Some(max) = max_damage(game, stack)
    {
        let n = core::wear(
            tree.logs.len() as u32,
            stack.enchantment_level("minecraft:unbreaking"),
            &mut st.rng,
        );
        let after = stack.damage + n;
        if after >= max {
            action_bar(
                game,
                *uuid,
                &format!("🪓 Chop! Ta hache est trop usée pour cet arbre ({n} points de plus)."),
            );
            return false;
        }
        if n > 0 {
            game.native_run_command(&format!(
                "item modify entity {name} weapon.mainhand {{type:\"minecraft:set_components\",components:{{\"minecraft:damage\":{after}}}}}"
            ));
        }
    }

    let dim = s.dim.clone();
    let center = |c: Pos| [c[0] as f64 + 0.5, c[1] as f64 + 0.5, c[2] as f64 + 0.5];
    let cut = center(s.cut);
    // a trunk knocked down breaks at its foot
    if s.standing {
        let items = if s.drops {
            game.block_loot_at(s.cut, s.cut_state, s.tool.as_ref())
        } else {
            Vec::new()
        };
        clear(game, &dim, s.cut, s.cut_state, NORMAL);
        block_bits(game, &config, s.cut_state, cut, 10.0);
        for item in items {
            drop_item(game, &mut st.rng, cut, item, s.owner);
        }
    }

    // which way, how far, what it crushes, whose trunks it comes down on
    let animate = config.on(Key::Animation);
    let (body, contact, crush, hits) = {
        let mut reader = Reader::new(game, &config);
        let mut look = core::Look::new(&mut reader);
        let sink = if animate {
            core::gap(&mut look, &tree)
        } else {
            0.0
        };
        let dir = if animate {
            core::choose_dir(&tree, &mut look, s.yaw, s.aim, s.cone, sink)
        } else {
            core::yaw_dir(s.yaw)
        };
        let body = core::Body::new(&tree, dir, sink);
        if animate {
            let contact = body.contact(&mut look, true);
            let crush = body.crush(&mut look, contact.theta);
            let hits = log_hits(&mut look, &contact);
            (body, contact, crush, hits)
        } else {
            let none = core::Contact {
                theta: 0.0,
                ground: true,
                points: Vec::new(),
            };
            (body, none, Vec::new(), Vec::new())
        }
    };

    // its loot, rolled with the tool while the blocks are still there
    let mut states = Vec::with_capacity(body.parts.len());
    let mut loot = Vec::with_capacity(body.parts.len());
    for part in &body.parts {
        let id = state_at(game, part.cell).unwrap_or(s.cut_state);
        states.push(id);
        loot.push(if s.drops {
            game.block_loot_at(part.cell, id, s.tool.as_ref())
        } else {
            Vec::new()
        });
    }
    let contacts = dust_points(game, &contact);
    let leaf = body
        .parts
        .iter()
        .position(|part| part.leaf)
        .map(|i| name_of(game, states[i]));

    let tint = tint_for(game, leaf.as_deref(), s.cut);
    credit_felling(st, &config, &s, &tree);

    // what hangs on it: its loot too
    let mut hung = Vec::with_capacity(tree.hangers.len());
    for h in &tree.hangers {
        if let Some(id) = state_at(game, *h) {
            let items = if s.drops {
                game.block_loot_at(*h, id, s.tool.as_ref())
            } else {
                Vec::new()
            };
            hung.push((*h, id, items));
        }
    }
    // the saplings that go back where it stood
    let replanting = plan_replant(
        game,
        &config,
        &tree,
        &cut_name,
        leaf.as_deref(),
        s.drops,
        &mut loot,
        &mut hung,
    );

    // what hangs on it breaks off first
    let silk = s
        .tool
        .as_ref()
        .is_some_and(|t| t.enchantment_level("minecraft:silk_touch") > 0);
    for (n, (h, id, items)) in hung.into_iter().enumerate() {
        let at = center(h);
        let nest = matches!(
            name_of(game, id).as_str(),
            "minecraft:bee_nest" | "minecraft:beehive"
        );
        if nest
            && !silk
            && config.on(Key::Bees)
            && let Some((_, facing)) = game.hive_state(h)
        {
            // its bees come out, angry, as when a player breaks it
            game.empty_hive(h, facing, horizon_client::beehive::Release::Emergency);
            game.anger_nearby_bees(h);
        }
        clear(game, &dim, h, id, NORMAL);
        block_bits(game, &config, id, at, 4.0);
        if n < 3 {
            let name = name_of(game, id);
            sound(game, &config, break_sound(&name), at, 0.6, 1.2);
        }
        for item in items {
            drop_item(game, &mut st.rng, at, item, None);
        }
    }
    // then the tree leaves the level, leaves first
    for i in (0..body.parts.len()).rev() {
        let part = &body.parts[i];
        let flags = if part.surface { NORMAL } else { QUIET };
        clear(game, &dim, part.cell, states[i], flags);
    }

    if !animate {
        if let Some((ghost, _)) = s.ghost {
            game.native_remove_entity(&dim, ghost);
        }
        // everything breaks where it stood, with a little tune
        let every = (body.parts.len() / 150).max(1);
        for (i, part) in body.parts.iter().enumerate() {
            let at = center(part.cell);
            if i % every == 0 {
                block_bits(game, &config, states[i], at, 3.0);
                particles(
                    game,
                    &config,
                    ParticleOptions::simple(ParticleType::Poof),
                    None,
                    at,
                    [0.2, 0.2, 0.2],
                    0.02,
                    1.0,
                );
            }
            for item in std::mem::take(&mut loot[i]) {
                drop_item(game, &mut st.rng, at, item, s.owner);
            }
        }
        sound(game, &config, CRACK, cut, 0.9, 0.8);
        if let Some(l) = &leaf {
            sound(game, &config, break_sound(l), cut, 0.8, 0.9);
        }
        sound(game, &config, PLOP, cut, 0.9, 1.2);
        sound(game, &config, BUBBLE, cut, 0.8, 1.0);
        sound(game, &config, BUBBLE, cut, 0.6, 1.4);
        if let Some((sapling, spots, from_loot)) = replanting {
            replant(game, &config, &mut st.rng, &dim, sapling, &spots, from_loot);
        }
        return true;
    }

    // the falling tree, and the ghost of the cut block shaking under it
    let id = st.next_id;
    st.next_id += 1;
    let anchor = cut;
    let mut ents = Vec::with_capacity(body.parts.len());
    for (i, part) in body.parts.iter().enumerate() {
        // where the block stood: the body is planned dropped, it shows upright first
        let tr = [0, 1, 2].map(|k| (f64::from(part.cell[k]) - anchor[k]) as f32);
        ents.push(summon_display(
            game, &dim, anchor, states[i], tr, 1.0, st.session, id,
        ));
    }
    let ghost = s.ghost.map(|(ghost, face)| {
        // the log that trembled swells one last time; it pops on the next tick
        let (tr, scale) = core::tremble_pose(0, 1.0, face);
        let tr = tr.map(|v| v - 0.06);
        pose_scaled(game, ghost, tr, [0.0, 0.0, 0.0, 1.0], scale + 0.12);
        ghost
    });
    if ghost.is_none() {
        // cut in one blow, or pushed over: it goes at once
        sound(game, &config, "minecraft:block.wood.hit", cut, 0.6, 1.3);
    }
    let tip = (0..body.parts.len())
        .max_by(|a, b| body.parts[*a].r.total_cmp(&body.parts[*b].r))
        .unwrap_or(0);
    let logs: Vec<usize> = (0..body.parts.len())
        .filter(|i| !body.parts[*i].leaf)
        .collect();
    let middle = logs.get(logs.len() / 2).copied().unwrap_or(tip);
    let leaves: Vec<usize> = (0..body.parts.len())
        .filter(|i| body.parts[*i].leaf)
        .collect();
    let motion = core::Motion::new(&body, &contact, config.get(Key::Speed));
    let lie = (config.get(Key::Rest) * 2).max(0) as u32;
    // the apples come loose as it goes
    let mut apples = Vec::new();
    if config.on(Key::Apples) {
        for &i in &leaves {
            for stack in loot[i].iter_mut() {
                if &*stack.item == "minecraft:apple" && stack.count > 0 {
                    let when = 3 + st.rng.below(10);
                    apples.push((i, stack.clone(), when));
                    stack.count = 0;
                }
            }
        }
    }
    st.trees.push(Falling {
        id,
        dim,
        owner: s.owner,
        fell: core::Fell::new(body, motion, crush, lie, false, config.get(Key::Speed)),
        ents,
        states,
        loot,
        anchor,
        contacts,
        struck: HashSet::new(),
        struck_players: HashSet::new(),
        whoosh: false,
        creaked: false,
        settled: false,
        leaf,
        ghost,
        cut_state: s.cut_state,
        hits,
        generation: s.generation,
        resumes: 0,
        tool: s.tool,
        drops: s.drops,
        later: Vec::new(),
        replant: replanting,
        credit: s.credit,
        tint,
        apples,
        creak_age: None,
        tip,
        middle,
        leaves,
        last: st.tick,
    });
    true
}

/// A felling counts in its feller's stats; a player's own cut also tires them.
fn credit_felling(st: &mut State, config: &Config, s: &Start, tree: &core::Tree) {
    let Some((uuid, name)) = &s.credit else {
        return;
    };
    let stats = st.stats.entry(*uuid).or_default();
    stats.name = name.clone();
    stats.trees += 1;
    stats.logs += tree.logs.len() as u64 + 1;
    stats.blocks += tree.blocks() as u64 + 1;
    stats.biggest = stats.biggest.max(tree.blocks() as u64 + 1);
    stats.chain = stats.chain.max(s.generation);
    st.dirty = true;
    if s.tired && config.on(Key::Fatigue) {
        *st.fatigue.entry(*uuid).or_default() += 0.15 * tree.logs.len() as f32;
        TIRED.store(true, Ordering::Relaxed);
    }
}

/// The logs of other trees among the cells the trunk touches.
fn log_hits(look: &mut core::Look<'_>, contact: &core::Contact) -> Vec<Pos> {
    let mut out: Vec<Pos> = Vec::new();
    for (_, cell) in &contact.points {
        if matches!(look.cell(*cell), Some(Cell::Log { .. })) && !out.contains(cell) {
            out.push(*cell);
        }
    }
    out
}

/// Up to ten points where the trunk touches, with the block there (its dust).
fn dust_points(game: &Game<'_>, contact: &core::Contact) -> Vec<([f64; 3], BlockStateId)> {
    contact
        .points
        .iter()
        .step_by((contact.points.len() / 10).max(1))
        .filter_map(|(w, c)| state_at(game, *c).map(|s| (*w, s)))
        .collect()
}

/// Takes the block at `p` out of the level (water stays where it was waterlogged).
fn clear(game: &mut Game<'_>, dim: &str, p: Pos, id: BlockStateId, flags: i32) {
    let wet = prop(game, id, "waterlogged").as_deref() == Some("true");
    let registry = game.pack.assets.registry;
    let Some(left) = registry.default_state(if wet {
        "minecraft:water"
    } else {
        "minecraft:air"
    }) else {
        return;
    };
    game.native_set_block(dim, p, left, flags);
}

/// An item pops out at `at`; or, for an `owner` in this level, goes into their
/// inventory (what doesn't fit at their feet) or straight to their feet.
fn drop_item(
    game: &mut Game<'_>,
    rng: &mut core::Rng,
    at: [f64; 3],
    item: ItemStack,
    owner: Owner,
) {
    if item.count == 0 {
        return;
    }
    let mut item = item;
    if let Some((o, bag)) = owner
        && let Some(p) = game
            .native_server_players()
            .into_iter()
            .find(|p| p.uuid == o && p.dimension == game.dim.id())
    {
        if bag {
            match game.native_give_stack(o, item) {
                None => return,
                Some(left) => item = left,
            }
        }
        game.spawn_item(item, p.position, [0.0; 3], 0);
        return;
    }
    let at = free_spot(game, at);
    let v = [rng.spread(0.08), 0.12 + rng.unit() * 0.1, rng.spread(0.08)];
    game.spawn_item(item, at, v, 10);
}

/// `at`, or just above it when it is inside a solid block.
fn free_spot(game: &Game<'_>, at: [f64; 3]) -> [f64; 3] {
    let b = game.res.behaviours();
    let mut p = at;
    for _ in 0..3 {
        let cell = [
            p[0].floor() as i32,
            p[1].floor() as i32,
            p[2].floor() as i32,
        ];
        let solid = state_at(game, cell)
            .is_some_and(|id| b.collision_shape(id).is_none_or(|s| !s.is_empty()) && !b.is_air(id));
        if !solid {
            return p;
        }
        p[1] = p[1].floor() + 1.2;
    }
    p
}

// ---------------------------------------------------------------------------------
// Replanting
// ---------------------------------------------------------------------------------

/// Whether the block under `p` takes `sapling` (dirt, grass, podzol, moss, mud...;
/// clay too for a propagule).
fn ground_ok(game: &Game<'_>, p: Pos, sapling: &str) -> bool {
    let Some(below) = state_at(game, [p[0], p[1] - 1, p[2]]) else {
        return false;
    };
    let name = name_of(game, below);
    if sapling.ends_with("_mushroom") {
        // a mushroom keeps to mycelium, podzol and nylium (the tag's name in 26.3,
        // and its name before)
        return in_tag(game, below, "minecraft:overrides_mushroom_light_requirement")
            || in_tag(game, below, "minecraft:mushroom_grow_block");
    }
    let fungus = sapling.ends_with("_fungus")
        && (in_tag(game, below, "minecraft:nylium")
            || name == "minecraft:mycelium"
            || name == "minecraft:soul_soil");
    fungus
        || in_tag(game, below, "minecraft:dirt")
        || in_tag(game, below, "minecraft:supports_vegetation")
        || name == "minecraft:farmland"
        || (sapling.ends_with("propagule") && name == "minecraft:clay")
}

/// Whether `p` has room for `sapling`: air, or water for a propagule.
fn room_for(game: &Game<'_>, p: Pos, sapling: &str) -> Option<bool> {
    let here = state_at(game, p)?;
    if game.res.behaviours().is_air(here) {
        Some(false)
    } else if sapling.ends_with("propagule") && name_of(game, here) == "minecraft:water" {
        Some(true)
    } else {
        None
    }
}

/// What to replant, where, and whether the saplings come out of the loot: the
/// tree's own sapling, on its places that stand on soil. With "une pousse du
/// butin" they are taken out of what the tree gives (as many as it gives, one a
/// place); without loot (creative, no block drops) they are given.
#[allow(clippy::too_many_arguments)]
fn plan_replant(
    game: &Game<'_>,
    config: &Config,
    tree: &core::Tree,
    cut_name: &str,
    leaf: Option<&str>,
    drops: bool,
    loot: &mut [Vec<ItemStack>],
    hung: &mut [(Pos, BlockStateId, Vec<ItemStack>)],
) -> Option<(&'static str, Vec<Pos>, bool)> {
    let mode = config.get(Key::Replant);
    if mode == 0 {
        return None;
    }
    let sapling = core::sapling(species(cut_name), leaf)?;
    let mut spots: Vec<Pos> = tree
        .spots()
        .into_iter()
        .filter(|p| ground_ok(game, *p, sapling))
        .collect();
    // a fungus never drops its own: the wart's tree gives it back free
    let from_loot = mode == 1 && drops && !sapling.ends_with("_fungus");
    if from_loot {
        let mut have = 0;
        let stacks = loot
            .iter_mut()
            .flatten()
            .chain(hung.iter_mut().flat_map(|h| h.2.iter_mut()));
        for stack in stacks {
            if have == spots.len() {
                break;
            }
            if &*stack.item == sapling && stack.count > 0 {
                let take = (spots.len() - have).min(stack.count as usize);
                stack.count -= take as u32;
                have += take;
            }
        }
        spots.truncate(have);
    }
    log::debug!("Chop!: replant {sapling} at {spots:?} (from the loot: {from_loot})");
    (!spots.is_empty()).then_some((sapling, spots, from_loot))
}

/// Plants `sapling` at each of `spots` still free on soil, with a little pop and
/// green sparkles; one that came out of the loot and has no room drops there.
fn replant(
    game: &mut Game<'_>,
    config: &Config,
    rng: &mut core::Rng,
    dim: &str,
    sapling: &'static str,
    spots: &[Pos],
    from_loot: bool,
) {
    let registry = game.pack.assets.registry;
    let Some(plain) = registry.default_state(sapling) else {
        log::warn!("Chop!: no block {sapling} to replant");
        return;
    };
    for p in spots {
        let at = [0, 1, 2].map(|k| f64::from(p[k]) + 0.5);
        let wet = room_for(game, *p, sapling).filter(|_| ground_ok(game, *p, sapling));
        log::debug!(
            "Chop!: planting {sapling} at {p:?}: room {:?}, ground {}",
            room_for(game, *p, sapling),
            ground_ok(game, *p, sapling)
        );
        let Some(wet) = wet else {
            if from_loot {
                let item = ItemStack {
                    item: sapling.into(),
                    count: 1,
                    damage: 0,
                    extra: None,
                };
                drop_item(game, rng, at, item, None);
            }
            continue;
        };
        let state = if wet {
            with_prop(game, plain, "waterlogged", "true").unwrap_or(plain)
        } else {
            plain
        };
        game.native_set_block(dim, *p, state, NORMAL);
        sound(game, config, "minecraft:block.grass.place", at, 0.8, 1.2);
        sound(game, config, BUBBLE, at, 0.4, 1.6);
        particles(
            game,
            config,
            ParticleOptions::simple(ParticleType::HappyVillager),
            None,
            at,
            [0.3, 0.3, 0.3],
            0.0,
            6.0,
        );
    }
}

/// The state `id` with property `key` set to `value`.
fn with_prop(game: &Game<'_>, id: BlockStateId, key: &str, value: &str) -> Option<BlockStateId> {
    let registry = game.pack.assets.registry;
    let mut state = registry.state(id)?.clone();
    let slot = state.properties.get_mut(key)?;
    *slot = value.into();
    registry.id(&state)
}

// ---------------------------------------------------------------------------------
// A log being dug
// ---------------------------------------------------------------------------------

/// The logs being dug in the level `dim` tremble a tick: harder as the dig goes
/// on, with a little knock, a few chips and leaves shivering off the crown. One
/// that broke waits a few ticks for its felling to take it; one let go stops.
fn tick_trembles(game: &mut Game<'_>, st: &mut State, dim: &str) {
    let config = st.config.clone();
    let mut i = 0;
    while i < st.trembles.len() {
        if st.trembles[i].dim != dim {
            i += 1;
            continue;
        }
        st.trembles[i].age += 1;
        let t = &st.trembles[i];
        let standing = state_at(game, t.pos) == Some(t.state);
        let done = if !standing {
            st.trembles[i].broken += 1;
            st.trembles[i].broken > 3
        } else {
            let slack = if t.shaking { 0 } else { 20 };
            t.stopped || t.age > t.ticks + slack || !(config.on(Key::Shake) || t.shaking)
        };
        if done {
            let t = st.trembles.remove(i);
            if let Some(d) = t.display {
                game.native_remove_entity(&t.dim, d);
            }
            continue;
        }
        if !standing {
            i += 1;
            continue;
        }
        if !st.trembles[i].checked {
            match tremble_start(game, st, i) {
                Some(setup) => {
                    let t = &mut st.trembles[i];
                    t.checked = true;
                    t.display = Some(setup.display);
                    t.leaves = setup.leaves;
                    t.tint = setup.tint;
                    t.preview = setup.preview;
                }
                None => {
                    st.trembles.remove(i);
                    continue;
                }
            }
        }
        let t = &st.trembles[i];
        let strength = if t.shaking {
            0.6
        } else {
            f64::from(t.age) / f64::from(t.ticks.max(1))
        };
        // where it will fall, for the players who want to see it
        if t.age % 4 == 1 && !t.preview.is_empty() {
            let mut dust = ParticleOptions::simple(ParticleType::Dust);
            dust.color = [1.0, 0.85, 0.35, 1.0];
            dust.scale = 0.9;
            let points = t.preview.clone();
            let world = config.on(Key::Preview);
            let want = move |u: u128| match perso_of(u).preview {
                -1 => world,
                v => v == 1,
            };
            for at in points {
                particles_to(
                    game,
                    &config,
                    dust.clone(),
                    None,
                    at,
                    [0.15, 0.0, 0.15],
                    0.0,
                    1.0,
                    &want,
                );
            }
        }
        let t = &st.trembles[i];
        if let Some(d) = t.display {
            let (tr, scale) = core::tremble_pose(t.age, strength, t.face);
            pose_scaled(game, d, tr, [0.0, 0.0, 0.0, 1.0], scale);
        }
        let face_at = [0, 1, 2].map(|k| f64::from(t.pos[k]) + 0.5 + 0.55 * f64::from(t.face[k]));
        if t.age % 2 == 0 {
            let pitch = if t.age % 4 == 0 { 1.5 } else { 1.7 } + 0.2 * strength;
            sound(
                game,
                &config,
                "minecraft:block.wood.hit",
                face_at,
                0.3,
                pitch,
            );
            block_bits(game, &config, t.state, face_at, 2.0);
        }
        if t.age % 3 == 0 && !t.leaves.is_empty() {
            let leaf = t.leaves[st.rng.below(t.leaves.len() as u32) as usize];
            if let Some(id) = state_at(game, leaf) {
                let name = name_of(game, id);
                let at = [0, 1, 2].map(|k| f64::from(leaf[k]) + 0.5);
                if let Some(o) = leaf_particle(&name, t.tint) {
                    particles(
                        game,
                        &config,
                        o,
                        None,
                        [at[0], at[1] - 0.6, at[2]],
                        [0.4, 0.1, 0.4],
                        0.0,
                        if t.shaking { 3.0 } else { 1.0 },
                    );
                }
                if t.age % 6 == 0 {
                    sound(game, &config, step_sound(&name), at, 0.3, 1.1);
                }
            }
        }
        i += 1;
    }
    TREMBLING.store(!st.trembles.is_empty(), Ordering::Relaxed);
}

/// Trees shaken by hand in the level `dim`: the trunk shivers, leaves flutter
/// down and a few of what leaves give (sticks, saplings, apples) fall out; a
/// tree gives once every 30 seconds.
fn tick_shakes(game: &mut Game<'_>, st: &mut State, dim: &str) {
    if st.shakes.is_empty() {
        return;
    }
    let config = st.config.clone();
    let (here, rest): (Vec<ShakeReq>, Vec<ShakeReq>) = std::mem::take(&mut st.shakes)
        .into_iter()
        .partition(|r| r.dim == dim);
    st.shakes = rest;
    for r in here {
        // one shake a player every half second
        if st.shook.get(&r.player).is_some_and(|t| st.tick < t + 10) {
            continue;
        }
        st.shook.insert(r.player, st.tick);
        let Some((base, base_state)) = trunk_base(game, r.pos) else {
            continue;
        };
        let Some(state) = state_at(game, r.pos) else {
            continue;
        };
        let tree = {
            let placed = st.placed.get(dim);
            let protect = config.on(Key::Protect);
            let is_placed = |q: Pos| protect && placed.is_some_and(|set| set.contains(&q));
            let mut reader = Reader::new(game, &config);
            let wood = reader.wood(&name_of(game, base_state));
            let mut look = core::Look::new(&mut reader);
            core::scan(
                &mut look,
                &is_placed,
                base,
                wood,
                &core::Limits::of(&config),
            )
        };
        let Ok(tree) = tree else {
            continue;
        };
        // the trunk shivers a moment
        st.trembles.push(Tremble {
            dim: r.dim.clone(),
            pos: r.pos,
            face: r.face,
            state,
            player: r.player,
            ticks: 8,
            age: 0,
            display: None,
            leaves: Vec::new(),
            checked: false,
            stopped: false,
            broken: 0,
            shaking: true,
            tint: None,
            preview: Vec::new(),
        });
        TREMBLING.store(true, Ordering::Relaxed);
        let leaf_name = tree
            .leaves
            .first()
            .and_then(|p| state_at(game, *p))
            .map(|id| name_of(game, id));
        if let Some(l) = &leaf_name {
            for k in 0..2.min(tree.leaves.len()) {
                let p = tree.leaves[(k * 7) % tree.leaves.len()];
                let at = [0, 1, 2].map(|i| f64::from(p[i]) + 0.5);
                sound(game, &config, step_sound(l), at, 0.7, 0.9 + 0.2 * k as f64);
            }
        }
        let key = (r.dim.clone(), base);
        if st.shaken.get(&key).is_some_and(|t| st.tick < t + 600) {
            continue;
        }
        st.shaken.insert(key, st.tick);
        let rolls = (tree.leaves.len() / 4).clamp(2, 12);
        for _ in 0..rolls {
            let leaf = tree.leaves[st.rng.below(tree.leaves.len() as u32) as usize];
            let Some(id) = state_at(game, leaf) else {
                continue;
            };
            if !in_tag(game, id, "minecraft:leaves") {
                continue;
            }
            let at = [0, 1, 2].map(|i| f64::from(leaf[i]) + 0.5 - if i == 1 { 0.6 } else { 0.0 });
            for item in game.block_loot_at(leaf, id, None) {
                if item.item.ends_with("_leaves") {
                    continue;
                }
                drop_item(game, &mut st.rng, at, item, None);
            }
        }
    }
}

/// What a log that starts to tremble shows.
struct TrembleSetup {
    display: u32,
    leaves: Vec<Pos>,
    tint: Option<u32>,
    preview: Vec<[f64; 3]>,
}

/// A dig that just started (or a tree shaken): if its digger may fell the tree
/// behind the log, the log's trembling display, some of the tree's lowest leaves,
/// their tint and, for a dig, where the trunk will lie.
fn tremble_start(game: &mut Game<'_>, st: &State, i: usize) -> Option<TrembleSetup> {
    let t = &st.trembles[i];
    let config = st.config.clone();
    let who = cutter(game, t.player)?;
    if who.dim != t.dim || (!t.shaking && who.game_mode == 1) {
        return None;
    }
    if !t.shaking {
        let axe = who.held.as_ref().is_some_and(|s| {
            game.native_server_item_in_tag(s, "minecraft:axes")
                .unwrap_or(false)
        });
        if !core::fells(&config, who.game_mode, who.sneaking, axe) {
            return None;
        }
    }
    let (tree, preview) = {
        let placed = st.placed.get(&t.dim);
        let protect = config.on(Key::Protect);
        let is_placed = |q: Pos| protect && placed.is_some_and(|set| set.contains(&q));
        let mut reader = Reader::new(game, &config);
        let wood = reader.wood(&name_of(game, t.state));
        let mut look = core::Look::new(&mut reader);
        let tree = core::scan(
            &mut look,
            &is_placed,
            t.pos,
            wood,
            &core::Limits::of(&config),
        )
        .ok()?;
        drop(look);
        // where the trunk will lie, as it would fall if the log broke now (the
        // log's own place counted open)
        let mut preview = Vec::new();
        if !t.shaking && config.on(Key::Preview) && config.on(Key::Animation) {
            let mut holed = Holed {
                inner: &mut reader,
                hole: t.pos,
            };
            let mut look = core::Look::new(&mut holed);
            let sink = core::gap(&mut look, &tree);
            let dir = core::choose_dir(
                &tree,
                &mut look,
                f64::from(who.yaw),
                config.on(Key::Aim),
                config.get(Key::Cone),
                sink,
            );
            let body = core::Body::new(&tree, dir, sink);
            let stop = body.contact(&mut look, true).theta;
            let mut seen = HashSet::new();
            for (k, part) in body.parts.iter().enumerate().filter(|(_, p)| !p.leaf) {
                let c = body.center(k, stop);
                let cell = [c[0].floor() as i32, c[2].floor() as i32];
                if seen.insert(cell)
                    && let Some(y) = ground_under(game, [cell[0], c[1].floor() as i32 + 1, cell[1]])
                {
                    preview.push([c[0], y, c[2]]);
                }
                let _ = part;
            }
        }
        (tree, preview)
    };
    let mut leaves = tree.leaves.clone();
    leaves.sort_by_key(|p| p[1]);
    leaves.truncate(12);
    let leaf = tree
        .leaves
        .first()
        .and_then(|p| state_at(game, *p))
        .map(|id| name_of(game, id));
    let tint = tint_for(game, leaf.as_deref(), t.pos);
    // the display stands in the cell in front of the face dug, so it is lit as
    // that side is
    let anchor = [0, 1, 2].map(|k| f64::from(t.pos[k] + t.face[k]) + 0.5);
    let (tr, scale) = core::tremble_pose(0, 0.0, t.face);
    let display = summon_display(game, &t.dim, anchor, t.state, tr, scale, st.session, 0)?;
    Some(TrembleSetup {
        display,
        leaves,
        tint,
        preview,
    })
}

/// The level with one cell open: the log being dug, as it will be once broken.
struct Holed<'r, 'g, 'a> {
    inner: &'r mut Reader<'g, 'a>,
    hole: Pos,
}

impl core::World for Holed<'_, '_, '_> {
    fn cell(&mut self, p: Pos) -> Option<Cell> {
        if p == self.hole {
            return Some(Cell::Open);
        }
        core::World::cell(self.inner, p)
    }
}

/// Whether the cell at `p` is open (air, plants).
fn look_is_open(game: &Game<'_>, p: Pos) -> bool {
    state_at(game, p).is_some_and(|id| {
        let b = game.res.behaviours();
        b.is_air(id) || b.is_replaceable(id) || b.collision_shape(id).is_some_and(|s| s.is_empty())
    })
}

/// The top of the ground under `p` (searching six blocks down): just above the
/// first block something can stand on.
fn ground_under(game: &Game<'_>, p: Pos) -> Option<f64> {
    for dy in 0..8 {
        let q = [p[0], p[1] - dy, p[2]];
        if !look_is_open(game, q) && state_at(game, q).is_some() {
            return Some(f64::from(q[1]) + 1.05);
        }
    }
    None
}

// ---------------------------------------------------------------------------------
// A chain reaction
// ---------------------------------------------------------------------------------

/// A falling trunk came down on other trees' logs.
struct Knock {
    hitter: u64,
    credit: Option<(u128, String)>,
    cells: Vec<Pos>,
    dir: [f64; 2],
    generation: u32,
    tool: Option<ItemStack>,
    drops: bool,
    owner: Owner,
    omega: f64,
    at: [f64; 3],
}

/// The lowest log of the trunk `cell` is in (same kind of log all the way down).
fn trunk_base(game: &Game<'_>, cell: Pos) -> Option<(Pos, BlockStateId)> {
    let id = state_at(game, cell)?;
    if !is_trunk(game, id) {
        return None;
    }
    let kind = species(&name_of(game, id)).to_owned();
    let mut base = (cell, id);
    for _ in 0..64 {
        let below = [base.0[0], base.0[1] - 1, base.0[2]];
        match state_at(game, below) {
            Some(b) if is_trunk(game, b) && species(&name_of(game, b)) == kind => {
                base = (below, b);
            }
            _ => break,
        }
    }
    Some(base)
}

/// Each tree whose trunk was hit falls in turn, the way it was pushed; the tree
/// that hit it, its way clear, falls on.
fn chain(game: &mut Game<'_>, st: &mut State, dim: &str, k: Knock) {
    let config = st.config.clone();
    let mut bases = HashSet::new();
    let mut felled = false;
    for cell in &k.cells {
        let Some((base, state)) = trunk_base(game, *cell) else {
            continue;
        };
        if !bases.insert(base) {
            continue;
        }
        let start = Start {
            dim: dim.to_owned(),
            cut: base,
            cut_state: state,
            yaw: core::dir_yaw(k.dir),
            aim: true,
            cone: 20,
            tool: k.tool.clone(),
            drops: k.drops,
            owner: k.owner,
            cutter: None,
            credit: k.credit.clone(),
            wear: false,
            tired: false,
            ghost: None,
            generation: k.generation,
            standing: true,
        };
        if fell_tree(game, st, start) {
            felled = true;
        }
    }
    if !felled {
        return;
    }
    // bonk!
    sound(game, &config, "minecraft:block.wood.hit", k.at, 1.0, 0.7);
    sound(game, &config, CRACK, k.at, 0.7, 0.8);
    sound(game, &config, PLOP, k.at, 0.7, 0.7);
    particles(
        game,
        &config,
        ParticleOptions::simple(ParticleType::Crit),
        None,
        k.at,
        [0.3, 0.3, 0.3],
        0.3,
        8.0,
    );
    let Some(t) = st.trees.iter_mut().find(|t| t.id == k.hitter) else {
        return;
    };
    let (contact, crush, hits) = {
        let mut reader = Reader::new(game, &config);
        let mut look = core::Look::new(&mut reader);
        let contact = t.fell.body.contact(&mut look, true);
        let crush = t.fell.body.crush(&mut look, contact.theta);
        let hits = log_hits(&mut look, &contact);
        (contact, crush, hits)
    };
    t.contacts = dust_points(game, &contact);
    t.hits = hits;
    t.resumes += 1;
    t.settled = false;
    t.fell.resume(&contact, crush, k.omega * 0.55);
}

// ---------------------------------------------------------------------------------
// A tick of a falling tree
// ---------------------------------------------------------------------------------

/// Moves `t` a tick in the level in place: whether it is done, and the trees its
/// trunk knocks into.
fn tick_tree(
    game: &mut Game<'_>,
    config: &Config,
    rng: &mut core::Rng,
    t: &mut Falling,
) -> (bool, Option<Knock>) {
    let was_falling = t.fell.phase == Phase::Falling;
    let frame = t.fell.tick();
    let theta = frame.theta;
    play_later(game, config, t);

    // the log that trembled as it was dug pops
    if let Some(id) = t.ghost.take() {
        game.native_remove_entity(&t.dim, id);
        let at = t.anchor;
        block_bits(game, config, t.cut_state, at, 10.0);
        particles(
            game,
            config,
            ParticleOptions::simple(ParticleType::Poof),
            None,
            at,
            [0.2, 0.2, 0.2],
            0.04,
            5.0,
        );
        sound(game, config, PLOP, at, 0.8, 1.3);
        sound(game, config, BUBBLE, at, 0.7, 1.2);
        sound(game, config, CRACK, at, 0.5, 1.2);
    }

    let body = &t.fell.body;
    if frame.moved {
        let q = body.quat(theta);
        for i in t.fell.remaining() {
            if let Some(id) = t.ents[i] {
                pose(
                    game,
                    id,
                    body.translation(body.parts[i].cell, theta, t.anchor),
                    q,
                );
            }
        }
    }
    if frame.landed.is_some() {
        // down on the stump: pomf
        let at = [body.pivot[0], body.pivot[1] - body.sink, body.pivot[2]];
        sound(game, config, WOOL, at, 0.9, 0.75);
        sound(game, config, THUD, at, 0.5, 0.8);
        block_bits(game, config, t.cut_state, at, 6.0);
    }
    if !t.creaked && t.fell.phase == Phase::Falling {
        // it starts to go: the fibres crack and the trunk groans
        t.creaked = true;
        let body = &t.fell.body;
        let base = [
            body.pivot[0],
            body.pivot[1] - body.lowered + 0.5,
            body.pivot[2],
        ];
        let mid = body.center(t.middle, theta);
        sound(game, config, CRACK, base, 0.7, 0.65);
        sound(game, config, CREAK, mid, 0.7, 0.55);
        let now = t.fell.age;
        t.later.push((now + 7, CREAK, mid, 0.5, 0.5));
        t.creak_age = Some(now);
        // a big tree's birds take off
        if config.on(Key::Birds) && t.leaves.len() >= 25 {
            let crown = t.fell.body.center(t.tip, theta);
            sound(game, config, "minecraft:entity.parrot.fly", crown, 0.7, 1.1);
            sound(
                game,
                config,
                "minecraft:entity.parrot.ambient",
                crown,
                0.6,
                1.5,
            );
            t.later
                .push((now + 3, "minecraft:entity.parrot.ambient", crown, 0.5, 1.75));
            t.later
                .push((now + 5, "minecraft:entity.parrot.fly", crown, 0.5, 1.3));
            particles(
                game,
                config,
                ParticleOptions::simple(ParticleType::Cloud),
                None,
                [crown[0], crown[1] + 1.0, crown[2]],
                [1.2, 0.4, 1.2],
                0.06,
                6.0,
            );
        }
    }
    // apples come loose as it falls; what is left goes when it lands
    if let Some(start) = t.creak_age
        && !t.apples.is_empty()
    {
        let lying = t.fell.phase != Phase::Falling;
        let now = t.fell.age;
        let mut k = 0;
        let mut plopped = false;
        while k < t.apples.len() {
            if lying || now >= start + t.apples[k].2 {
                let (i, apple, _) = t.apples.remove(k);
                let at = t.fell.body.center(i, theta);
                drop_item(game, rng, at, apple, t.owner);
                if !plopped {
                    plopped = true;
                    sound(game, config, PLOP, at, 0.3, 1.5);
                }
            } else {
                k += 1;
            }
        }
    }
    if was_falling {
        falling_effects(game, config, t);
        strike(game, config, t);
    }
    let mut knock = None;
    if let Some(impact) = frame.impact {
        impact_effects(game, config, rng, t, impact);
        let tip_speed = impact.omega.abs() * t.fell.body.reach;
        if impact.first
            && impact.ground
            && i64::from(t.generation) < i64::from(config.get(Key::Chain))
            && t.resumes < 3
            && !t.hits.is_empty()
            && tip_speed > 2.0
        {
            let at = t.contacts.first().map_or(t.anchor, |c| c.0);
            knock = Some(Knock {
                hitter: t.id,
                credit: t.credit.clone(),
                cells: std::mem::take(&mut t.hits),
                dir: t.fell.body.dir,
                generation: t.generation + 1,
                tool: t.tool.clone(),
                drops: t.drops,
                owner: t.owner,
                omega: impact.omega,
                at,
            });
        }
    }
    if was_falling && t.fell.phase != Phase::Falling && !t.settled {
        settle(game, t);
    }

    // leaves crushed under the trunk
    let mut squished = false;
    for i in frame.crushed {
        let at = t.fell.body.center(i, theta);
        if let Some(id) = t.ents[i].take() {
            game.native_remove_entity(&t.dim, id);
        }
        block_bits(game, config, t.states[i], at, 4.0);
        if !squished {
            squished = true;
            sound(
                game,
                config,
                "minecraft:block.moss_carpet.place",
                at,
                0.5,
                1.2,
            );
            if let Some(l) = &t.leaf {
                sound(game, config, break_sound(l), at, 0.4, 1.3);
            }
        }
        for item in std::mem::take(&mut t.loot[i]) {
            drop_item(game, rng, at, item, t.owner);
        }
    }

    if matches!(t.fell.phase, Phase::Lying(_) | Phase::Breaking)
        && config.on(Key::Leaves)
        && t.fell.age % 3 == 0
    {
        twirl(game, config, rng, t);
    }

    // from the base up, each block puffs up and pops
    let body = &t.fell.body;
    for &(i, scale, lift) in &frame.popping {
        if let Some(id) = t.ents[i] {
            let (tr, q) = body.pose(i, theta, t.anchor, scale, lift);
            pose_scaled(game, id, tr, q, scale as f32);
        }
    }
    if let Some(n) = frame.note
        && let Some(&(i, ..)) = frame.popping.last()
    {
        // the pops rise in pitch as the breaking climbs
        let at = t.fell.body.center(i, theta);
        sound(game, config, BUBBLE, at, 0.5, 0.75 + 0.07 * n as f64);
        particles(
            game,
            config,
            ParticleOptions::simple(ParticleType::HappyVillager),
            None,
            [at[0], at[1] + 0.5, at[2]],
            [0.3, 0.2, 0.3],
            0.0,
            2.0,
        );
    }
    let mut plops = 0;
    for i in frame.broke {
        let at = t.fell.body.center(i, theta);
        if let Some(id) = t.ents[i].take() {
            game.native_remove_entity(&t.dim, id);
        }
        block_bits(game, config, t.states[i], at, 3.0);
        particles(
            game,
            config,
            ParticleOptions::simple(ParticleType::Poof),
            None,
            at,
            [0.15, 0.15, 0.15],
            0.02,
            1.0,
        );
        if plops < 1 {
            plops += 1;
            sound(game, config, PLOP, at, 0.3, 1.2 + rng.unit() * 0.4);
        }
        for item in std::mem::take(&mut t.loot[i]) {
            drop_item(game, rng, at, item, t.owner);
        }
    }
    if frame.done {
        for (i, apple, _) in std::mem::take(&mut t.apples) {
            let at = t.fell.body.center(i, theta);
            drop_item(game, rng, at, apple, t.owner);
        }
    }
    if frame.done
        && let Some((sapling, spots, from_loot)) = t.replant.take()
    {
        replant(game, config, rng, &t.dim, sapling, &spots, from_loot);
    }
    if frame.done {
        // all gone: a last pop and a few hearts where the crown was
        let at = t.fell.body.center(t.tip, theta);
        sound(game, config, PLOP, at, 0.7, 1.0);
        sound(game, config, BUBBLE, at, 0.6, 1.5);
        sound(game, config, "minecraft:entity.item.pickup", at, 0.4, 1.6);
        particles(
            game,
            config,
            ParticleOptions::simple(ParticleType::Heart),
            None,
            [at[0], at[1] + 0.5, at[2]],
            [0.6, 0.3, 0.6],
            0.0,
            3.0,
        );
    }
    (frame.done, knock)
}

/// The sounds whose tick has come.
fn play_later(game: &mut Game<'_>, config: &Config, t: &mut Falling) {
    let now = t.fell.age;
    let mut i = 0;
    while i < t.later.len() {
        if t.later[i].0 <= now {
            let (_, name, at, volume, pitch) = t.later.remove(i);
            sound(game, config, name, at, volume, pitch);
        } else {
            i += 1;
        }
    }
}

/// The displays' final pose kept by the server, so players who come later see the
/// tree lying where it lies.
fn settle(game: &mut Game<'_>, t: &mut Falling) {
    t.settled = true;
    let theta = t.fell.theta();
    let body = &t.fell.body;
    let q = body.quat(theta);
    for i in t.fell.remaining() {
        if let Some(id) = t.ents[i] {
            let d = NativeDisplay {
                translation: body.translation(body.parts[i].cell, theta, t.anchor),
                left_rotation: q,
                ..NativeDisplay::default()
            };
            game.native_set_display(&t.dim, id, &d);
        }
    }
}

/// The crown rustles as it gathers speed, and flutters once it is fast.
fn falling_effects(game: &mut Game<'_>, config: &Config, t: &mut Falling) {
    let body = &t.fell.body;
    let tip_speed = t.fell.motion.omega.abs() * body.reach;
    let crown = body.center(t.tip, t.fell.theta());
    if let Some(leaf) = &t.leaf
        && tip_speed > 3.0
        && t.fell.age % 4 == 0
    {
        let v = (tip_speed / 14.0).clamp(0.2, 0.7);
        sound(game, config, step_sound(leaf), crown, v, 0.8);
    }
    if !t.whoosh && tip_speed > 6.0 {
        t.whoosh = true;
        sound(
            game,
            config,
            "minecraft:entity.parrot.fly",
            crown,
            0.7,
            0.75,
        );
        sound(
            game,
            config,
            "minecraft:entity.player.attack.sweep",
            crown,
            0.2,
            0.8,
        );
    }
}

/// The ground takes it softly: a deep pomf and a squish, then a little rising
/// arpeggio; the soil's dust and a burst of leaves. A bounce is a boing.
fn impact_effects(
    game: &mut Game<'_>,
    config: &Config,
    rng: &mut core::Rng,
    t: &mut Falling,
    impact: core::Impact,
) {
    if !impact.ground {
        return;
    }
    let theta = t.fell.theta();
    let body = &t.fell.body;
    let logs = body.logs as f64;
    let speed = (impact.omega.abs() * body.reach / 10.0).clamp(0.4, 1.3);
    let mid = body.center(t.middle, theta);
    let crown = body.center(t.tip, theta);
    if impact.first {
        // a deep wooden thud as big as the tree, softened, branches snapping and
        // the crown crashing, then a few bubbly pops
        let deep = (0.75 - logs * 0.008).clamp(0.5, 0.75);
        sound(game, config, THUD, mid, 1.0 * speed, deep);
        sound(game, config, WOOL, mid, 1.0 * speed, 0.6);
        sound(game, config, CRACK, mid, 0.7, 0.6);
        if let Some(l) = &t.leaf {
            sound(game, config, break_sound(l), crown, 0.9, 0.8);
        }
        let now = t.fell.age;
        for (k, pitch) in [(2, 1.0), (3, 1.25), (5, 1.45)] {
            t.later.push((now + k, BUBBLE, crown, 0.4, pitch));
        }
    } else {
        // a bounce: a lighter thud and a little pop
        sound(game, config, THUD, mid, 0.4 + 0.3 * speed, 1.0);
        sound(game, config, "minecraft:block.wool.step", mid, 0.6, 0.9);
        sound(game, config, BUBBLE, mid, 0.3, 1.3);
    }
    if config.on(Key::Dust) {
        let k = if impact.first { 1.0 } else { 0.35 };
        for (at, ground) in t.contacts.clone() {
            particles(
                game,
                config,
                ParticleOptions::simple(ParticleType::DustPillar),
                Some(ground),
                at,
                [0.3, 0.05, 0.3],
                0.2,
                5.0 * k * speed,
            );
            particles(
                game,
                config,
                ParticleOptions::simple(ParticleType::Block),
                Some(ground),
                at,
                [0.45, 0.1, 0.45],
                0.15,
                8.0 * k * speed,
            );
            if impact.first {
                particles(
                    game,
                    config,
                    ParticleOptions::simple(ParticleType::Poof),
                    None,
                    at,
                    [0.3, 0.1, 0.3],
                    0.02,
                    1.0,
                );
            }
        }
    }
    if impact.first
        && config.on(Key::Leaves)
        && let Some(l) = t.leaf.clone()
    {
        let alive: Vec<usize> = t
            .leaves
            .iter()
            .copied()
            .filter(|i| t.fell.alive[*i])
            .collect();
        for _ in 0..10.min(alive.len()) {
            let i = alive[rng.below(alive.len() as u32) as usize];
            let at = t.fell.body.center(i, theta);
            if let Some(o) = leaf_particle(&l, t.tint) {
                particles(game, config, o, None, at, [0.5, 0.4, 0.5], 0.03, 3.0);
            }
        }
    }
}

/// Leaves drift down from the crown while it lies there.
fn twirl(game: &mut Game<'_>, config: &Config, rng: &mut core::Rng, t: &Falling) {
    let Some(l) = &t.leaf else {
        return;
    };
    let alive: Vec<usize> = t
        .leaves
        .iter()
        .copied()
        .filter(|i| t.fell.alive[*i])
        .collect();
    if alive.is_empty() {
        return;
    }
    for _ in 0..3 {
        let i = alive[rng.below(alive.len() as u32) as usize];
        let mut at = t.fell.body.center(i, t.fell.theta());
        at[1] += 0.4;
        if let Some(o) = leaf_particle(l, t.tint) {
            particles(game, config, o, None, at, [0.4, 0.2, 0.4], 0.0, 1.0);
        }
    }
}

/// What the falling tree comes down on: mobs and players allowed by the settings,
/// hurt by how fast the block that meets them moves, and pushed along the fall.
fn strike(game: &mut Game<'_>, config: &Config, t: &mut Falling) {
    let omega = t.fell.motion.omega;
    if !config.on(Key::Damage) || omega.abs() < 0.3 {
        return;
    }
    let theta = t.fell.theta();
    let body = &t.fell.body;
    let reach = body.reach + 2.0;
    let pivot = body.pivot;
    let dir = body.dir;
    let near = |p: [f64; 3]| {
        let d: f64 = (0..3).map(|k| (p[k] - pivot[k]).powi(2)).sum();
        d <= reach * reach
    };
    let max = f64::from(config.get(Key::Strength));
    let harm = |hit: Option<(f64, bool)>| -> Option<f32> {
        let (speed, leaf) = hit?;
        let felt = if leaf { speed * 0.3 } else { speed };
        (felt >= 2.0).then(|| (felt * 0.9).clamp(1.0, max) as f32)
    };
    // mobs
    let mut hits = Vec::new();
    for m in game.mobs.iter() {
        if m.removed || !m.alive() || t.struck.contains(&m.id) {
            continue;
        }
        let pos = [m.position.x, m.position.y, m.position.z];
        if !near(pos) {
            continue;
        }
        let id = &m.kind.id;
        let kind = if id.contains(':') {
            id.to_string()
        } else {
            format!("minecraft:{id}")
        };
        let ally = m.extra.owner.is_some() || ALLIES.contains(&kind.as_str());
        let allowed = if ally {
            config.on(Key::Allies)
        } else if m.kind.category == "monster" {
            config.on(Key::Monsters)
        } else {
            config.on(Key::Animals)
        };
        if !allowed {
            continue;
        }
        let hw = m.kind.width / 2.0;
        let lo = [pos[0] - hw, pos[1], pos[2] - hw];
        let hi = [pos[0] + hw, pos[1] + m.kind.height, pos[2] + hw];
        if let Some(dmg) = harm(body.strike(theta, omega, lo, hi, &t.fell.alive)) {
            hits.push((m.id, dmg, pos));
        }
    }
    for (id, dmg, pos) in hits {
        t.struck.insert(id);
        if let Some(m) = game.mobs.iter_mut().find(|m| m.id == id) {
            m.damage(
                dmg,
                Damage {
                    kind: DamageKind::FallingBlock,
                    attacker: None,
                    from: Some([pos[0] - dir[0], pos[1], pos[2] - dir[1]]),
                },
            );
        }
    }
    // players
    if !config.on(Key::Players) {
        return;
    }
    let mut struck = Vec::new();
    for p in game.native_server_players() {
        if p.dimension != t.dim
            || !(p.game_mode == 0 || p.game_mode == 2)
            || p.health <= 0.0
            || t.struck_players.contains(&p.uuid)
            || !near(p.position)
        {
            continue;
        }
        let pos = p.position;
        let lo = [pos[0] - 0.3, pos[1], pos[2] - 0.3];
        let hi = [pos[0] + 0.3, pos[1] + 1.8, pos[2] + 0.3];
        if let Some(dmg) = harm(body.strike(theta, omega, lo, hi, &t.fell.alive)) {
            struck.push((p.uuid, p.name, dmg, pos));
        }
    }
    for (uuid, name, dmg, pos) in struck {
        t.struck_players.insert(uuid);
        game.native_run_command(&format!(
            "damage {name} {dmg:.1} minecraft:falling_block at {} {} {}",
            pos[0] - dir[0],
            pos[1],
            pos[2] - dir[1]
        ));
    }
}

/// Ends a tree at once: its displays go and what it still holds drops where each
/// block is. In the level in place it is done directly; elsewhere with commands.
fn end_tree(game: &mut Game<'_>, t: &mut Falling) {
    let theta = t.fell.theta();
    let here = game.dim.id() == t.dim;
    let mut rng = core::Rng::new(t.id ^ 0x5EED);
    if here && let Some(id) = t.ghost.take() {
        game.native_remove_entity(&t.dim, id);
    }
    for (i, apple, _) in std::mem::take(&mut t.apples) {
        t.loot[i].push(apple);
        t.fell.alive[i] = true;
    }
    let remaining: Vec<usize> = t.fell.remaining().collect();
    for i in remaining {
        t.fell.alive[i] = false;
        let at = t.fell.body.center(i, theta);
        let items = std::mem::take(&mut t.loot[i]);
        if here {
            if let Some(id) = t.ents[i].take() {
                game.native_remove_entity(&t.dim, id);
            }
            for item in items {
                drop_item(game, &mut rng, at, item, None);
            }
        } else {
            for item in items {
                let mut c = Compound::new();
                c.insert("Item".into(), horizon_items::item_nbt::item_to_nbt(&item));
                c.insert("PickupDelay".into(), Tag::Short(10));
                let snbt = horizon_world::snbt_text::to_snbt(&Tag::Compound(c));
                game.native_run_command(&format!(
                    "execute in {} run summon minecraft:item {} {} {} {snbt}",
                    t.dim, at[0], at[1], at[2]
                ));
            }
        }
    }
    if !here {
        game.native_run_command(&format!(
            "execute in {} run kill @e[type=minecraft:block_display,tag=chop.t{}]",
            t.dim, t.id
        ));
    }
    if let Some((sapling, spots, from_loot)) = t.replant.take() {
        if here {
            let config = Config::default();
            replant(
                game,
                &config,
                &mut rng,
                &t.dim.clone(),
                sapling,
                &spots,
                from_loot,
            );
        } else {
            for p in spots {
                game.native_run_command(&format!(
                    "execute in {} run setblock {} {} {} {sapling} keep",
                    t.dim, p[0], p[1], p[2]
                ));
            }
        }
    }
}

// ---------------------------------------------------------------------------------
// /chop
// ---------------------------------------------------------------------------------

fn color(name: &str) -> u32 {
    named_color(name).unwrap_or(0xFFFFFF)
}

fn span(text: &str, color_name: &str) -> Text {
    Text::styled(text, Style::color(color(color_name)))
}

fn bold(text: &str, color_name: &str) -> Text {
    Text::styled(
        text,
        Style {
            bold: Some(true),
            ..Style::color(color(color_name))
        },
    )
}

/// `label` that runs `command` when clicked, with `hint` on hover.
fn button(label: &str, color_name: &str, hint: &str, command: &str) -> Text {
    Text::styled(
        label,
        Style {
            click: Some(ClickEvent::RunCommand(command.to_owned())),
            hover: Some(Box::new(Text::literal(hint))),
            ..Style::color(color(color_name))
        },
    )
}

fn hinted(text: &str, color_name: &str, hint: &str) -> Text {
    Text::styled(
        text,
        Style {
            hover: Some(Box::new(Text::literal(hint))),
            ..Style::color(color(color_name))
        },
    )
}

/// `/chop` in every player's command tree. The settings are checked when run, so
/// the owner of a single player world uses them without cheats.
pub(super) fn register(commands: &mut Vec<NativeCommand>) {
    for usage in [
        "chop",
        "chop on",
        "chop off",
        "chop aide",
        "chop config",
        "chop config <page:word>",
        "chop set <option:word> <value:integer>",
        "chop reset",
        "chop perso",
        "chop perso <option:word> <value:integer>",
        "chop stats",
    ] {
        commands.push(NativeCommand::new(usage));
    }
}

/// `native_perform_command`: `/chop ...`.
pub(super) fn perform(
    game: &mut Game<'_>,
    player: &NativeServerPlayer,
    command: &str,
    ran: &mut bool,
) {
    let words: Vec<&str> = command.split_whitespace().collect();
    if words.first() != Some(&"chop") {
        return;
    }
    *ran = true;
    let uuid = player.uuid;
    let admin = player.permission >= 2 || owner(game, uuid);
    let reply = with_state(game, |_, st| -> Vec<Text> {
        match words[1..] {
            [] | ["on"] | ["off"] => {
                let off = match words.get(1) {
                    Some(&"on") => false,
                    Some(&"off") => true,
                    _ => !st.off.contains(&uuid),
                };
                if off {
                    st.off.insert(uuid);
                } else {
                    st.off.remove(&uuid);
                }
                st.dirty = true;
                vec![toggled(off)]
            }
            ["config" | "set" | "reset", ..] if !admin => vec![span(
                "🪓 Chop! Les réglages sont réservés aux opérateurs (et au propriétaire du monde).",
                "red",
            )],
            ["config"] => page(&st.config, Section::Fall),
            ["config", w] => match Section::from_word(w) {
                Some(s) => page(&st.config, s),
                None => vec![span(
                    "🪓 Chop! Pages : chute, effets, degats, regles.",
                    "red",
                )],
            },
            ["set", word, value] => match value.parse::<i32>() {
                Ok(v) => match st.config.set(word, v) {
                    Ok(o) => {
                        st.dirty = true;
                        page(&st.config, o.section)
                    }
                    Err(e) => vec![span(&format!("🪓 Chop! {e}."), "red")],
                },
                Err(_) => vec![span(
                    "🪓 Chop! La valeur doit être un nombre entier.",
                    "red",
                )],
            },
            ["reset"] => {
                st.config = Config::default();
                st.dirty = true;
                let mut lines = vec![span("🪓 Chop! Réglages par défaut remis.", "green")];
                lines.extend(page(&st.config, Section::Fall));
                lines
            }
            ["perso"] => perso_page(&st.config, st.perso.get(&uuid).copied().unwrap_or_default()),
            ["perso", word, value] => match (
                value.parse::<i32>(),
                PERSO_OPTS.iter().find(|o| o.0 == word),
            ) {
                (Ok(v), Some(o)) if (-1..=o.2).contains(&v) => {
                    let p = st.perso.entry(uuid).or_default();
                    match o.0 {
                        "sons" => p.volume = v,
                        "particules" => p.particles = v,
                        _ => p.preview = v,
                    }
                    st.dirty = true;
                    perso_snapshot(st);
                    perso_page(&st.config, st.perso.get(&uuid).copied().unwrap_or_default())
                }
                _ => vec![span(
                    "🪓 Chop! Réglages perso : sons (0 à 200), particules (0 à 3), apercu (0 ou 1), -1 pour suivre le monde.",
                    "red",
                )],
            },
            ["stats"] => stats_lines(st, uuid),
            _ => help(admin),
        }
    });
    let lines = reply.unwrap_or_else(|| vec![span("🪓 Chop! est occupé, réessaie.", "red")]);
    for line in lines {
        game.native_feedback(line);
    }
}

fn toggled(off: bool) -> Text {
    let head = bold("🪓 Chop! ", "gold");
    if off {
        head.append(span("désactivé", "red"))
            .append(span(" pour toi : tu coupes bûche par bûche. ", "gray"))
            .append(button("[Réactiver]", "green", "/chop on", "/chop on"))
    } else {
        head.append(span("activé", "green"))
            .append(span(" pour toi : une bûche coupée abat l'arbre. ", "gray"))
            .append(button("[Désactiver]", "red", "/chop off", "/chop off"))
    }
}

fn help(admin: bool) -> Vec<Text> {
    let mut lines = vec![
        bold("🪓 Chop! ", "gold").append(span(
            "Coupe une bûche à la hache : tout l'arbre tombe.",
            "gray",
        )),
        button("/chop", "yellow", "Cliquer pour basculer", "/chop")
            .append(span(" active ou désactive Chop! pour toi.", "gray")),
        button("/chop perso", "yellow", "Tes réglages", "/chop perso")
            .append(span(" tes propres sons, particules et aperçu.", "gray")),
        button("/chop stats", "yellow", "Tes arbres", "/chop stats")
            .append(span(" tes arbres abattus et le classement.", "gray")),
    ];
    if admin {
        lines.push(
            button(
                "/chop config",
                "yellow",
                "Ouvrir les réglages",
                "/chop config",
            )
            .append(span(" ouvre les réglages (cliquables).", "gray")),
        );
        lines.push(
            span("/chop set <réglage> <valeur>", "yellow")
                .append(span(" change un réglage.", "gray")),
        );
        lines.push(
            span("/chop reset", "yellow").append(span(" remet les réglages par défaut.", "gray")),
        );
    }
    lines
}

/// A player's own settings: the word, the label, the highest value, the step and
/// how the value reads.
const PERSO_OPTS: [(&str, &str, i32, i32, Unit); 3] = [
    ("sons", "Volume des sons", 200, 25, Unit::Percent),
    (
        "particules",
        "Particules",
        3,
        1,
        Unit::Choice(core::PARTICLES),
    ),
    ("apercu", "Aperçu de la chute", 1, 1, Unit::Switch),
];

/// `/chop perso`: the player's own settings, each one either theirs or the
/// world's.
fn perso_page(config: &Config, p: Perso) -> Vec<Text> {
    let mut lines = vec![bold("🪓 Chop! ", "gold").append(bold("[Tes réglages]", "white"))];
    for (word, label, max, step, unit) in PERSO_OPTS {
        let (own, world) = match word {
            "sons" => (p.volume, config.get(Key::Volume)),
            "particules" => (p.particles, config.get(Key::Particles)),
            _ => (p.preview, config.get(Key::Preview)),
        };
        let v = if own >= 0 { own } else { world };
        let mut line = span("  ", "gray")
            .append(span(label, "gray"))
            .append(span(" : ", "dark_gray"));
        if unit == Unit::Switch {
            let (yes, no) = if v != 0 {
                ("green", "dark_gray")
            } else {
                ("dark_gray", "red")
            };
            line = line
                .append(button(
                    "[oui]",
                    yes,
                    "oui",
                    &format!("/chop perso {word} 1"),
                ))
                .append(span(" ", "gray"))
                .append(button("[non]", no, "non", &format!("/chop perso {word} 0")));
        } else {
            let less = (v - step).max(0);
            let more = (v + step).min(max);
            line = line
                .append(button(
                    "[-]",
                    "red",
                    &unit.show(less),
                    &format!("/chop perso {word} {less}"),
                ))
                .append(span(" ", "gray"))
                .append(bold(&unit.show(v), "yellow"))
                .append(span(" ", "gray"))
                .append(button(
                    "[+]",
                    "green",
                    &unit.show(more),
                    &format!("/chop perso {word} {more}"),
                ));
        }
        line = if own >= 0 {
            line.append(span("  ", "gray")).append(button(
                "[comme le monde]",
                "aqua",
                &format!("Suivre le réglage du monde : {}", unit.show(world)),
                &format!("/chop perso {word} -1"),
            ))
        } else {
            line.append(span("  (comme le monde)", "dark_gray"))
        };
        lines.push(line);
    }
    lines
}

/// `/chop stats`: the player's trees and the leaderboard.
fn stats_lines(st: &State, uuid: u128) -> Vec<Text> {
    let mut lines = Vec::new();
    match st.stats.get(&uuid) {
        Some(s) => lines.push(
            bold("🪓 Chop! ", "gold")
                .append(span("Tes arbres : ", "gray"))
                .append(bold(&s.trees.to_string(), "yellow"))
                .append(span(
                    &format!(
                        " ({} bûches, {} blocs) · le plus grand : {} blocs · chaîne record : {}",
                        s.logs, s.blocks, s.biggest, s.chain
                    ),
                    "gray",
                )),
        ),
        None => lines.push(
            bold("🪓 Chop! ", "gold").append(span("Tu n'as encore abattu aucun arbre.", "gray")),
        ),
    }
    let mut board: Vec<&Stats> = st.stats.values().filter(|s| s.trees > 0).collect();
    board.sort_by(|a, b| b.trees.cmp(&a.trees).then(b.biggest.cmp(&a.biggest)));
    if !board.is_empty() {
        lines.push(span("  Classement :", "gold"));
        for (k, s) in board.iter().take(5).enumerate() {
            let medal = ["🥇", "🥈", "🥉", " 4.", " 5."][k];
            lines.push(span(
                &format!(
                    "  {medal} {} — {} arbres, record {} blocs",
                    s.name, s.trees, s.biggest
                ),
                if k == 0 { "yellow" } else { "gray" },
            ));
        }
    }
    lines
}

/// A page of the settings menu: tabs, one line per setting with its buttons.
fn page(config: &Config, section: Section) -> Vec<Text> {
    let mut tabs = bold("🪓 Chop! ", "gold");
    for s in Section::ALL {
        let label = format!("[{}]", s.title());
        tabs = if s == section {
            tabs.append(bold(&label, "white"))
        } else {
            tabs.append(button(
                &label,
                "dark_gray",
                &format!("Page {}", s.title()),
                &format!("/chop config {}", s.word()),
            ))
        }
        .append(span(" ", "gray"));
    }
    let mut lines = vec![tabs];
    if !config.on(Key::Active) && section != Section::Rules {
        lines.push(span(
            "  Chop! est désactivé sur ce monde (page Règles).",
            "red",
        ));
    }
    for o in core::OPTS.iter().filter(|o| o.section == section) {
        let v = config.get(o.key);
        let mut line = span("  ", "gray")
            .append(hinted(o.label, "gray", o.help))
            .append(span(" : ", "dark_gray"));
        if o.unit == Unit::Switch {
            let (yes, no) = if v != 0 {
                ("green", "dark_gray")
            } else {
                ("dark_gray", "red")
            };
            line = line
                .append(button(
                    "[oui]",
                    yes,
                    &format!("{} : oui", o.label),
                    &format!("/chop set {} 1", o.word),
                ))
                .append(span(" ", "gray"))
                .append(button(
                    "[non]",
                    no,
                    &format!("{} : non", o.label),
                    &format!("/chop set {} 0", o.word),
                ));
        } else {
            let less = (v - o.step).max(o.min);
            let more = (v + o.step).min(o.max);
            let minus = if v > o.min {
                button(
                    "[-]",
                    "red",
                    &o.unit.show(less),
                    &format!("/chop set {} {less}", o.word),
                )
            } else {
                span("[-]", "dark_gray")
            };
            let plus = if v < o.max {
                button(
                    "[+]",
                    "green",
                    &o.unit.show(more),
                    &format!("/chop set {} {more}", o.word),
                )
            } else {
                span("[+]", "dark_gray")
            };
            line = line
                .append(minus)
                .append(span(" ", "gray"))
                .append(bold(&o.unit.show(v), "yellow"))
                .append(span(" ", "gray"))
                .append(plus);
        }
        lines.push(line);
    }
    lines.push(
        span("  ", "gray")
            .append(button(
                "[Tout remettre par défaut]",
                "dark_red",
                "Remet tous les réglages de Chop! par défaut",
                "/chop reset",
            ))
            .append(span("  ", "gray"))
            .append(button("[Aide]", "aqua", "Les commandes", "/chop aide")),
    );
    lines
}
