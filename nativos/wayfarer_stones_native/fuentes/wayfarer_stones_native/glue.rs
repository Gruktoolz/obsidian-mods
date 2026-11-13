// Wayfarer Stones in the game: the edges between the game's types and the rules
// (`game::wayfarer_stones`, runtime.rs). Added to the game as
// `game::wayfarer_stones_glue`; each small helper file calls it from a hook point.
//
// It has two halves, one on each side of a mod message. The server half runs
// where the world is simulated (a dedicated server, a LAN host, single
// player's server thread): the state of the open world, its files, and
// travels, which are queued and made at the end of a server tick, where
// `native_run_command` may run, as are the blocks and effects that go with them.
// The client half runs in the window: the
// screen, which draws and hears input in hooks that may only read, so what it
// asks for waits in `Ui::outbox` for `native_frame`.

use crate::game::{Game, GameLevel, LevelDimension, wayfarer_stones as rules};
use horizon_client::{
    hud::HudBatch,
    items::ItemStack,
    lang,
    mod_api::{
        NativeCommand, NativeLifecycleContext, NativeMod, NativeScreenView, NativeServerPlayer, ServerTickPhase,
        WorldPhase,
    },
    text::{self, ClickEvent, Style, Text},
};
use horizon_core::position::BlockPos;
use horizon_mods::content::{Block, BlockProperties, BlockShapes, Item, Rarity};
use horizon_world::{level::PlacementContext, registry::BlockStateId};
use std::{
    collections::{BTreeMap, HashMap},
    fs,
    io::{self, Write as _},
    path::{Path, PathBuf},
    sync::{
        Mutex,
        atomic::{AtomicU64, Ordering::Relaxed},
    },
};

/// The mod of the open world.
struct State {
    dir: PathBuf,
    reg: rules::Registry,
    /// Each player's open screen, as far as the server knows.
    sessions: HashMap<u128, rules::Session>,
    /// Screens to open at the end of the tick.
    opens: rules::OpenQueue,
    /// Travels waiting for the next tick (or their warmup).
    pending: Vec<rules::Pending>,
    /// Travels made to chunks not yet loaded: their players wait on the stone.
    settling: Vec<rules::Settling>,
    /// When each player may travel again (ms). Not saved: it is seconds long.
    cooldowns: HashMap<u128, i64>,
    /// Console commands for the end of the tick: a stone's other block,
    /// and what a player sees and hears. A player's click can't run them.
    commands: Vec<String>,
    /// Errors reading the files: the commands say so instead of writing
    /// over a damaged file.
    broken: Option<String>,
    /// The last save's error, logged once until a save succeeds.
    save_error: Option<String>,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);
static TICKS: AtomicU64 = AtomicU64::new(0);

// ---- files -----------------------------------------------------------------

fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| i64::try_from(d.as_millis()).unwrap_or(i64::MAX))
}

/// `8-4-4-4-12` hex, as `players.json` keys and the owners are written.
pub(in crate::game) fn uuid_string(u: u128) -> String {
    let h = format!("{u:032x}");
    format!("{}-{}-{}-{}-{}", &h[..8], &h[8..12], &h[12..16], &h[16..20], &h[20..])
}

/// A JSON document: its default when the file doesn't exist, an error when
/// it exists but can't be read, so a damaged file is never replaced.
fn read_json<T: serde::de::DeserializeOwned + Default>(path: &Path) -> Result<T, String> {
    match fs::read(path) {
        Ok(bytes) => serde_json::from_slice(&bytes).map_err(|e| format!("{}: {e}", path.display())),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(T::default()),
        Err(e) => Err(format!("{}: {e}", path.display())),
    }
}

/// The world's folder as the rules save into it: a file is staged as
/// `.<name>.tmp` beside it, flushed, and put in place by a rename.
struct Folder<'a>(&'a Path);

impl Folder<'_> {
    fn temp(&self, name: &str) -> PathBuf {
        self.0.join(format!(".{name}.tmp"))
    }
}

impl rules::SaveFiles for Folder<'_> {
    fn stage(&mut self, name: &str, bytes: &[u8]) -> Result<(), String> {
        let temp = self.temp(name);
        let write = || -> io::Result<()> {
            fs::create_dir_all(self.0)?;
            let mut file = fs::File::create(&temp)?;
            file.write_all(bytes)?;
            file.sync_all()
        };
        write().map_err(|e| format!("{}: {e}", temp.display()))
    }

    fn commit(&mut self, name: &str) -> Result<(), String> {
        let path = self.0.join(name);
        fs::rename(self.temp(name), &path).map_err(|e| format!("{}: {e}", path.display()))
    }
}

/// Written whole and atomically: a temporary file beside it, flushed, then
/// renamed over the old one.
fn write_json<T: serde::Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let dir = path.parent().unwrap_or(Path::new("."));
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("file");
    let mut bytes = serde_json::to_vec_pretty(value).map_err(|e| format!("{}: {e}", path.display()))?;
    bytes.push(b'\n');
    let mut folder = Folder(dir);
    rules::SaveFiles::stage(&mut folder, name, &bytes)?;
    rules::SaveFiles::commit(&mut folder, name)
}

fn open(dir: PathBuf) -> State {
    let mut broken = None;
    let config_path = dir.join("config.json");
    let config = match read_json::<Option<serde_json::Value>>(&config_path) {
        Ok(None) => {
            let c = rules::Config::default();
            if let Err(e) = write_json(&config_path, &c) {
                log::warn!("[wayfarer_stones] {e}");
            }
            c
        }
        Ok(Some(v)) => match serde_json::from_value::<rules::Config>(v.clone()) {
            Ok(c) => {
                // Settings a newer version added, so a player can find them.
                if serde_json::to_value(&c).is_ok_and(|now| now != v)
                    && let Err(e) = write_json(&config_path, &c)
                {
                    log::warn!("[wayfarer_stones] {e}");
                }
                c
            }
            Err(e) => {
                broken = Some(format!("{}: {e}", config_path.display()));
                rules::Config::default()
            }
        },
        Err(e) => {
            broken = Some(e);
            rules::Config::default()
        }
    };
    let data = read_json::<rules::Data>(&dir.join(rules::STONES_FILE)).unwrap_or_else(|e| {
        broken.get_or_insert(e);
        rules::Data::default()
    });
    let players = read_json::<rules::Players>(&dir.join(rules::PLAYERS_FILE)).unwrap_or_else(|e| {
        broken.get_or_insert(e);
        rules::Players::default()
    });
    if let Some(e) = &broken {
        log::error!("[wayfarer_stones] can't read {e}: the mod is read-only until it is fixed");
    } else {
        log::info!(
            "[wayfarer_stones] {} stone(s) in {}",
            data.stones.len(),
            dir.display()
        );
    }
    State {
        dir,
        reg: rules::Registry::new(config, data, players),
        sessions: HashMap::new(),
        opens: rules::OpenQueue::default(),
        pending: Vec::new(),
        settling: Vec::new(),
        cooldowns: HashMap::new(),
        commands: Vec::new(),
        broken,
        save_error: None,
    }
}

/// The state of the world in `dir` (`<world>/horizon-wayfarer-stones/`), opened
/// on first use and again when another world opens.
fn state(dir: PathBuf) -> std::sync::MutexGuard<'static, Option<State>> {
    let mut state = STATE.lock().unwrap_or_else(|e| e.into_inner());
    if state.as_ref().is_none_or(|s| s.dir != dir) {
        if let Some(old) = state.as_mut() {
            close(old);
        }
        *state = Some(open(dir));
    }
    state
}

/// Writes what changed. A failed write leaves it changed, so the next save
/// (once a second) tries again; the error is logged once until one succeeds.
fn save(st: &mut State) {
    if st.broken.is_some() {
        // Never write over a file that couldn't be read.
        return;
    }
    match st.reg.save(&mut Folder(&st.dir)) {
        Ok(()) => {
            if st.save_error.take().is_some() {
                log::info!("[wayfarer_stones] saved {} again", st.dir.display());
            }
        }
        Err(e) => {
            if st.save_error.as_ref() != Some(&e) {
                log::warn!("[wayfarer_stones] can't save, will retry: {e}");
            }
            st.save_error = Some(e);
        }
    }
}

/// The world's last save, as its state is dropped: what can't be written now
/// is lost, so it says so.
fn close(st: &mut State) {
    save(st);
    if st.reg.changed && st.broken.is_none() {
        log::error!(
            "[wayfarer_stones] {} closed with changes that could not be saved: {}",
            st.dir.display(),
            st.save_error.as_deref().unwrap_or("unknown error")
        );
    }
}

fn world_state(game: &Game<'_>) -> Option<std::sync::MutexGuard<'static, Option<State>>> {
    let dir = game.native_world_dir()?;
    Some(state(dir.join(rules::DIR)))
}

fn actor(p: &NativeServerPlayer) -> rules::Actor {
    rules::Actor {
        uuid: uuid_string(p.uuid),
        name: p.name.clone(),
        level: p.permission,
    }
}

// ---- declarations ----------------------------------------------------------

/// A stone's two blocks. A heavy core is their template: a plain block that
/// doesn't hide its neighbours' faces, which a model narrower than a cube needs,
/// and whose one property, `waterlogged`, gives `shapes` something to select by.
/// The lower block has the item. Textures, models, the language files, the loot
/// tables, the recipes and the villages are the companion content mod's
/// (`content/`).
pub(in crate::game) fn blocks(blocks: &mut Vec<Block>) {
    for (id, item) in [(rules::BLOCK, true), (rules::TOP, false)] {
        // Narrower than a block, as tall as one: a player stands on top.
        let shape = vec![[0.125, 0.0, 0.125, 0.875, 1.0, 0.875]];
        blocks.push(Block {
            id: id.into(),
            template: "minecraft:heavy_core".into(),
            properties: BlockProperties {
                hardness: Some(3.0),
                explosion_resistance: Some(1200.0),
                light: Some(7),
                requires_tool: Some(true),
                sounds: Some("minecraft:lodestone".into()),
                ..Default::default()
            },
            shapes: Some(BlockShapes {
                property: "waterlogged".into(),
                properties: Vec::new(),
                values: BTreeMap::from([
                    ("false".to_owned(), shape.clone()),
                    ("true".to_owned(), shape),
                ]),
                default: None,
            }),
            item,
            ..Default::default()
        });
    }
}

/// The wayfarer's scroll: paper that opens the menu from anywhere and is used up
/// by the travel.
pub(in crate::game) fn items(items: &mut Vec<Item>) {
    items.push(Item {
        id: rules::SCROLL.into(),
        template: "minecraft:paper".into(),
        max_stack_size: Some(16),
        rarity: Some(Rarity::Uncommon),
        ..Default::default()
    });
}

/// The row in Options > Mods.
pub(in crate::game) fn listed(mods: &mut Vec<NativeMod>) {
    mods.push(NativeMod {
        id: rules::ID.into(),
        title: format!("Wayfarer Stones {}", rules::VERSION),
        description: lang::get_or("wayfarer_stones.mod.description", rules::DESCRIPTION),
        keys: Vec::new(),
    });
}

/// The commands in every player's command tree.
pub(in crate::game) fn commands(commands: &mut Vec<NativeCommand>) {
    commands.push(NativeCommand::new("wayfarer"));
    commands.push(NativeCommand::new("wayfarer list"));
    commands.push(NativeCommand::new("wayfarer to <name:text>"));
    commands.push(NativeCommand::new("wayfarer name <name:text>"));
    commands.push(NativeCommand::new("wayfarer forget <name:text>"));
    commands.push(NativeCommand::op("wayfarer global <name:text>", 2));
    commands.push(NativeCommand::op("wayfarer remove <name:text>", 2));
}

// ---- text and items --------------------------------------------------------

/// A line as a component: it goes to a player's client by its key, and they
/// read it in their language.
fn line(m: &rules::Msg) -> Text {
    let args: Vec<Text> = m
        .args
        .iter()
        .map(|a| match a {
            rules::Arg::Plain(s) => Text::literal(s.clone()),
            rules::Arg::Nested(n) => line(n),
        })
        .collect();
    Text::translate(&m.key, &args)
}
fn green(m: rules::Msg) -> Text {
    line(&m).with_color(text::GREEN)
}
fn red(m: rules::Msg) -> Text {
    line(&m).with_color(text::RED)
}
fn gold(m: rules::Msg) -> Text {
    line(&m).with_color(text::GOLD)
}

/// The lower block of the stone at `pos`, either of its two blocks; none
/// when no stone stands there (a top alone, left by `/setblock`, is none).
/// In the world of the player acting.
fn lower_of(game: &Game<'_>, pos: [i32; 3]) -> Option<[i32; 3]> {
    let block = game.native_block(pos);
    if block == rules::BLOCK {
        return Some(pos);
    }
    let below = [pos[0], pos[1] - 1, pos[2]];
    (block == rules::TOP && game.native_block(below) == rules::BLOCK).then_some(below)
}

/// Whether a stone's lower block has its top on it.
fn has_upper(game: &Game<'_>, [x, y, z]: [i32; 3]) -> bool {
    game.native_block([x, y + 1, z]) == rules::TOP
}

/// Sends the player the stones they know.
fn send_list(
    game: &mut Game<'_>,
    st: &State,
    p: &NativeServerPlayer,
    here: Option<u64>,
    open: bool,
    notice: Option<rules::Msg>,
) -> Result<(), String> {
    let entries = rules::entries(&st.reg, &actor(p), &p.dimension, p.position, here);
    let data = rules::encode_list(rules::ListMessage {
        open,
        entries,
        total: 0,
        notice,
        scroll: st.reg.needs_scroll(here),
        cooldown_ms: st
            .cooldowns
            .get(&p.uuid)
            .map_or(0, |until| (until - now_ms()).max(0)),
    });
    game.native_net_send_to_player(p.uuid, rules::CHANNEL_LIST, &data)
}

/// Queues a travel, after the config's warmup.
fn queue_travel(game: &mut Game<'_>, st: &mut State, p: &NativeServerPlayer, to: u64, scroll: bool) {
    let Some(w) = st.reg.get(to) else {
        game.native_tell(p.uuid, red(rules::msg("wayfarer_stones.travel.gone", &[])));
        return;
    };
    let left = st
        .cooldowns
        .get(&p.uuid)
        .map_or(0, |until| rules::seconds_left(*until, now_ms()));
    if left > 0 {
        game.native_tell(p.uuid, red(rules::msg("wayfarer_stones.travel.cooldown", &[&left])));
        return;
    }
    let cost = st.reg.cost((p.dimension.as_str(), p.position), w);
    let name = w.name.clone();
    st.pending.retain(|t| t.uuid != p.uuid);
    let warmup = i64::from(st.reg.config.warmup_seconds) * 1000;
    st.pending.push(rules::Pending {
        uuid: p.uuid,
        name: p.name.clone(),
        to,
        cost,
        scroll,
        from: (p.dimension.clone(), p.position),
        due: now_ms() + warmup,
    });
    if warmup > 0 {
        game.native_tell(
            p.uuid,
            gold(rules::msg(
                "wayfarer_stones.travel.warmup",
                &[&name, &st.reg.config.warmup_seconds],
            )),
        );
    }
}

// ---- hook points -----------------------------------------------------------

/// A player placed a block: the stone registers itself.
pub(in crate::game) fn placed(game: &mut Game<'_>, stack: &ItemStack, pos: [i32; 3]) {
    if !game.native_on(rules::ID) || &*stack.item != rules::BLOCK {
        return;
    }
    let Some(who) = game.native_acting_player() else {
        return;
    };
    let Some(mut guard) = world_state(game) else {
        return;
    };
    let st = guard.as_mut().expect("opened");
    let w = st.reg.place(&who.dimension, pos, &actor(&who), now_ms());
    st.commands.push(rules::upper_block_command(&who.dimension, pos));
    game.native_tell(who.uuid, green(rules::msg("wayfarer_stones.placed", &[&w.name])));
}

/// A block is about to be placed: a stone needs room for its upper block.
pub(in crate::game) fn state_for_placement(
    level: &GameLevel<'_>,
    state: BlockStateId,
    context: &PlacementContext,
    answer: &mut Option<Option<BlockStateId>>,
) {
    if answer.is_some() || level.native_state_block(state) != Some(rules::BLOCK) {
        return;
    }
    let at = context.pos;
    let above = level.world.block(at.x, at.y + 1, at.z);
    if !level.facts.get(above).replaceable {
        *answer = Some(None);
    }
}

/// A block changed, whoever changed it (a player, `/fill`, `/setblock`): a
/// stone doesn't outlive its lower block, and its two blocks go together.
/// Runs for every change in the world, so it asks the block's name first.
pub(in crate::game) fn block_changed(
    level: &GameLevel<'_>,
    dimension: LevelDimension,
    pos: BlockPos,
    old: BlockStateId,
    new: BlockStateId,
) {
    let was = &*level.facts.get(old).name;
    if (was != rules::BLOCK && was != rules::TOP) || &*level.facts.get(new).name == was {
        return;
    }
    let dimension = match dimension {
        LevelDimension::Overworld => "minecraft:overworld",
        LevelDimension::Nether => "minecraft:the_nether",
        LevelDimension::End => "minecraft:the_end",
    };
    let mut state = STATE.lock().unwrap_or_else(|e| e.into_inner());
    let Some(st) = state.as_mut() else {
        return;
    };
    let named = |y: i32| &*level.facts.get(level.world.block(pos.x, y, pos.z)).name;
    if was == rules::BLOCK {
        let at = [pos.x, pos.y, pos.z];
        if let Some(id) = st.reg.at(dimension, at).map(|w| w.id) {
            st.reg.remove(id);
            st.pending.retain(|t| t.to != id);
            st.settling.retain(|t| t.to != id);
        }
        if named(pos.y + 1) == rules::TOP {
            let above = [pos.x, pos.y + 1, pos.z];
            st.commands.push(rules::remove_block_command(dimension, above));
        }
    } else if named(pos.y - 1) == rules::BLOCK {
        let below = [pos.x, pos.y - 1, pos.z];
        st.commands.push(rules::remove_block_command(dimension, below));
    }
}

/// A piston won't part a stone's two blocks, nor crush them.
pub(in crate::game) fn piston_move(
    level: &GameLevel<'_>,
    push: &[BlockPos],
    destroy: &[BlockPos],
    canceled: &mut bool,
) {
    let ours = |p: &BlockPos| {
        let state = level.world.block(p.x, p.y, p.z);
        let name = &*level.facts.get(state).name;
        name == rules::BLOCK || name == rules::TOP
    };
    if push.iter().chain(destroy).any(ours) {
        *canceled = true;
    }
}

/// A right click with an item on a block, before the placement: refuses a
/// stone past the owner's limit.
pub(in crate::game) fn use_item_on(game: &mut Game<'_>, stack: &ItemStack, claimed: &mut bool) {
    if *claimed || !game.native_on(rules::ID) || &*stack.item != rules::BLOCK {
        return;
    }
    let Some(who) = game.native_acting_player() else {
        return;
    };
    let Some(mut guard) = world_state(game) else {
        return;
    };
    let st = guard.as_mut().expect("opened");
    if let Err(e) = st.reg.may_place(&actor(&who)) {
        *claimed = true;
        game.native_action_bar(who.uuid, red(e));
    }
}

/// A player breaks a block: a stone goes, or stays for its owner.
pub(in crate::game) fn destroyed(game: &mut Game<'_>, pos: [i32; 3], canceled: &mut bool) {
    if *canceled || !game.native_on(rules::ID) {
        return;
    }
    let Some(who) = game.native_acting_player() else {
        return;
    };
    let Some(mut guard) = world_state(game) else {
        return;
    };
    let st = guard.as_mut().expect("opened");
    let Some(lower) = lower_of(game, pos) else {
        return;
    };
    match st.reg.broken(&who.dimension, lower, &actor(&who)) {
        // `block_changed` takes the other block.
        Ok(None) => {}
        Ok(Some(w)) => {
            st.pending.retain(|t| t.to != w.id);
            game.native_tell(who.uuid, gold(rules::msg("wayfarer_stones.removed", &[&w.name])));
        }
        Err(e) => {
            *canceled = true;
            game.native_action_bar(who.uuid, red(e));
        }
    }
}

/// A right click on a block: a stone activates and opens the screen. One a
/// village generated joins the registry here; a one-block stone from before 0.4
/// grows its upper block.
pub(in crate::game) fn right_click(game: &mut Game<'_>, pos: [i32; 3], canceled: &mut bool) {
    if *canceled || !game.native_on(rules::ID) {
        return;
    }
    let Some(who) = game.native_acting_player() else {
        return;
    };
    let Some(mut guard) = world_state(game) else {
        return;
    };
    let st = guard.as_mut().expect("opened");
    let Some(lower) = lower_of(game, pos) else {
        return;
    };
    *canceled = true;
    let w = match st.reg.at(&who.dimension, lower).cloned() {
        Some(w) => w,
        None => st.reg.discover(&who.dimension, lower, now_ms()),
    };
    let [x, y, z] = lower;
    if !has_upper(game, lower) && game.native_block([x, y + 1, z]) == "minecraft:air" {
        st.commands.push(rules::upper_block_command(&who.dimension, lower));
    }
    if st.reg.activate(&uuid_string(who.uuid), w.id) {
        st.commands.extend(rules::activation_commands(&w));
        game.native_tell(who.uuid, green(rules::msg("wayfarer_stones.activated", &[&w.name])));
    }
    st.opens.request(rules::OpenRequest {
        uuid: who.uuid,
        here: Some(w.id),
    });
}

/// A player uses an item: a wayfarer's scroll opens the screen.
pub(in crate::game) fn use_item(game: &mut Game<'_>, stack: &ItemStack, claimed: &mut bool) {
    if *claimed || !game.native_on(rules::ID) || &*stack.item != rules::SCROLL {
        return;
    }
    let Some(who) = game.native_acting_player() else {
        return;
    };
    let Some(mut guard) = world_state(game) else {
        return;
    };
    let st = guard.as_mut().expect("opened");
    if !st.reg.config.wayfarers_scroll {
        return;
    }
    *claimed = true;
    st.opens.request(rules::OpenRequest {
        uuid: who.uuid,
        here: None,
    });
}

/// A player's command on the server: ours, when it is one.
pub(in crate::game) fn perform(game: &mut Game<'_>, player: &NativeServerPlayer, line: &str, ran: &mut bool) {
    let word = line.split(' ').next().unwrap_or_default();
    if *ran || !game.native_on(rules::ID) || !rules::COMMANDS.contains(&word) {
        return;
    }
    *ran = true;
    let Some(mut guard) = world_state(game) else {
        game.native_feedback(red(rules::msg("wayfarer_stones.error.no_world", &[])));
        return;
    };
    let st = guard.as_mut().expect("opened");
    if let Some(e) = &st.broken {
        game.native_feedback(red(rules::msg("wayfarer_stones.error.read_only", &[e])));
    }
    let out = st.reg.command(&actor(player), &player.dimension, player.position, line);
    for o in out {
        match o {
            rules::Out::Reply(s) => game.native_feedback(green(s)),
            rules::Out::Error(s) => game.native_feedback(red(s)),
            rules::Out::Open => st.opens.request(rules::OpenRequest {
                uuid: player.uuid,
                here: None,
            }),
            rules::Out::Travel { id, scroll } => queue_travel(game, st, player, id, scroll),
        }
    }
}

/// A request from a player's client, checked here as any player's input.
pub(in crate::game) fn net_from_client(game: &mut Game<'_>, player: u128, channel: &str, data: &[u8]) {
    if channel != rules::CHANNEL_ACT || !game.native_on(rules::ID) {
        return;
    }
    let Some(act) = rules::decode_act(data) else {
        log::warn!("[wayfarer_stones] an unreadable request from a client");
        return;
    };
    let Some(p) = game.native_server_players().into_iter().find(|p| p.uuid == player) else {
        return;
    };
    let Some(mut guard) = world_state(game) else {
        return;
    };
    let st = guard.as_mut().expect("opened");
    let who = actor(&p);
    match act {
        rules::ClientAct::Travel(id) => {
            // Only from a screen this server opened, once.
            let Some(session) = st.sessions.remove(&player) else {
                return;
            };
            if session.scroll && !st.reg.config.wayfarers_scroll {
                let why = rules::msg("wayfarer_stones.error.stand_at_stone", &[]);
                game.native_tell(player, red(why));
            } else if st.reg.knows(&who.uuid, id) && session.here != Some(id) {
                queue_travel(game, st, &p, id, session.scroll);
            } else {
                game.native_tell(player, red(rules::msg("wayfarer_stones.travel.refused", &[])));
            }
        }
        rules::ClientAct::Rename { id, name } => {
            let notice = st.reg.rename(id, &name, &who).err();
            let here = st.sessions.get(&player).and_then(|s| s.here);
            if let Err(e) = send_list(game, st, &p, here, false, notice) {
                log::warn!("[wayfarer_stones] {e}");
            }
        }
    }
}

// ---- the screen, in the window ---------------------------------------------

/// The screen of this window, while the server's last list is open.
static UI: Mutex<Option<rules::Ui>> = Mutex::new(None);

fn ui() -> std::sync::MutexGuard<'static, Option<rules::Ui>> {
    UI.lock().unwrap_or_else(|e| e.into_inner())
}

fn ours(game: &Game<'_>, screen: &NativeScreenView<'_>) -> bool {
    screen.id == "mod" && game.native_mod_screen() == Some(rules::ID)
}

/// The server's list: opens the screen, or refreshes the one that is open.
pub(in crate::game) fn net_from_server(game: &mut Game<'_>, channel: &str, data: &[u8]) {
    if channel != rules::CHANNEL_LIST {
        return;
    }
    let Some(message) = rules::decode_list(data) else {
        log::warn!("[wayfarer_stones] an unreadable list from the server");
        return;
    };
    if message.open {
        if game.native_mod_screen_open(rules::ID) {
            let ready_at = now_ms() + message.cooldown_ms;
            let mut open = rules::Ui::new(message);
            open.ready_at = ready_at;
            *ui() = Some(open);
        } else {
            log::warn!("[wayfarer_stones] the screen can't open over the one that is open");
        }
    } else if game.native_mod_screen() == Some(rules::ID)
        && let Some(open) = ui().as_mut()
    {
        open.ready_at = now_ms() + message.cooldown_ms;
        open.refresh(message);
    }
}

/// Once a frame: what the screen's read-only hooks asked for.
pub(in crate::game) fn frame(game: &mut Game<'_>) {
    let actions = match ui().as_mut() {
        Some(open) if !open.outbox.is_empty() => std::mem::take(&mut open.outbox),
        _ => return,
    };
    for action in actions {
        match action {
            rules::UiAction::Send(act) => {
                let data = rules::encode_act(&act);
                if let Err(e) = game.native_net_send_to_server(rules::CHANNEL_ACT, &data)
                    && let Some(open) = ui().as_mut()
                {
                    open.notice = Some(rules::msg("wayfarer_stones.screen.send_failed", &[&e]));
                }
            }
            rules::UiAction::Close => game.native_mod_screen_close(rules::ID),
        }
    }
}

/// Draws the screen over the world.
pub(in crate::game) fn render(game: &Game<'_>, batch: &mut HudBatch, screen: &NativeScreenView<'_>) {
    if !ours(game, screen) {
        return;
    }
    let caret = now_ms() / 500 % 2 == 0;
    let draws = {
        let mut guard = ui();
        let Some(open) = guard.as_mut() else {
            return;
        };
        open.window = [batch.width, batch.height];
        open.cooldown = rules::seconds_left(open.ready_at, now_ms());
        open.render(
            screen.mouse,
            caret,
            &|text| game.native_text_width(text),
            &|m| line(m).plain(),
        )
    };
    for draw in draws {
        match draw {
            rules::Draw::Fill(rect, argb) => game.native_fill(batch, rect, argb),
            rules::Draw::Sprite(name, src, dst) => {
                game.native_sprite(batch, name, src, dst);
            }
            rules::Draw::Text(text, at, rgb) => game.native_text(batch, &text, at, rgb, false),
            rules::Draw::Item(item, at, size) => {
                game.native_item(batch, item, at, size);
            }
        }
    }
}

pub(in crate::game) fn mouse_pressed(game: &Game<'_>, screen: &NativeScreenView<'_>, button: i32, canceled: &mut bool) {
    if ours(game, screen)
        && let Some(open) = ui().as_mut()
    {
        open.click(screen.mouse, button);
        *canceled = true;
    }
}

pub(in crate::game) fn mouse_scrolled(game: &Game<'_>, screen: &NativeScreenView<'_>, scroll: [f64; 2], canceled: &mut bool) {
    if ours(game, screen)
        && let Some(open) = ui().as_mut()
    {
        open.wheel(scroll[1]);
        *canceled = true;
    }
}

pub(in crate::game) fn key_pressed(game: &Game<'_>, screen: &NativeScreenView<'_>, key: &str, canceled: &mut bool) {
    if ours(game, screen)
        && let Some(open) = ui().as_mut()
        && open.key(key)
    {
        *canceled = true;
    }
}

pub(in crate::game) fn char_typed(game: &Game<'_>, screen: &NativeScreenView<'_>, c: char, canceled: &mut bool) {
    if ours(game, screen)
        && let Some(open) = ui().as_mut()
    {
        open.typed(c);
        *canceled = true;
    }
}

// ---- the server's tick -----------------------------------------------------

/// A due travel, as console commands to run with the state's lock let go.
struct Travel {
    uuid: u128,
    /// Fails when the player lacks the levels; none for a free travel.
    check: Option<String>,
    /// Takes the wayfarer's scroll; fails when the player has none.
    scroll: Option<String>,
    refund: Option<String>,
    /// What the player leaves behind.
    departure: [String; 2],
    /// Where they go, as far as can be told now.
    landing: rules::Landing,
    /// The stone's top, where they wait when nobody can tell yet.
    top: [f64; 3],
    dimension: String,
    settling: rules::Settling,
    cost: u32,
    name: String,
}

/// A block as a player landing there sees it; none while it isn't loaded.
fn cell(
    game: &Game<'_>,
    context: &NativeLifecycleContext,
    dimension: &str,
    pos: [i32; 3],
) -> Option<rules::Cell> {
    let state = game.native_server_block(context, dimension, pos).ok()?;
    Some(match state.name.as_str() {
        "minecraft:lava" | "minecraft:fire" | "minecraft:soul_fire" => rules::Cell::Hazard,
        _ if game
            .native_server_block_in_tag(&state, "minecraft:replaceable")
            .unwrap_or(false) =>
        {
            rules::Cell::Passable
        }
        _ => rules::Cell::Solid,
    })
}

/// The end of each server tick: due travels are made, and the files that
/// changed are written once a second.
pub(in crate::game) fn tick(
    game: &mut Game<'_>,
    context: &NativeLifecycleContext,
    phase: ServerTickPhase,
) {
    if !matches!(phase, ServerTickPhase::ServerTickEnd { .. }) {
        return;
    }
    let n = TICKS.fetch_add(1, Relaxed);
    let Some(mut guard) = world_state(game) else {
        return;
    };
    let st = guard.as_mut().expect("opened");
    let second = n % 20 == 0;
    let idle = st.pending.is_empty() && st.opens.is_empty() && st.settling.is_empty();
    if idle && st.commands.is_empty() && !second {
        return;
    }
    let now = now_ms();
    st.cooldowns.retain(|_, until| *until > now);
    let players = if idle {
        Vec::new()
    } else {
        game.native_server_players()
    };
    // Screens: here, where the server's connections are back in place.
    for r in st.opens.take(n) {
        let Some(p) = players.iter().find(|p| p.uuid == r.uuid) else {
            continue;
        };
        let here = r.here.or_else(|| {
            rules::here_of(&st.reg, &uuid_string(p.uuid), &p.dimension, p.position)
        });
        let scroll = st.reg.needs_scroll(here);
        st.sessions.insert(p.uuid, rules::Session { scroll, here });
        if let Err(e) = send_list(game, st, p, here, true, None) {
            log::warn!("[wayfarer_stones] {e}");
            game.native_tell(p.uuid, red(rules::msg("wayfarer_stones.screen.cant_open", &[])));
        }
    }
    // Players waiting on a stone for its chunks: on to the safe place, or back.
    let mut moves: Vec<String> = Vec::new();
    let mut undo: Vec<rules::Settling> = Vec::new();
    st.settling.retain(|t| {
        let Some(w) = st.reg.get(t.to) else {
            return false;
        };
        let top = rules::landing(w);
        let Some(p) = players.iter().find(|p| p.uuid == t.uuid) else {
            return false; // left
        };
        // Gone on by themselves: it is no business of ours any more.
        if p.dimension != w.dimension || rules::distance(p.position, top) > 3.0 {
            return false;
        }
        match rules::landing_spot(w, &|pos| cell(game, context, &w.dimension, pos)) {
            rules::Landing::Unknown => n < t.until,
            rules::Landing::At(spot) => {
                if spot != top {
                    moves.push(rules::travel_command(&t.name, &w.dimension, spot));
                }
                false
            }
            rules::Landing::Blocked => {
                undo.push(t.clone());
                false
            }
        }
    });
    // Travels: each is resolved to the commands to run, then the lock is let
    // go before any command runs (a teleport may fire other hook points).
    let mut go: Vec<Travel> = Vec::new();
    let mut tell: Vec<(u128, Text)> = Vec::new();
    st.pending.retain(|t| {
        let Some(p) = players.iter().find(|p| p.uuid == t.uuid) else {
            return false; // left
        };
        if rules::moved(&t.from, (p.dimension.as_str(), p.position)) {
            tell.push((t.uuid, red(rules::msg("wayfarer_stones.travel.moved", &[]))));
            return false;
        }
        if t.due > now {
            return true;
        }
        let Some(w) = st.reg.get(t.to) else {
            tell.push((t.uuid, red(rules::msg("wayfarer_stones.travel.gone", &[]))));
            return false;
        };
        go.push(Travel {
            uuid: t.uuid,
            check: (t.cost > 0).then(|| rules::level_check_command(&t.name, t.cost)),
            scroll: t.scroll.then(|| rules::consume_scroll_command(&t.name)),
            refund: t.scroll.then(|| rules::refund_scroll_command(&t.name)),
            departure: rules::departure_commands(&p.dimension, p.position),
            landing: rules::landing_spot(w, &|pos| cell(game, context, &w.dimension, pos)),
            top: rules::landing(w),
            dimension: w.dimension.clone(),
            settling: rules::Settling {
                uuid: t.uuid,
                name: t.name.clone(),
                to: t.to,
                from: (p.dimension.clone(), p.position),
                cost: t.cost,
                scroll: t.scroll,
                until: n + rules::SETTLE_TICKS,
            },
            cost: t.cost,
            name: w.name.clone(),
        });
        false
    });
    if second {
        save(st);
    }
    let commands = std::mem::take(&mut st.commands);
    drop(guard);
    for (uuid, line) in tell {
        game.native_tell(uuid, line);
    }
    for command in commands.iter().chain(&moves) {
        if !game.native_run_command(command) {
            log::warn!("[wayfarer_stones] failed: {command}");
        }
    }
    for t in undo {
        let back = rules::travel_command(&t.name, &t.from.0, t.from.1);
        game.native_run_command(&back);
        if t.scroll {
            game.native_run_command(&rules::refund_scroll_command(&t.name));
        }
        if t.cost > 0 {
            game.native_give_levels(t.uuid, i32::try_from(t.cost).unwrap_or(i32::MAX));
        }
        game.native_tell(t.uuid, red(rules::msg("wayfarer_stones.travel.blocked", &[])));
    }
    let mut travelled: Vec<u128> = Vec::new();
    let mut waits: Vec<rules::Settling> = Vec::new();
    for t in go {
        // In this order: nothing is charged for a travel that didn't happen.
        let (spot, wait) = match t.landing {
            rules::Landing::At(spot) => (spot, false),
            rules::Landing::Unknown => (t.top, true),
            rules::Landing::Blocked => {
                game.native_tell(t.uuid, red(rules::msg("wayfarer_stones.travel.blocked", &[])));
                continue;
            }
        };
        if let Some(check) = &t.check
            && !game.native_run_command(check)
        {
            let why = rules::msg("wayfarer_stones.travel.levels", &[&t.cost, &t.name]);
            game.native_tell(t.uuid, red(why));
            continue;
        }
        if let Some(scroll) = &t.scroll
            && !game.native_run_command(scroll)
        {
            game.native_tell(t.uuid, red(rules::msg("wayfarer_stones.travel.no_scroll", &[])));
            continue;
        }
        // Before the player goes: `tp` moves them out of earshot of it.
        for command in &t.departure {
            game.native_run_command(command);
        }
        if !game.native_run_command(&rules::travel_command(&t.settling.name, &t.dimension, spot)) {
            if let Some(refund) = &t.refund {
                game.native_run_command(refund);
            }
            game.native_tell(t.uuid, red(rules::msg("wayfarer_stones.travel.failed", &[&t.name])));
            continue;
        }
        for command in &rules::arrival_commands(&t.dimension, spot) {
            game.native_run_command(command);
        }
        travelled.push(t.uuid);
        if wait {
            waits.push(t.settling);
        }
        if t.cost > 0 {
            game.native_give_levels(t.uuid, -i32::try_from(t.cost).unwrap_or(i32::MAX));
        }
        game.native_tell(t.uuid, green(rules::msg("wayfarer_stones.travel.done", &[&t.name])));
    }
    if travelled.is_empty() {
        return;
    }
    let mut state = STATE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(st) = state.as_mut() {
        let until = now_ms() + i64::from(st.reg.config.cooldown_seconds) * 1000;
        if st.reg.config.cooldown_seconds > 0 {
            st.cooldowns.extend(travelled.iter().map(|uuid| (*uuid, until)));
        }
        st.settling.extend(waits);
    }
}

/// The world closing: the files are written and the state let go.
/// A player joins. The assets half, if it's there, reads the tag to know this
/// half is too (content/data/wayfarer_stones/function/check.mcfunction). Without the
/// assets half, which brings the block's loot table, they hear what's missing and
/// where to get it, in plain text, as the language files are gone too.
pub(in crate::game) fn logged_in(game: &mut Game<'_>, player: u128, name: &str) {
    if !game.native_on(rules::ID) {
        return;
    }
    let registry = &game.pack.assets.registry;
    let assets = registry
        .default_state(rules::BLOCK)
        .is_some_and(|state| world_items!(game).block_loot_table(registry, state).is_some());
    if assets {
        game.native_run_command(&format!("tag {name} add wayfarer_stones.native"));
        return;
    }
    const ASSETS_PAGE: &str = "https://playhorizon.dev/mods/wayfarer_stones";
    let link = Style {
        color: Some(text::AQUA),
        underlined: Some(true),
        click: Some(ClickEvent::OpenUrl(ASSETS_PAGE.to_owned())),
        ..Style::default()
    };
    for line in [
        "Wayfarer Stones: the assets mod is missing. Install wayfarer_stones too: ",
        "Wayfarer Stones: не установлен мод с ассетами. Поставьте ещё wayfarer_stones: ",
    ] {
        let line = Text::literal(line).append(Text::styled(ASSETS_PAGE, link.clone()));
        game.native_tell(player, line);
    }
}

pub(in crate::game) fn lifecycle(phase: WorldPhase) {
    if !matches!(phase, WorldPhase::WorldCloseFinal | WorldPhase::WorldCloseFailed) {
        return;
    }
    let mut state = STATE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(st) = state.as_mut() {
        close(st);
    }
    *state = None;
}
