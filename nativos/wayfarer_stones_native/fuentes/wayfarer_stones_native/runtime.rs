// Wayfarer Stones for Horizon: the rules. This file is both the mod's Cargo library
// (its unit tests, `cargo test`) and the module `crate::game::wayfarer_stones` the
// bundle adds to the game, so it names no game type: glue.rs turns these
// rules into hook points, menus and commands.
//
// A stone is a block a player places. The placer owns it and knows it;
// any other player learns it by right-clicking it once ("activating" it).
// Right-clicking a known stone, or using a wayfarer's scroll, makes the server
// send the player the stones they know, and their client opens the mod's
// own screen (`Ui`); a click travels there, for experience levels when the
// config asks for them. The client only asks: the server checks every request.
//
// A stone is two blocks tall, `wayfarer_stones:wayfarer_stone` and a top on it that is
// a block of its own; the registry knows it by its lower block. Villages generate some: those have no owner and
// join the registry when a player first right-clicks them.
//
// Every line a player reads is a `Msg`, a key of the content mod's language
// files, so each player reads it in their own language.
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Display};

/// The native part's id: its registry entry, and what turns it on. The blocks and items are in the
/// namespace of the content mod, whose id it is; the game joins them to it there.
pub const ID: &str = "wayfarer_stones_native";
pub const VERSION: &str = "0.6.3";
pub const DESCRIPTION: &str = "Wayfarer Stones: place one, right-click to activate it, and travel between the ones you know.";
/// The block and the item the mod declares, in its namespace.
pub const BLOCK: &str = "wayfarer_stones:wayfarer_stone";
/// A stone's upper block. Its own block, not a state of `BLOCK`: a village's
/// template rotates a state through the table of its block's template, which
/// knows no property the mod declares, so a `half` would come out `lower`.
pub const TOP: &str = "wayfarer_stones:wayfarer_stone_top";
pub const SCROLL: &str = "wayfarer_stones:wayfarers_scroll";
/// The folder of the mod's files in a world's folder.
pub const DIR: &str = "horizon-wayfarer-stones";
/// The commands the mod registers (their first word).
pub const COMMANDS: [&str; 1] = ["wayfarer"];
/// Mod messages: the server's list of stones, and the client's requests.
pub const CHANNEL_LIST: &str = "wayfarer_stones:list";
pub const CHANNEL_ACT: &str = "wayfarer_stones:act";
/// The most a mod message carries.
pub const MAX_MESSAGE: usize = 30_000;
/// The longest name, in characters.
pub const MAX_NAME: usize = 32;
/// How close a player stands to a stone they rename or forget by position.
pub const NEAR: f64 = 4.0;
/// How close to a stone's middle a player counts as standing at it.
pub const AT_STONE: f64 = 3.0;

/// A line for a player: a language file's key and what goes in its `%s`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Msg {
    pub key: String,
    pub args: Vec<Arg>,
}

/// What goes in a `%s`: text as it is, or another line in the same language.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Arg {
    Plain(String),
    Nested(Msg),
}

/// A line with plain arguments.
pub fn msg(key: &str, args: &[&dyn Display]) -> Msg {
    Msg {
        key: key.to_owned(),
        args: args.iter().map(|a| Arg::Plain(a.to_string())).collect(),
    }
}

impl Msg {
    /// With one more argument, itself a line.
    pub fn and(mut self, nested: Msg) -> Self {
        self.args.push(Arg::Nested(nested));
        self
    }
}

/// `config.json`, written with its defaults the first time.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Experience levels a travel costs (0: free).
    pub xp_levels_cost: u32,
    /// The cost is multiplied by this across dimensions.
    pub cross_dimension_multiplier: u32,
    /// A travel shorter than this, in the same dimension, is free (0: never).
    pub free_within_blocks: u32,
    /// One more level for each this many blocks, in the same dimension (0: never).
    pub blocks_per_level: u32,
    /// The most a travel costs, whatever the distance (0: no limit).
    pub max_levels_cost: u32,
    /// Seconds after a travel before the player may travel again (0: none).
    pub cooldown_seconds: u32,
    /// Seconds a player stands still before the travel (0: at once).
    pub warmup_seconds: u32,
    /// Only the owner (or an operator of level 2) breaks a stone.
    pub owner_only_break: bool,
    /// The wayfarer's scroll item opens the screen from anywhere, and a travel that
    /// doesn't start at a stone uses one up.
    pub wayfarers_scroll: bool,
    /// Travels start from anywhere, without a stone or a wayfarer's scroll.
    pub travel_anywhere: bool,
    /// Stones one player may own (0: no limit).
    pub max_stones_per_player: u32,
    /// Names a new stone draws from (a number is added when all are taken).
    pub name_pool: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            xp_levels_cost: 1,
            cross_dimension_multiplier: 3,
            free_within_blocks: 0,
            blocks_per_level: 500,
            max_levels_cost: 5,
            cooldown_seconds: 10,
            warmup_seconds: 0,
            owner_only_break: true,
            wayfarers_scroll: true,
            travel_anywhere: false,
            max_stones_per_player: 0,
            name_pool: default_name_pool(),
        }
    }
}

pub fn default_name_pool() -> Vec<String> {
    [
        "Ashenvale",
        "Brightwater",
        "Cinderfall",
        "Duskmire",
        "Eldergrove",
        "Frosthollow",
        "Gloomreach",
        "Hearthstead",
        "Ironcrest",
        "Juniper Rest",
        "Kingsroad",
        "Lonely Spire",
        "Mistwood",
        "Northwatch",
        "Oakenshield",
        "Pinecliff",
        "Quietmarsh",
        "Ravenmoor",
        "Silverbrook",
        "Thornfield",
        "Umberfell",
        "Verdant Gate",
        "Westmarch",
        "Yarrowdell",
        "Amber Crossing",
        "Blackwell",
        "Coldspring",
        "Driftstone",
        "Emberhold",
        "Fernridge",
        "Goldleaf",
        "Highmeadow",
        "Ivory Steps",
        "Larkspur",
        "Moonstone",
        "Nightshade Hollow",
        "Old Harbour",
        "Pebblebrook",
        "Redfern",
        "Stormwatch",
    ]
    .iter()
    .map(|s| (*s).to_owned())
    .collect()
}

/// A placed stone.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stone {
    pub id: u64,
    pub name: String,
    /// `minecraft:overworld`, `minecraft:the_nether`, `minecraft:the_end`.
    pub dimension: String,
    pub pos: [i32; 3],
    /// The placer's UUID, as text.
    pub owner: String,
    pub owner_name: String,
    /// Known to everyone (an operator made it so).
    pub global: bool,
    /// Milliseconds since the epoch.
    pub created: i64,
}

/// `stones.json`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Data {
    pub next_id: u64,
    pub stones: Vec<Stone>,
}

/// `players.json`: the stones each player (UUID as text) knows.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Players {
    pub known: BTreeMap<String, Vec<u64>>,
}

/// Who acts: a player's UUID as text, their name and operator level.
#[derive(Clone, Debug, PartialEq)]
pub struct Actor {
    pub uuid: String,
    pub name: String,
    pub level: u8,
}

/// Whom a stone a village generated belongs to: nobody.
pub const NO_OWNER: &str = "";

impl Actor {
    /// Operators of level 2 manage everyone's stones.
    pub fn manages(&self, w: &Stone) -> bool {
        self.level >= 2 || w.owner == self.uuid
    }
}

/// What a command answers.
#[derive(Clone, Debug, PartialEq)]
pub enum Out {
    Reply(Msg),
    Error(Msg),
    /// Open the screen for the player.
    Open,
    /// Travel to this stone (the glue queues it), using up a wayfarer's scroll
    /// when `scroll` says so.
    Travel { id: u64, scroll: bool },
}

/// The world's stones, who knows them, and the config.
pub struct Registry {
    pub config: Config,
    pub data: Data,
    pub players: Players,
    /// Something to write at the next save.
    pub changed: bool,
}

impl Registry {
    pub fn new(config: Config, data: Data, players: Players) -> Self {
        Self {
            config,
            data,
            players,
            changed: false,
        }
    }

    pub fn get(&self, id: u64) -> Option<&Stone> {
        self.data.stones.iter().find(|w| w.id == id)
    }

    fn get_mut(&mut self, id: u64) -> Option<&mut Stone> {
        self.data.stones.iter_mut().find(|w| w.id == id)
    }

    /// The stone at a block position.
    pub fn at(&self, dimension: &str, pos: [i32; 3]) -> Option<&Stone> {
        self.data
            .stones
            .iter()
            .find(|w| w.dimension == dimension && w.pos == pos)
    }

    /// A stone by name, case-insensitively; an exact match first.
    pub fn by_name(&self, name: &str) -> Option<&Stone> {
        let name = name.trim();
        self.data
            .stones
            .iter()
            .find(|w| w.name == name)
            .or_else(|| {
                self.data
                    .stones
                    .iter()
                    .find(|w| w.name.eq_ignore_ascii_case(name))
            })
    }

    pub fn owned_by(&self, uuid: &str) -> usize {
        self.data
            .stones
            .iter()
            .filter(|w| w.owner == uuid)
            .count()
    }

    /// Whether `actor` may place one more stone.
    pub fn may_place(&self, actor: &Actor) -> Result<(), Msg> {
        let max = self.config.max_stones_per_player as usize;
        if max > 0 && actor.level < 2 && self.owned_by(&actor.uuid) >= max {
            return Err(msg("wayfarer_stones.error.limit", &[&max]));
        }
        Ok(())
    }

    /// A stone block was placed: registers it, named from the pool, known
    /// to its owner. A second one at the same position replaces the first.
    pub fn place(&mut self, dimension: &str, pos: [i32; 3], actor: &Actor, now: i64) -> Stone {
        if let Some(old) = self.at(dimension, pos).map(|w| w.id) {
            self.remove(old);
        }
        let id = self.data.next_id.max(1);
        self.data.next_id = id + 1;
        let name = self.fresh_name(seed(dimension, pos));
        let w = Stone {
            id,
            name,
            dimension: dimension.to_owned(),
            pos,
            owner: actor.uuid.clone(),
            owner_name: actor.name.clone(),
            global: false,
            created: now,
        };
        self.data.stones.push(w.clone());
        self.activate(&actor.uuid, id);
        self.changed = true;
        w
    }

    /// A stone a village generated, first met at `pos`: it joins the
    /// registry with a name from the pool and no owner.
    pub fn discover(&mut self, dimension: &str, pos: [i32; 3], now: i64) -> Stone {
        let id = self.data.next_id.max(1);
        self.data.next_id = id + 1;
        let w = Stone {
            id,
            name: self.fresh_name(seed(dimension, pos)),
            dimension: dimension.to_owned(),
            pos,
            owner: NO_OWNER.to_owned(),
            owner_name: String::new(),
            global: false,
            created: now,
        };
        self.data.stones.push(w.clone());
        self.changed = true;
        w
    }

    /// A name from the pool no stone has, picked by `seed`; a number is
    /// added when all are taken.
    fn fresh_name(&self, seed: u64) -> String {
        let pool = if self.config.name_pool.is_empty() {
            default_name_pool()
        } else {
            self.config.name_pool.clone()
        };
        let taken = |n: &str| self.data.stones.iter().any(|w| w.name == n);
        let start = (seed % pool.len() as u64) as usize;
        for i in 0..pool.len() {
            let n = &pool[(start + i) % pool.len()];
            if !taken(n) {
                return n.clone();
            }
        }
        let base = &pool[start];
        (2..)
            .map(|k| format!("{base} {k}"))
            .find(|n| !taken(n))
            .expect("unbounded")
    }

    /// A stone block was broken by `actor`: its entry goes, unless only
    /// the owner may break it. Ok(None) when no stone was there. One without
    /// an owner is anybody's to break.
    pub fn broken(
        &mut self,
        dimension: &str,
        pos: [i32; 3],
        actor: &Actor,
    ) -> Result<Option<Stone>, Msg> {
        let Some(w) = self.at(dimension, pos).cloned() else {
            return Ok(None);
        };
        if self.config.owner_only_break && w.owner != NO_OWNER && !actor.manages(&w) {
            return Err(msg(
                "wayfarer_stones.error.only_owner_breaks",
                &[&w.owner_name, &w.name],
            ));
        }
        self.remove(w.id);
        Ok(Some(w))
    }

    /// Takes a stone out of the registry and out of everyone's list.
    pub fn remove(&mut self, id: u64) -> Option<Stone> {
        let i = self.data.stones.iter().position(|w| w.id == id)?;
        let w = self.data.stones.remove(i);
        for list in self.players.known.values_mut() {
            list.retain(|&k| k != id);
        }
        self.players.known.retain(|_, l| !l.is_empty());
        self.changed = true;
        Some(w)
    }

    /// The player learns the stone: true when it is new to them.
    pub fn activate(&mut self, uuid: &str, id: u64) -> bool {
        let list = self.players.known.entry(uuid.to_owned()).or_default();
        if list.contains(&id) {
            return false;
        }
        list.push(id);
        self.changed = true;
        true
    }

    pub fn forget(&mut self, uuid: &str, id: u64) -> bool {
        let Some(list) = self.players.known.get_mut(uuid) else {
            return false;
        };
        let before = list.len();
        list.retain(|&k| k != id);
        let gone = list.len() != before;
        if gone {
            self.changed = true;
        }
        gone
    }

    pub fn knows(&self, uuid: &str, id: u64) -> bool {
        self.get(id).is_some_and(|w| w.global)
            || self
                .players
                .known
                .get(uuid)
                .is_some_and(|l| l.contains(&id))
    }

    /// The stones the player knows, the global ones included, by name.
    pub fn known(&self, uuid: &str) -> Vec<&Stone> {
        let mut list: Vec<&Stone> = self
            .data
            .stones
            .iter()
            .filter(|w| {
                w.global
                    || self
                        .players
                        .known
                        .get(uuid)
                        .is_some_and(|l| l.contains(&w.id))
            })
            .collect();
        list.sort_by(|a, b| {
            a.name
                .to_lowercase()
                .cmp(&b.name.to_lowercase())
                .then(a.id.cmp(&b.id))
        });
        list
    }

    /// The stone the actor manages nearest to `pos`, within `NEAR`.
    pub fn near(&self, actor: &Actor, dimension: &str, pos: [f64; 3]) -> Option<&Stone> {
        self.data
            .stones
            .iter()
            .filter(|w| w.dimension == dimension && actor.manages(w))
            .map(|w| (distance(pos, center(w)), w))
            .filter(|(d, _)| *d <= NEAR)
            .min_by(|a, b| a.0.total_cmp(&b.0))
            .map(|(_, w)| w)
    }

    pub fn rename(&mut self, id: u64, name: &str, actor: &Actor) -> Result<(), Msg> {
        let name = name.trim();
        if name.is_empty() || name.chars().count() > MAX_NAME {
            return Err(msg("wayfarer_stones.error.name_length", &[&MAX_NAME]));
        }
        // A client's text is checked by nothing else.
        if name.chars().any(|c| c.is_control() || c == '\u{a7}') {
            return Err(msg("wayfarer_stones.error.name_characters", &[]));
        }
        if self.by_name(name).is_some_and(|w| w.id != id) {
            return Err(msg("wayfarer_stones.error.name_taken", &[&name]));
        }
        let w = self
            .get_mut(id)
            .ok_or_else(|| msg("wayfarer_stones.error.no_such", &[]))?;
        if !actor.manages(w) {
            return Err(if w.owner == NO_OWNER {
                msg("wayfarer_stones.error.only_operator_renames", &[])
            } else {
                msg("wayfarer_stones.error.only_owner_renames", &[&w.owner_name])
            });
        }
        w.name = name.to_owned();
        self.changed = true;
        Ok(())
    }

    pub fn set_global(&mut self, id: u64, global: bool) -> Result<(), Msg> {
        let w = self
            .get_mut(id)
            .ok_or_else(|| msg("wayfarer_stones.error.no_such", &[]))?;
        w.global = global;
        self.changed = true;
        Ok(())
    }

    /// Levels a travel from `from` (dimension, position) to `to` costs.
    pub fn cost(&self, from: (&str, [f64; 3]), to: &Stone) -> u32 {
        let base = self.config.xp_levels_cost;
        if base == 0 {
            return 0;
        }
        let cost = if from.0 != to.dimension {
            base.saturating_mul(self.config.cross_dimension_multiplier.max(1))
        } else {
            let far = distance(from.1, center(to));
            let free = f64::from(self.config.free_within_blocks);
            if free > 0.0 && far <= free {
                return 0;
            }
            let per = f64::from(self.config.blocks_per_level);
            let extra = if per > 0.0 { (far / per) as u32 } else { 0 };
            base.saturating_add(extra)
        };
        match self.config.max_levels_cost {
            0 => cost,
            max => cost.min(max),
        }
    }

    /// A player's command (`/wayfarer ...`, `/wayfarer`), with where they
    /// stand. `line` has no slash.
    pub fn command(
        &mut self,
        actor: &Actor,
        dimension: &str,
        pos: [f64; 3],
        line: &str,
    ) -> Vec<Out> {
        let line = line.trim();
        let (word, rest) = split_first(line);
        if word != "wayfarer" {
            return Vec::new();
        }
        let (sub, arg) = split_first(rest);
        match sub {
            "" => vec![Out::Open],
            "list" => {
                let known = self.known(&actor.uuid);
                if known.is_empty() {
                    return vec![Out::Reply(msg("wayfarer_stones.list.none", &[]))];
                }
                let mut out = vec![Out::Reply(msg("wayfarer_stones.list.count", &[&known.len()]))];
                for w in known {
                    let key = if w.global {
                        "wayfarer_stones.list.row_global"
                    } else {
                        "wayfarer_stones.list.row"
                    };
                    out.push(Out::Reply(msg(key, &[&w.name]).and(place_of(w))));
                }
                out
            }
            "to" => {
                if arg.is_empty() {
                    return vec![Out::Error(msg("wayfarer_stones.usage.to", &[]))];
                }
                let Some(w) = self.by_name(arg) else {
                    return vec![Out::Error(msg("wayfarer_stones.error.no_name", &[&arg]))];
                };
                if !self.knows(&actor.uuid, w.id) {
                    return vec![Out::Error(msg("wayfarer_stones.error.unknown", &[&w.name]))];
                }
                let id = w.id;
                let here = here_of(self, &actor.uuid, dimension, pos);
                if here == Some(id) {
                    return vec![Out::Error(msg("wayfarer_stones.error.already_here", &[]))];
                }
                let scroll = self.needs_scroll(here);
                if scroll && !self.config.wayfarers_scroll {
                    return vec![Out::Error(msg("wayfarer_stones.error.stand_at_stone", &[]))];
                }
                vec![Out::Travel { id, scroll }]
            }
            "name" => {
                if arg.is_empty() {
                    return vec![Out::Error(msg("wayfarer_stones.usage.name", &[]))];
                }
                let Some(id) = self.near(actor, dimension, pos).map(|w| w.id) else {
                    return vec![Out::Error(msg("wayfarer_stones.error.stand_near", &[&NEAR]))];
                };
                match self.rename(id, arg, actor) {
                    Ok(()) => vec![Out::Reply(msg("wayfarer_stones.renamed", &[&arg.trim()]))],
                    Err(e) => vec![Out::Error(e)],
                }
            }
            "forget" => {
                let Some(w) = self.by_name(arg) else {
                    return vec![Out::Error(msg("wayfarer_stones.error.no_name", &[&arg]))];
                };
                let (id, name, global) = (w.id, w.name.clone(), w.global);
                if global {
                    return vec![Out::Error(msg("wayfarer_stones.error.forget_global", &[]))];
                }
                if self.forget(&actor.uuid, id) {
                    vec![Out::Reply(msg("wayfarer_stones.forgot", &[&name]))]
                } else {
                    vec![Out::Error(msg("wayfarer_stones.error.not_known", &[&name]))]
                }
            }
            "global" => {
                if actor.level < 2 {
                    return vec![Out::Error(msg("wayfarer_stones.error.operator_global", &[]))];
                }
                let Some(w) = self.by_name(arg) else {
                    return vec![Out::Error(msg("wayfarer_stones.error.no_name", &[&arg]))];
                };
                let (id, name, global) = (w.id, w.name.clone(), !w.global);
                match self.set_global(id, global) {
                    Ok(()) => vec![Out::Reply(if global {
                        msg("wayfarer_stones.global.on", &[&name])
                    } else {
                        msg("wayfarer_stones.global.off", &[&name])
                    })],
                    Err(e) => vec![Out::Error(e)],
                }
            }
            "remove" => {
                if actor.level < 2 {
                    return vec![Out::Error(msg("wayfarer_stones.error.operator_remove", &[]))];
                }
                let Some(w) = self.by_name(arg) else {
                    return vec![Out::Error(msg("wayfarer_stones.error.no_name", &[&arg]))];
                };
                let (id, name) = (w.id, w.name.clone());
                self.remove(id);
                vec![Out::Reply(msg("wayfarer_stones.removed_from_registry", &[&name]))]
            }
            _ => vec![Out::Error(msg("wayfarer_stones.usage.all", &[]))],
        }
    }

    /// Whether a travel from where a player stands uses up a wayfarer's scroll:
    /// `here` is the stone they stand at. A stone travels for its levels
    /// alone; anywhere else takes a scroll, unless the config lets it be.
    pub fn needs_scroll(&self, here: Option<u64>) -> bool {
        here.is_none() && !self.config.travel_anywhere
    }
}

fn split_first(s: &str) -> (&str, &str) {
    let s = s.trim();
    match s.split_once(' ') {
        Some((a, b)) => (a, b.trim()),
        None => (s, ""),
    }
}

/// A seed from a position: the same block always draws the same first name.
fn seed(dimension: &str, pos: [i32; 3]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in dimension
        .bytes()
        .chain(pos.iter().flat_map(|c| c.to_le_bytes()))
    {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h
}

/// Where a travel lands: on top of the stone's two blocks, at its centre.
pub fn landing(w: &Stone) -> [f64; 3] {
    [
        f64::from(w.pos[0]) + 0.5,
        f64::from(w.pos[1]) + 2.0,
        f64::from(w.pos[2]) + 0.5,
    ]
}

/// The stone's middle, where its two blocks meet, for distances.
pub fn center(w: &Stone) -> [f64; 3] {
    [
        f64::from(w.pos[0]) + 0.5,
        f64::from(w.pos[1]) + 1.0,
        f64::from(w.pos[2]) + 0.5,
    ]
}

pub fn distance(a: [f64; 3], b: [f64; 3]) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

/// Where a stone is, as a player reads it.
pub fn place_of(w: &Stone) -> Msg {
    place_line(&w.dimension, w.pos)
}

fn place_line(dimension: &str, [x, y, z]: [i32; 3]) -> Msg {
    match dimension {
        "minecraft:overworld" => msg("wayfarer_stones.place.overworld", &[&x, &y, &z]),
        "minecraft:the_nether" => msg("wayfarer_stones.place.nether", &[&x, &y, &z]),
        "minecraft:the_end" => msg("wayfarer_stones.place.end", &[&x, &y, &z]),
        other => msg("wayfarer_stones.place.other", &[&other, &x, &y, &z]),
    }
}

/// What one block is to a player landing there.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    /// Air, water, grass: a player stands in it.
    Passable,
    Solid,
    /// Lava or fire.
    Hazard,
}

/// Where a travel to a stone ends.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Landing {
    At([f64; 3]),
    /// Nowhere is safe: built in, or over lava.
    Blocked,
    /// Its chunks aren't loaded, so nobody can tell yet.
    Unknown,
}

/// The safe place to land at a stone: beside it on solid ground, on one of
/// its four sides, else on top of it. `cell` reads a block; none while it isn't
/// loaded. A player two blocks tall needs both free.
pub fn landing_spot(w: &Stone, cell: &dyn Fn([i32; 3]) -> Option<Cell>) -> Landing {
    let [x, y, z] = w.pos;
    let free = |pos: [i32; 3]| -> Option<bool> {
        let [a, b, c] = pos;
        Some(cell([a, b, c])? == Cell::Passable && cell([a, b + 1, c])? == Cell::Passable)
    };
    for [dx, dz] in [[0, 1], [0, -1], [1, 0], [-1, 0]] {
        let at = [x + dx, y, z + dz];
        let Some(room) = free(at) else {
            return Landing::Unknown;
        };
        let Some(floor) = cell([x + dx, y - 1, z + dz]) else {
            return Landing::Unknown;
        };
        if room && floor == Cell::Solid {
            return Landing::At([
                f64::from(at[0]) + 0.5,
                f64::from(at[1]),
                f64::from(at[2]) + 0.5,
            ]);
        }
    }
    match free([x, y + 2, z]) {
        Some(true) => Landing::At(landing(w)),
        Some(false) => Landing::Blocked,
        None => Landing::Unknown,
    }
}

/// The console command that moves `player` there. It fails (the game's
/// `native_run_command` answers false) when the player can't be found.
pub fn travel_command(player: &str, dimension: &str, [x, y, z]: [f64; 3]) -> String {
    // `.3`: a whole number would be read as a block's centre by `tp`.
    format!("execute in {dimension} run tp {player} {x:.3} {y:.3} {z:.3}")
}

/// The console command that fails unless `player` has `levels` experience
/// levels. The `if` must end the command: followed by `run`, a condition that
/// doesn't hold forks to nothing and the command succeeds, having done nothing.
pub fn level_check_command(player: &str, levels: u32) -> String {
    format!("execute if entity @a[name={player},level={levels}..]")
}

/// The command that puts a stone's upper block over its lower one.
pub fn upper_block_command(dimension: &str, [x, y, z]: [i32; 3]) -> String {
    let y = y + 1;
    format!("execute in {dimension} run setblock {x} {y} {z} {TOP}")
}

/// The command that takes away the other block of a broken stone: `setblock`
/// drops nothing, so the block the player broke is the only item.
pub fn remove_block_command(dimension: &str, [x, y, z]: [i32; 3]) -> String {
    format!("execute in {dimension} run setblock {x} {y} {z} minecraft:air")
}

/// A burst of `particle` at `at`.
fn particles_command(dimension: &str, particle: &str, [x, y, z]: [f64; 3], count: u32) -> String {
    format!(
        "execute in {dimension} run particle minecraft:{particle} {x:.3} {y:.3} {z:.3} 0.4 0.8 0.4 0.4 {count}"
    )
}

/// `sound` at `at`, for the players of that dimension within earshot. The
/// selector keeps it there: `playsound` alone reaches every dimension's players.
fn sound_command(dimension: &str, sound: &str, [x, y, z]: [f64; 3], volume: f32, pitch: f32) -> String {
    format!(
        "execute in {dimension} positioned {x:.3} {y:.3} {z:.3} run playsound minecraft:{sound} player @a[distance=..32] {x:.3} {y:.3} {z:.3} {volume} {pitch}"
    )
}

/// What leaving a place looks and sounds like, `at` being the player's feet.
pub fn departure_commands(dimension: &str, [x, y, z]: [f64; 3]) -> [String; 2] {
    let at = [x, y + 1.0, z];
    [
        particles_command(dimension, "portal", at, 64),
        sound_command(dimension, "entity.enderman.teleport", at, 1.0, 1.0),
    ]
}

/// What arriving looks and sounds like, `at` being the player's feet.
pub fn arrival_commands(dimension: &str, [x, y, z]: [f64; 3]) -> [String; 2] {
    let at = [x, y + 1.0, z];
    [
        particles_command(dimension, "reverse_portal", at, 64),
        sound_command(dimension, "entity.enderman.teleport", at, 1.0, 1.2),
    ]
}

/// What a stone a player has just learned looks and sounds like.
pub fn activation_commands(w: &Stone) -> [String; 2] {
    let at = center(w);
    [
        particles_command(&w.dimension, "enchant", at, 48),
        sound_command(&w.dimension, "block.beacon.activate", at, 0.7, 1.4),
    ]
}

/// The command that gives a wayfarer's scroll back, when a travel failed after it was taken.
pub fn refund_scroll_command(player: &str) -> String {
    format!("give {player} {SCROLL} 1")
}

/// The command that takes one wayfarer's scroll from the player.
pub fn consume_scroll_command(player: &str) -> String {
    format!("clear {player} {SCROLL} 1")
}

/// One stone as a player's screen shows it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    pub id: u64,
    pub name: String,
    pub dimension: String,
    pub pos: [i32; 3],
    pub owner_name: String,
    pub global: bool,
    pub cost: u32,
    /// Blocks away, in the same dimension.
    pub distance: Option<f64>,
    /// The player stands at it.
    pub here: bool,
    /// The player owns or manages it, so the screen offers renaming.
    pub can_rename: bool,
}

/// The player's entries: what they know, with the cost and distance from where
/// they stand, and whether they stand at it.
pub fn entries(
    reg: &Registry,
    actor: &Actor,
    dimension: &str,
    pos: [f64; 3],
    here: Option<u64>,
) -> Vec<Entry> {
    reg.known(&actor.uuid)
        .into_iter()
        .map(|w| Entry {
            id: w.id,
            name: w.name.clone(),
            dimension: w.dimension.clone(),
            pos: w.pos,
            owner_name: w.owner_name.clone(),
            global: w.global,
            cost: reg.cost((dimension, pos), w),
            distance: (w.dimension == dimension).then(|| distance(pos, center(w))),
            here: here == Some(w.id),
            can_rename: actor.manages(w),
        })
        .collect()
}

/// The server's message to a player's client.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListMessage {
    /// Open the screen; otherwise only refresh it if it is open.
    pub open: bool,
    pub entries: Vec<Entry>,
    /// How many the player knows, when the message couldn't carry them all.
    pub total: usize,
    pub notice: Option<Msg>,
    /// A travel from where the player stands uses up a wayfarer's scroll.
    #[serde(default)]
    pub scroll: bool,
    /// Milliseconds before the player may travel again.
    #[serde(default)]
    pub cooldown_ms: i64,
}

/// The message as bytes, without the last entries if they don't fit in one.
pub fn encode_list(mut message: ListMessage) -> Vec<u8> {
    message.total = message.entries.len();
    loop {
        let bytes = serde_json::to_vec(&message).unwrap_or_default();
        if bytes.len() <= MAX_MESSAGE || message.entries.is_empty() {
            return bytes;
        }
        // Each entry is about the same size: go straight near the limit.
        let keep = message.entries.len() * MAX_MESSAGE / bytes.len();
        message
            .entries
            .truncate(keep.clamp(0, message.entries.len() - 1));
    }
}

pub fn decode_list(data: &[u8]) -> Option<ListMessage> {
    serde_json::from_slice(data).ok()
}

/// What a client asks of the server. Treated as a player's own input.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ClientAct {
    Travel(u64),
    Rename { id: u64, name: String },
}

pub fn encode_act(act: &ClientAct) -> Vec<u8> {
    serde_json::to_vec(act).unwrap_or_default()
}

pub fn decode_act(data: &[u8]) -> Option<ClientAct> {
    serde_json::from_slice(data).ok()
}

/// What the server remembers of a player's open screen.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Session {
    /// Opened away from a stone: travelling uses up a wayfarer's scroll.
    pub scroll: bool,
    /// The stone the player stands at.
    pub here: Option<u64>,
}

// ---- the screen ------------------------------------------------------------

/// What the screen asks of the game, from the hooks that may only read.
#[derive(Clone, Debug, PartialEq)]
pub enum UiAction {
    Send(ClientAct),
    Close,
}

/// One thing to draw, in GUI pixels, in order.
#[derive(Clone, Debug, PartialEq)]
pub enum Draw {
    Fill([f32; 4], u32),
    Sprite(&'static str, [f32; 4], [f32; 4]),
    Text(String, [f32; 2], u32),
    Item(&'static str, [f32; 2], f32),
}

pub const PANEL_SPRITE: &str = "wayfarer_stones:gui/panel";
pub const RENAME_SPRITE: &str = "wayfarer_stones:gui/rename";
const PANEL_WIDTH: f32 = 252.0;
const ROW_HEIGHT: f32 = 24.0;
const MAX_ROWS: usize = 7;
const HEAD: f32 = 38.0;
const FOOT: f32 = 22.0;
const BORDER: f32 = 8.0;
const SCROLLBAR: f32 = 6.0;

const DIM: u32 = 0x9000_0000;
const STONE: u32 = 0xFF3A_3E4E;
const INNER: u32 = 0xFF22_2531;
const ROW_EVEN: u32 = 0xFF2A_2E3B;
const ROW_ODD: u32 = 0xFF26_2935;
const ROW_HOVER: u32 = 0xFF34_4C58;
const FIELD: u32 = 0xFF15_171F;
const FIELD_EDGE: u32 = 0xFF2C_C4C4;
const THUMB: u32 = 0xFF2C_C4C4;
const TRACK: u32 = 0xFF15_171F;
const TITLE: u32 = 0x96F6E8;
const NAME: u32 = 0xFFFFFF;
const NAME_GLOBAL: u32 = 0xFF88FF;
const NAME_HERE: u32 = 0xAAAAAA;
const MUTED: u32 = 0x8A90A4;
const GOLD: u32 = 0xFFAA00;
const GREEN: u32 = 0x55FF55;
const YELLOW: u32 = 0xFFFF55;
const RED: u32 = 0xFF6060;

fn inside(rect: [f32; 4], p: [f32; 2]) -> bool {
    p[0] >= rect[0] && p[1] >= rect[1] && p[0] < rect[0] + rect[2] && p[1] < rect[1] + rect[3]
}

/// `text` cut with `..` to fit `max` pixels.
fn fit(text: &str, max: f32, measure: &dyn Fn(&str) -> f32) -> String {
    if measure(text) <= max {
        return text.to_owned();
    }
    let mut out = String::new();
    for c in text.chars() {
        let longer = format!("{out}{c}..");
        if measure(&longer) > max {
            break;
        }
        out.push(c);
    }
    out + ".."
}

/// Where everything is, for a window of this size.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Geometry {
    panel: [f32; 4],
    close: [f32; 4],
    search: [f32; 4],
    list: [f32; 4],
    rows: usize,
    footer: [f32; 2],
}

impl Geometry {
    fn new(window: [f32; 2]) -> Self {
        let room = ((window[1] - 16.0 - HEAD - FOOT) / ROW_HEIGHT).floor();
        let rows = (room.max(1.0) as usize).min(MAX_ROWS);
        let height = HEAD + rows as f32 * ROW_HEIGHT + FOOT;
        let x = ((window[0] - PANEL_WIDTH) / 2.0).floor().max(0.0);
        let y = ((window[1] - height) / 2.0).floor().max(0.0);
        let inner = PANEL_WIDTH - 2.0 * BORDER;
        Self {
            panel: [x, y, PANEL_WIDTH, height],
            close: [x + PANEL_WIDTH - BORDER - 10.0, y + 7.0, 10.0, 10.0],
            search: [x + BORDER, y + 20.0, inner, 14.0],
            list: [
                x + BORDER,
                y + HEAD,
                inner - SCROLLBAR,
                rows as f32 * ROW_HEIGHT,
            ],
            rows,
            footer: [x + BORDER, y + height - FOOT + 3.0],
        }
    }

    fn row(&self, i: usize) -> [f32; 4] {
        [
            self.list[0],
            self.list[1] + i as f32 * ROW_HEIGHT,
            self.list[2],
            ROW_HEIGHT,
        ]
    }

    fn pencil(&self, i: usize) -> [f32; 4] {
        let r = self.row(i);
        [r[0] + r[2] - 14.0, r[1] + 7.0, 10.0, 10.0]
    }
}

/// The client's screen: its list, what is typed, and what it asks for.
#[derive(Debug)]
pub struct Ui {
    pub entries: Vec<Entry>,
    pub total: usize,
    pub filter: String,
    pub scroll: usize,
    scroll_part: f64,
    /// The stone being renamed, and the name so far.
    pub rename: Option<(u64, String)>,
    pub notice: Option<Msg>,
    /// A travel from here uses up a wayfarer's scroll.
    pub needs_scroll: bool,
    /// When the player may travel again, on this window's clock (0: now),
    /// and the seconds until then as last drawn.
    pub ready_at: i64,
    pub cooldown: u64,
    pub outbox: Vec<UiAction>,
    /// The window's size in GUI pixels, as last drawn: clicks have no other.
    pub window: [f32; 2],
}

impl Ui {
    pub fn new(message: ListMessage) -> Self {
        Self {
            entries: message.entries,
            total: message.total,
            filter: String::new(),
            scroll: 0,
            scroll_part: 0.0,
            rename: None,
            notice: message.notice,
            needs_scroll: message.scroll,
            ready_at: 0,
            cooldown: 0,
            outbox: Vec::new(),
            window: [480.0, 270.0],
        }
    }

    /// The server sent a new list while the screen is open.
    pub fn refresh(&mut self, message: ListMessage) {
        self.entries = message.entries;
        self.total = message.total;
        self.notice = message.notice;
        self.needs_scroll = message.scroll;
        if let Some((id, _)) = &self.rename
            && !self.entries.iter().any(|e| e.id == *id)
        {
            self.rename = None;
        }
        self.clamp_scroll();
    }

    /// The entries the search leaves, by name.
    pub fn visible(&self) -> Vec<&Entry> {
        let needle = self.filter.trim().to_lowercase();
        self.entries
            .iter()
            .filter(|e| needle.is_empty() || e.name.to_lowercase().contains(&needle))
            .collect()
    }

    fn clamp_scroll(&mut self) {
        let rows = Geometry::new(self.window).rows;
        self.scroll = self.scroll.min(self.visible().len().saturating_sub(rows));
    }

    fn travel(&mut self, id: u64) {
        if self.cooldown > 0 {
            return;
        }
        self.outbox.push(UiAction::Send(ClientAct::Travel(id)));
        self.outbox.push(UiAction::Close);
    }

    /// A mouse button went down at `at` (0: the left one).
    pub fn click(&mut self, at: [f32; 2], button: i32) {
        if button != 0 {
            return;
        }
        let g = Geometry::new(self.window);
        // A click anywhere else leaves a rename without saving it.
        if self.rename.is_some() {
            let editing = self.rename.as_ref().map(|r| r.0);
            let on_it = (0..g.rows).any(|i| {
                inside(g.row(i), at) && self.visible().get(self.scroll + i).map(|e| e.id) == editing
            });
            if !on_it {
                self.rename = None;
            }
            return;
        }
        if inside(g.close, at) {
            self.outbox.push(UiAction::Close);
            return;
        }
        let Some(i) = (0..g.rows).find(|&i| inside(g.row(i), at)) else {
            return;
        };
        let Some((id, name, here, can_rename)) = self
            .visible()
            .get(self.scroll + i)
            .map(|e| (e.id, e.name.clone(), e.here, e.can_rename))
        else {
            return;
        };
        if can_rename && inside(g.pencil(i), at) {
            self.rename = Some((id, name));
        } else if !here {
            self.travel(id);
        }
    }

    /// The wheel turned by `y` (up is positive); a touchpad's parts add up.
    pub fn wheel(&mut self, y: f64) {
        self.scroll_part += y;
        while self.scroll_part >= 1.0 {
            self.scroll_part -= 1.0;
            self.scroll = self.scroll.saturating_sub(1);
        }
        while self.scroll_part <= -1.0 {
            self.scroll_part += 1.0;
            self.scroll += 1;
        }
        self.clamp_scroll();
    }

    /// A key went down: true when the screen took it. Every key but Escape on
    /// the list is taken, so typing `e` in the search doesn't close the screen.
    pub fn key(&mut self, key: &str) -> bool {
        match key {
            "key.keyboard.escape" => {
                if self.rename.is_some() {
                    self.rename = None;
                    return true;
                }
                return false;
            }
            "key.keyboard.backspace" => {
                if let Some((_, name)) = &mut self.rename {
                    name.pop();
                } else {
                    self.filter.pop();
                    self.clamp_scroll();
                }
            }
            "key.keyboard.enter" | "key.keyboard.keypad.enter" => {
                if let Some((id, name)) = self.rename.take() {
                    self.outbox.push(UiAction::Send(ClientAct::Rename {
                        id,
                        name: name.trim().to_owned(),
                    }));
                } else if let [only] = self.visible()[..]
                    && !only.here
                {
                    let id = only.id;
                    self.travel(id);
                }
            }
            "key.keyboard.up" => self.wheel(1.0),
            "key.keyboard.down" => self.wheel(-1.0),
            _ => {}
        }
        true
    }

    /// A character was typed.
    pub fn typed(&mut self, c: char) {
        if c.is_control() {
            return;
        }
        if let Some((_, name)) = &mut self.rename {
            if name.chars().count() < MAX_NAME {
                name.push(c);
            }
        } else if self.filter.chars().count() < MAX_NAME {
            self.filter.push(c);
            self.scroll = 0;
        }
    }

    /// What to draw, with `mouse` for hovering, `caret` for the blink,
    /// `measure` the font's width and `tr` the player's language.
    pub fn render(
        &self,
        mouse: [f32; 2],
        caret: bool,
        measure: &dyn Fn(&str) -> f32,
        tr: &dyn Fn(&Msg) -> String,
    ) -> Vec<Draw> {
        let g = Geometry::new(self.window);
        let [px, py, pw, ph] = g.panel;
        let mut out = vec![Draw::Fill([0.0, 0.0, self.window[0], self.window[1]], DIM)];
        // Flat colours first: with no texture (no content mod) it still reads.
        out.push(Draw::Fill(g.panel, STONE));
        out.push(Draw::Fill([px + 4.0, py + 4.0, pw - 8.0, ph - 8.0], INNER));
        panel_border(&mut out, g.panel);

        out.push(Draw::Text(
            tr(&msg("wayfarer_stones.screen.title", &[])),
            [px + BORDER, py + 8.0],
            TITLE,
        ));
        let count = tr(&msg("wayfarer_stones.screen.known", &[&self.total]));
        let count_x = g.close[0] - 6.0 - measure(&count);
        out.push(Draw::Text(count, [count_x, py + 8.0], MUTED));
        let close_hover = inside(g.close, mouse) && self.rename.is_none();
        out.push(Draw::Text(
            "x".into(),
            [g.close[0] + 3.0, g.close[1] + 1.0],
            if close_hover { RED } else { MUTED },
        ));

        // The search.
        let [sx, sy, sw, sh] = g.search;
        out.push(Draw::Fill(g.search, FIELD_EDGE));
        out.push(Draw::Fill([sx + 1.0, sy + 1.0, sw - 2.0, sh - 2.0], FIELD));
        if self.filter.is_empty() && self.rename.is_none() {
            out.push(Draw::Text(
                tr(&msg("wayfarer_stones.screen.search", &[])),
                [sx + 4.0, sy + 3.0],
                MUTED,
            ));
        } else {
            let mut shown = fit(&self.filter, sw - 12.0, measure);
            if caret && self.rename.is_none() {
                shown.push('_');
            }
            out.push(Draw::Text(shown, [sx + 4.0, sy + 3.0], NAME));
        }

        // The list.
        let visible = self.visible();
        for i in 0..g.rows {
            let r = g.row(i);
            let Some(e) = visible.get(self.scroll + i) else {
                let back = if i.is_multiple_of(2) {
                    ROW_EVEN
                } else {
                    ROW_ODD
                };
                out.push(Draw::Fill(r, back));
                continue;
            };
            self.draw_row(&mut out, &g, i, e, mouse, caret, measure, tr);
        }
        if visible.is_empty() {
            let line = tr(&msg(
                if self.entries.is_empty() {
                    "wayfarer_stones.screen.empty"
                } else {
                    "wayfarer_stones.screen.no_match"
                },
                &[],
            ));
            let line = fit(&line, g.list[2] - 8.0, measure);
            let x = g.list[0] + (g.list[2] - measure(&line)) / 2.0;
            out.push(Draw::Text(line, [x, g.list[1] + 8.0], MUTED));
        }

        // The scrollbar.
        let track = [
            g.list[0] + g.list[2] + 2.0,
            g.list[1],
            SCROLLBAR - 2.0,
            g.list[3],
        ];
        out.push(Draw::Fill(track, TRACK));
        if visible.len() > g.rows {
            let height = (track[3] * g.rows as f32 / visible.len() as f32).max(8.0);
            let span = (visible.len() - g.rows) as f32;
            let y = track[1] + (track[3] - height) * self.scroll as f32 / span;
            out.push(Draw::Fill([track[0], y, track[2], height], THUMB));
        }

        // The line at the bottom.
        let (line, color) = if let Some(n) = &self.notice {
            (n.clone(), RED)
        } else if self.rename.is_some() {
            (msg("wayfarer_stones.screen.hint_rename", &[]), MUTED)
        } else if self.cooldown > 0 {
            (msg("wayfarer_stones.screen.cooldown", &[&self.cooldown]), YELLOW)
        } else if self.total > self.entries.len() {
            (
                msg("wayfarer_stones.screen.showing", &[&self.entries.len(), &self.total]),
                YELLOW,
            )
        } else if self.needs_scroll {
            (msg("wayfarer_stones.screen.hint_scroll", &[]), GOLD)
        } else {
            (msg("wayfarer_stones.screen.hint", &[]), MUTED)
        };
        out.push(Draw::Text(
            fit(&tr(&line), pw - 2.0 * BORDER, measure),
            g.footer,
            color,
        ));
        out
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_row(
        &self,
        out: &mut Vec<Draw>,
        g: &Geometry,
        i: usize,
        e: &Entry,
        mouse: [f32; 2],
        caret: bool,
        measure: &dyn Fn(&str) -> f32,
        tr: &dyn Fn(&Msg) -> String,
    ) {
        let r = g.row(i);
        let editing = self.rename.as_ref().filter(|(id, _)| *id == e.id);
        let hover = inside(r, mouse) && !e.here && self.rename.is_none() && self.cooldown == 0;
        let back = if hover {
            ROW_HOVER
        } else if i.is_multiple_of(2) {
            ROW_EVEN
        } else {
            ROW_ODD
        };
        out.push(Draw::Fill(r, back));
        out.push(Draw::Item(BLOCK, [r[0] + 4.0, r[1] + 4.0], 16.0));

        // On the right: the price and the distance, then the pencil.
        let right = r[0] + r[2] - if e.can_rename { 18.0 } else { 4.0 };
        let (price, price_color) = if e.here {
            (msg("wayfarer_stones.screen.here", &[]), YELLOW)
        } else {
            match e.cost {
                0 => (msg("wayfarer_stones.screen.free", &[]), GREEN),
                1 => (msg("wayfarer_stones.screen.level", &[]), GOLD),
                n => (msg("wayfarer_stones.screen.levels", &[&n]), GOLD),
            }
        };
        let price = tr(&price);
        let price_width = measure(&price);
        out.push(Draw::Text(
            price,
            [right - price_width, r[1] + 3.0],
            price_color,
        ));
        if let Some(d) = e.distance.filter(|_| !e.here) {
            let line = tr(&msg("wayfarer_stones.screen.metres", &[&(d.round() as i64)]));
            let width = measure(&line);
            out.push(Draw::Text(line, [right - width, r[1] + 13.0], MUTED));
        }
        if e.can_rename {
            out.push(Draw::Sprite(
                RENAME_SPRITE,
                [0.0, 0.0, 16.0, 16.0],
                g.pencil(i),
            ));
        }

        // On the left: the name, then where it is.
        let x = r[0] + 24.0;
        let room = right - price_width - 6.0 - x;
        if let Some((_, name)) = editing {
            let field = [x - 2.0, r[1] + 1.0, room + 2.0, 11.0];
            out.push(Draw::Fill(field, FIELD_EDGE));
            out.push(Draw::Fill(
                [
                    field[0] + 1.0,
                    field[1] + 1.0,
                    field[2] - 2.0,
                    field[3] - 2.0,
                ],
                FIELD,
            ));
            let mut shown = fit(name, room - 8.0, measure);
            if caret {
                shown.push('_');
            }
            out.push(Draw::Text(shown, [x, r[1] + 3.0], NAME));
        } else {
            let color = match (e.here, e.global) {
                (true, _) => NAME_HERE,
                (false, true) => NAME_GLOBAL,
                (false, false) => NAME,
            };
            out.push(Draw::Text(
                fit(&e.name, room, measure),
                [x, r[1] + 3.0],
                color,
            ));
        }
        let place = tr(&place_line(&e.dimension, e.pos));
        let place_room = right - 34.0 - x;
        out.push(Draw::Text(
            fit(&place, place_room, measure),
            [x, r[1] + 13.0],
            MUTED,
        ));
    }
}

/// The carved border: the sprite's corners, and its edges stretched between them.
fn panel_border(out: &mut Vec<Draw>, [x, y, w, h]: [f32; 4]) {
    const SIZE: f32 = 32.0;
    const C: f32 = 8.0;
    let (mx, my) = (w - 2.0 * C, h - 2.0 * C);
    let (right, bottom) = (x + w - C, y + h - C);
    let mid = SIZE - 2.0 * C;
    let far = SIZE - C;
    for (src, dst) in [
        ([0.0, 0.0, C, C], [x, y, C, C]),
        ([far, 0.0, C, C], [right, y, C, C]),
        ([0.0, far, C, C], [x, bottom, C, C]),
        ([far, far, C, C], [right, bottom, C, C]),
        ([C, 0.0, mid, C], [x + C, y, mx, C]),
        ([C, far, mid, C], [x + C, bottom, mx, C]),
        ([0.0, C, C, mid], [x, y + C, C, my]),
        ([far, C, C, mid], [right, y + C, C, my]),
    ] {
        out.push(Draw::Sprite(PANEL_SPRITE, src, dst));
    }
}

/// The stone the player knows and stands at: on it or beside it.
pub fn here_of(reg: &Registry, uuid: &str, dimension: &str, pos: [f64; 3]) -> Option<u64> {
    reg.known(uuid)
        .into_iter()
        .filter(|w| w.dimension == dimension)
        .map(|w| (distance(pos, center(w)), w.id))
        .filter(|(d, _)| *d < AT_STONE)
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, id)| id)
}

/// A screen to open at the end of the tick.
#[derive(Clone, Debug, PartialEq)]
pub struct OpenRequest {
    pub uuid: u128,
    /// The stone clicked; none finds the one the player stands at.
    pub here: Option<u64>,
}

/// Ticks after an opening in which a player's further requests are dropped:
/// one right click reaches the hook once per hand, and may do so again.
pub const REOPEN_TICKS: u64 = 5;

/// The menus waiting for the end of the tick. A player's click runs with the
/// server's connections taken away, so `native_open_menu` finds nobody there;
/// the server tick has them back.
#[derive(Debug, Default)]
pub struct OpenQueue {
    waiting: Vec<OpenRequest>,
    opened: BTreeMap<u128, u64>,
}

impl OpenQueue {
    /// Adds a request; a player's first one in a tick stands.
    pub fn request(&mut self, request: OpenRequest) {
        if !self.waiting.iter().any(|r| r.uuid == request.uuid) {
            self.waiting.push(request);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.waiting.is_empty()
    }

    /// The requests to act on at tick `now`.
    pub fn take(&mut self, now: u64) -> Vec<OpenRequest> {
        self.opened
            .retain(|_, at| now.saturating_sub(*at) < REOPEN_TICKS);
        let mut out = Vec::new();
        for r in std::mem::take(&mut self.waiting) {
            if let std::collections::btree_map::Entry::Vacant(e) = self.opened.entry(r.uuid) {
                e.insert(now);
                out.push(r);
            }
        }
        out
    }
}

/// A travel waiting for its warmup, or made at the next tick.
#[derive(Clone, Debug, PartialEq)]
pub struct Pending {
    pub uuid: u128,
    pub name: String,
    pub to: u64,
    pub cost: u32,
    /// Take a wayfarer's scroll for it.
    pub scroll: bool,
    /// Where the player stood when it started, and when it is due (ms).
    pub from: (String, [f64; 3]),
    pub due: i64,
}

/// A travel made to a place whose chunks weren't loaded: the player waits on top
/// of the stone until they are, then goes to the safe place, or back.
#[derive(Clone, Debug, PartialEq)]
pub struct Settling {
    pub uuid: u128,
    pub name: String,
    pub to: u64,
    pub from: (String, [f64; 3]),
    pub cost: u32,
    pub scroll: bool,
    /// The tick it gives up, and the player stays where they are.
    pub until: u64,
}

/// Ticks a player waits for the place they travelled to.
pub const SETTLE_TICKS: u64 = 200;

/// Whole seconds until `until`, on a clock in milliseconds; 0 once it has passed.
pub fn seconds_left(until: i64, now: i64) -> u64 {
    u64::try_from(until.saturating_sub(now)).map_or(0, |ms| ms.div_ceil(1000))
}

/// Whether the player moved away from where a warmup started.
pub fn moved(from: &(String, [f64; 3]), now: (&str, [f64; 3])) -> bool {
    from.0 != now.0 || distance(from.1, now.1) > 1.0
}

// ---- saving ----------------------------------------------------------------

/// The world's stones, in `DIR`.
pub const STONES_FILE: &str = "stones.json";
/// Who knows which stones, in `DIR`.
pub const PLAYERS_FILE: &str = "players.json";

/// Where a save goes. `stage` writes a file's new bytes beside it, not yet in
/// its place; `commit` puts a staged file in its place (a rename).
pub trait SaveFiles {
    fn stage(&mut self, name: &str, bytes: &[u8]) -> Result<(), String>;
    fn commit(&mut self, name: &str) -> Result<(), String>;
}

impl Registry {
    /// Writes the stones and the players, if something changed, and is clean
    /// only once both are in place: after a failed write it stays changed, so
    /// the next save writes both again.
    ///
    /// Both files are staged before either is put in place, so a failed write
    /// (a full disk) leaves the old pair as it was. Only a failed rename
    /// between the two can leave them apart, and then the stones are the ones
    /// in place: `stones.json` holds `next_id`, so `players.json` never names a
    /// stone `stones.json` hasn't handed out, and an id is never handed out
    /// twice. At worst a player's newest discoveries wait for the retry.
    pub fn save(&mut self, files: &mut impl SaveFiles) -> Result<(), String> {
        if !self.changed {
            return Ok(());
        }
        let json = |name: &str, value: &dyn erased::Json| {
            value.json().map_err(|e| format!("{name}: {e}"))
        };
        let stones = json(STONES_FILE, &self.data)?;
        let players = json(PLAYERS_FILE, &self.players)?;
        files.stage(STONES_FILE, &stones)?;
        files.stage(PLAYERS_FILE, &players)?;
        files.commit(STONES_FILE)?;
        files.commit(PLAYERS_FILE)?;
        self.changed = false;
        Ok(())
    }
}

mod erased {
    /// A document as the file's bytes: pretty JSON and a final newline.
    pub trait Json {
        fn json(&self) -> serde_json::Result<Vec<u8>>;
    }

    impl<T: serde::Serialize> Json for T {
        fn json(&self) -> serde_json::Result<Vec<u8>> {
            let mut bytes = serde_json::to_vec_pretty(self)?;
            bytes.push(b'\n');
            Ok(bytes)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Files in memory; `fail_stage` / `fail_commit` name a file whose write fails.
    #[derive(Default)]
    struct Disk {
        files: BTreeMap<String, Vec<u8>>,
        staged: BTreeMap<String, Vec<u8>>,
        fail_stage: Option<&'static str>,
        fail_commit: Option<&'static str>,
        writes: usize,
    }

    impl SaveFiles for Disk {
        fn stage(&mut self, name: &str, bytes: &[u8]) -> Result<(), String> {
            self.writes += 1;
            if self.fail_stage == Some(name) {
                return Err(format!("{name}: No space left on device"));
            }
            self.staged.insert(name.into(), bytes.to_vec());
            Ok(())
        }
        fn commit(&mut self, name: &str) -> Result<(), String> {
            if self.fail_commit == Some(name) {
                return Err(format!("{name}: rename failed"));
            }
            let bytes = self.staged.remove(name).ok_or(format!("{name}: not staged"))?;
            self.files.insert(name.into(), bytes);
            Ok(())
        }
    }

    impl Disk {
        fn stones(&self) -> Option<Data> {
            self.files.get(STONES_FILE).map(|b| serde_json::from_slice(b).unwrap())
        }
        fn players(&self) -> Option<Players> {
            self.files.get(PLAYERS_FILE).map(|b| serde_json::from_slice(b).unwrap())
        }
    }

    /// A registry with one stone the player `a` knows.
    fn found_one() -> Registry {
        let mut r = Registry::new(Config::default(), Data::default(), Players::default());
        let id = r.place("minecraft:overworld", [0, 64, 0], &actor("a", 0), 0).id;
        r.activate("a", id);
        assert!(r.changed);
        r
    }

    #[test]
    fn a_save_whose_first_write_fails_is_retried() {
        let mut r = found_one();
        let mut disk = Disk { fail_stage: Some(STONES_FILE), ..Disk::default() };
        assert!(r.save(&mut disk).is_err());
        assert!(r.changed, "still to write");
        assert!(disk.files.is_empty());
        disk.fail_stage = None;
        assert!(r.save(&mut disk).is_ok());
        assert!(!r.changed);
        assert_eq!(disk.stones().unwrap().stones.len(), 1);
        assert_eq!(disk.players().unwrap().known["a"].len(), 1);
    }

    #[test]
    fn a_save_whose_second_write_fails_leaves_the_old_pair_and_is_retried() {
        let mut r = found_one();
        let mut disk = Disk { fail_stage: Some(PLAYERS_FILE), ..Disk::default() };
        assert!(r.save(&mut disk).is_err());
        assert!(r.changed, "still to write");
        assert!(disk.files.is_empty(), "neither file is put in place alone");
        disk.fail_stage = None;
        assert!(r.save(&mut disk).is_ok());
        assert!(!r.changed);
        assert_eq!(disk.stones().unwrap().stones.len(), 1);
        assert_eq!(disk.players().unwrap().known["a"].len(), 1);
    }

    #[test]
    fn a_failed_rename_never_leaves_players_naming_a_stone_the_stones_file_lacks() {
        let mut r = found_one();
        let mut disk = Disk { fail_commit: Some(PLAYERS_FILE), ..Disk::default() };
        assert!(r.save(&mut disk).is_err());
        assert!(r.changed, "still to write");
        // The stones go in place first, so every id players.json names is one
        // stones.json has handed out (its next_id is past it).
        assert!(disk.players().is_none());
        assert_eq!(disk.stones().unwrap().stones.len(), 1);
        disk.fail_commit = None;
        assert!(r.save(&mut disk).is_ok());
        assert!(!r.changed);
        assert_eq!(disk.players().unwrap().known["a"].len(), 1);

        let mut disk = Disk { fail_commit: Some(STONES_FILE), ..Disk::default() };
        r.changed = true;
        assert!(r.save(&mut disk).is_err());
        assert!(r.changed, "still to write");
        assert!(disk.files.is_empty(), "players.json waits for stones.json");
    }

    #[test]
    fn nothing_changed_writes_nothing() {
        let mut r = Registry::new(Config::default(), Data::default(), Players::default());
        let mut disk = Disk::default();
        assert!(r.save(&mut disk).is_ok());
        assert_eq!(disk.writes, 0);
    }

    fn actor(uuid: &str, level: u8) -> Actor {
        Actor {
            uuid: uuid.into(),
            name: format!("Player_{uuid}"),
            level,
        }
    }

    fn reg() -> Registry {
        Registry::new(Config::default(), Data::default(), Players::default())
    }

    /// A file of the mod's folder, read as the tests run: the bundle's rules
    /// allow no source to pull in another file.
    fn file(path: &str) -> String {
        std::fs::read_to_string(format!("{}/{path}", env!("CARGO_MANIFEST_DIR"))).unwrap()
    }

    fn language(code: &str) -> BTreeMap<String, String> {
        let path = format!("content/assets/wayfarer_stones/lang/{code}.json");
        serde_json::from_str(&file(&path)).unwrap()
    }

    /// A line in English, as the game fills in a language file's line.
    fn english(m: &Msg) -> String {
        let table = language("en_us");
        let mut out = table
            .get(&m.key)
            .unwrap_or_else(|| panic!("{} is in no language file", m.key))
            .clone();
        for a in &m.args {
            let a = match a {
                Arg::Plain(s) => s.clone(),
                Arg::Nested(n) => english(n),
            };
            out = out.replacen("%s", &a, 1);
        }
        out
    }

    /// The keys a piece of source names: whole strings only.
    fn keys_in(source: &str) -> Vec<String> {
        source
            .split('"')
            .filter(|t| t.starts_with("wayfarer_stones.") && !t.ends_with(".json"))
            .filter(|t| !t.contains(' ') && !t.ends_with('.'))
            .map(str::to_owned)
            .collect()
    }

    #[test]
    fn every_line_is_in_both_languages_with_the_same_arguments() {
        let (en, ru) = (language("en_us"), language("ru_ru"));
        let used: std::collections::BTreeSet<String> = [
            "mixins/wayfarer_stones_native/runtime.rs",
            "mixins/wayfarer_stones_native/glue.rs",
            "content/data/wayfarer_stones/function/check.mcfunction",
        ]
        .iter()
        .flat_map(|s| keys_in(&file(s)))
        .collect();
        assert!(used.len() > 40, "{used:?}");
        for key in &used {
            assert!(en.contains_key(key), "{key} isn't in en_us");
        }
        for (key, line) in &en {
            let theirs = ru
                .get(key)
                .unwrap_or_else(|| panic!("{key} isn't in ru_ru"));
            assert_eq!(
                line.matches("%s").count(),
                theirs.matches("%s").count(),
                "{key}"
            );
            assert!(
                key.starts_with("block.") || key.starts_with("item.") || used.contains(key),
                "{key} is used nowhere"
            );
        }
        assert_eq!(en.len(), ru.len());
    }

    /// Written as the game's own tables are: a pool's `condition` with a `type`.
    /// A condition it can't read is false, so such a pool silently drops nothing.
    #[test]
    fn both_blocks_drop_one_stone() {
        for name in ["wayfarer_stone", "wayfarer_stone_top"] {
            let path = format!("content/data/wayfarer_stones/loot_table/blocks/{name}.json");
            let table: serde_json::Value = serde_json::from_str(&file(&path)).unwrap();
            let pool = &table["pools"][0];
            assert_eq!(pool["condition"]["type"], "minecraft:survives_explosion", "{name}");
            assert_eq!(pool["rolls"], 1, "{name}");
            assert_eq!(pool["entries"][0]["name"], BLOCK, "{name}");
        }
    }

    #[test]
    fn a_nested_line_survives_a_message() {
        let m = msg("wayfarer_stones.list.row", &[&"Home"]).and(msg("wayfarer_stones.place.end", &[&1, &2, &3]));
        let back: Msg = serde_json::from_str(&serde_json::to_string(&m).unwrap()).unwrap();
        assert_eq!(back, m);
        assert_eq!(english(&m), "  Home: End 1, 2, 3");
    }

    const OW: &str = "minecraft:overworld";
    const NETHER: &str = "minecraft:the_nether";

    #[test]
    fn placing_registers_names_and_teaches_the_owner() {
        let mut r = reg();
        let a = actor("a", 0);
        let w = r.place(OW, [10, 64, 10], &a, 1);
        assert_eq!(w.id, 1);
        assert!(!w.name.is_empty());
        assert!(r.knows("a", w.id));
        assert!(!r.knows("b", w.id));
        assert!(r.changed);
        // The same block placed again replaces the entry, with a new id.
        let w2 = r.place(OW, [10, 64, 10], &a, 2);
        assert_eq!(r.data.stones.len(), 1);
        assert_eq!(w2.id, 2);
        assert!(!r.knows("a", 1), "the old id is gone from the owner's list");
    }

    #[test]
    fn names_are_unique_and_run_out_into_numbers() {
        let mut r = reg();
        r.config.name_pool = vec!["Alpha".into(), "Beta".into()];
        let a = actor("a", 0);
        let names: Vec<String> = (0..4).map(|i| r.place(OW, [i, 0, 0], &a, 0).name).collect();
        assert!(names.contains(&"Alpha".to_owned()));
        assert!(names.contains(&"Beta".to_owned()));
        let set: std::collections::BTreeSet<_> = names.iter().collect();
        assert_eq!(set.len(), 4, "{names:?}");
        assert!(names.iter().any(|n| n.ends_with(" 2")));
    }

    #[test]
    fn only_the_owner_or_an_operator_breaks_it() {
        let mut r = reg();
        let (a, b, op) = (actor("a", 0), actor("b", 0), actor("op", 2));
        let w = r.place(OW, [0, 0, 0], &a, 0);
        r.activate("b", w.id);
        assert!(r.broken(OW, [0, 0, 0], &b).is_err());
        assert_eq!(r.at(OW, [0, 0, 0]).map(|w| w.id), Some(w.id));
        assert_eq!(r.broken(OW, [1, 0, 0], &b), Ok(None), "no stone there");
        assert_eq!(
            r.broken(OW, [0, 0, 0], &op).unwrap().map(|w| w.id),
            Some(w.id)
        );
        assert!(r.data.stones.is_empty());
        assert!(r.known("b").is_empty(), "removed from everyone's list");
        r.config.owner_only_break = false;
        let w = r.place(OW, [0, 0, 0], &a, 0);
        assert!(
            r.broken(OW, [0, 0, 0], &b)
                .unwrap()
                .is_some_and(|x| x.id == w.id)
        );
    }

    #[test]
    fn the_cost_grows_with_distance_up_to_a_limit() {
        let mut r = reg();
        let a = actor("a", 0);
        let w = r.place(OW, [0, 64, 0], &a, 0);
        let from = |x: f64| (OW, [x, 65.0, 0.5]);
        assert_eq!(r.cost(from(100.0), &w), 1, "under 500 blocks: the base");
        assert_eq!(r.cost(from(600.0), &w), 2, "one more per 500");
        assert_eq!(r.cost(from(1600.0), &w), 4);
        assert_eq!(r.cost(from(90000.0), &w), 5, "never past the limit");
        r.config.max_levels_cost = 0;
        assert_eq!(r.cost(from(5100.0), &w), 11, "no limit");
        r.config.blocks_per_level = 0;
        assert_eq!(r.cost(from(5100.0), &w), 1, "flat");
        // Across dimensions there is no distance; the limit still holds.
        r.config.max_levels_cost = 2;
        assert_eq!(r.cost((NETHER, [0.0, 0.0, 0.0]), &w), 2);
    }

    /// A small world for `landing_spot`: everything is air but what's listed;
    /// the stone's two blocks are solid.
    fn world(
        cells: Vec<([i32; 3], Cell)>,
        unloaded: Option<i32>,
    ) -> impl Fn([i32; 3]) -> Option<Cell> {
        move |pos: [i32; 3]| {
            if unloaded.is_some_and(|x| pos[0] >= x) {
                return None;
            }
            let listed = cells.iter().find(|(at, _)| *at == pos).map(|(_, c)| *c);
            Some(listed.unwrap_or(if pos[1] < 64 { Cell::Solid } else { Cell::Passable }))
        }
    }

    #[test]
    fn a_travel_lands_beside_the_stone_on_solid_ground() {
        let w = reg().place(OW, [0, 64, 0], &actor("a", 0), 0);
        // Open ground: the first side.
        assert_eq!(
            landing_spot(&w, &world(vec![], None)),
            Landing::At([0.5, 64.0, 1.5])
        );
        // A wall on that side, feet or head: the next.
        let wall = vec![([0, 65, 1], Cell::Solid)];
        assert_eq!(
            landing_spot(&w, &world(wall, None)),
            Landing::At([0.5, 64.0, -0.5])
        );
        // No ground there, or lava under it, or in it: not there.
        let drop = vec![
            ([0, 63, 1], Cell::Passable),
            ([0, 63, -1], Cell::Hazard),
            ([1, 64, 0], Cell::Hazard),
        ];
        assert_eq!(
            landing_spot(&w, &world(drop, None)),
            Landing::At([-0.5, 64.0, 0.5])
        );
    }

    #[test]
    fn a_stone_walled_in_lands_on_top_and_one_built_over_is_blocked() {
        let w = reg().place(OW, [0, 64, 0], &actor("a", 0), 0);
        let mut walls: Vec<([i32; 3], Cell)> = [[0, 1], [0, -1], [1, 0], [-1, 0]]
            .iter()
            .map(|[dx, dz]| ([*dx, 64, *dz], Cell::Solid))
            .collect();
        assert_eq!(
            landing_spot(&w, &world(walls.clone(), None)),
            Landing::At(landing(&w))
        );
        walls.push(([0, 67, 0], Cell::Solid));
        assert_eq!(landing_spot(&w, &world(walls, None)), Landing::Blocked);
    }

    #[test]
    fn an_unloaded_stone_is_unknown_not_blocked() {
        let w = reg().place(OW, [0, 64, 0], &actor("a", 0), 0);
        assert_eq!(landing_spot(&w, &world(vec![], Some(-100))), Landing::Unknown);
        // Solid sides, and the top in a chunk not loaded: unknown too.
        let walls = [[0, 1], [0, -1], [1, 0], [-1, 0]]
            .iter()
            .map(|[dx, dz]| ([*dx, 64, *dz], Cell::Solid))
            .collect();
        let only_sides = world(walls, None);
        let cell = |p: [i32; 3]| if p[1] >= 66 { None } else { only_sides(p) };
        assert_eq!(landing_spot(&w, &cell), Landing::Unknown);
    }

    #[test]
    fn a_cooldown_counts_whole_seconds_and_stops_the_screen() {
        assert_eq!(seconds_left(0, 5000), 0);
        assert_eq!(seconds_left(5000, 5000), 0);
        assert_eq!(seconds_left(5001, 5000), 1);
        assert_eq!(seconds_left(15000, 5000), 10);
        let mut u = ui(vec![entry(1, "Alpha"), entry(2, "Beta")]);
        let g = Geometry::new(u.window);
        u.cooldown = 4;
        u.click(centre(g.row(0)), 0);
        assert!(u.outbox.is_empty(), "a click waits for the cooldown");
        u.typed('B');
        u.key("key.keyboard.enter");
        assert!(u.outbox.is_empty(), "and so does Enter");
        let draws = u.render(centre(g.row(0)), false, &measure, &english);
        assert!(!draws.contains(&Draw::Fill(g.row(0), ROW_HOVER)));
        let wanted = english(&msg("wayfarer_stones.screen.cooldown", &[&4]));
        assert!(draws.iter().any(|d| matches!(d, Draw::Text(t, ..) if *t == wanted)));
        // Once it is over, a click travels.
        u.cooldown = 0;
        u.filter.clear();
        u.click(centre(g.row(0)), 0);
        assert_eq!(u.outbox[0], UiAction::Send(ClientAct::Travel(1)));
    }

    #[test]
    fn cost_depends_on_dimension_and_distance() {
        let mut r = reg();
        let a = actor("a", 0);
        let w = r.place(OW, [100, 64, 0], &a, 0);
        assert_eq!(r.cost((OW, [0.0, 64.0, 0.0]), &w), 1);
        assert_eq!(r.cost((NETHER, [0.0, 64.0, 0.0]), &w), 3);
        r.config.free_within_blocks = 200;
        assert_eq!(r.cost((OW, [0.0, 64.0, 0.0]), &w), 0);
        assert_eq!(r.cost((OW, [400.0, 64.0, 0.0]), &w), 1);
        r.config.xp_levels_cost = 0;
        assert_eq!(r.cost((NETHER, [0.0, 0.0, 0.0]), &w), 0);
    }

    #[test]
    fn commands_land_on_top_and_check_levels_with_a_terminal_condition() {
        let mut r = reg();
        let w = r.place(NETHER, [3, 70, -4], &actor("a", 0), 0);
        assert_eq!(
            travel_command("Alex", &w.dimension, landing(&w)),
            "execute in minecraft:the_nether run tp Alex 3.500 72.000 -3.500"
        );
        let check = level_check_command("Alex", 2);
        assert_eq!(check, "execute if entity @a[name=Alex,level=2..]");
        assert!(
            !check.contains(" run "),
            "a condition followed by run succeeds without running anything"
        );
        assert_eq!(
            consume_scroll_command("Alex"),
            "clear Alex wayfarer_stones:wayfarers_scroll 1"
        );
    }

    #[test]
    fn the_other_block_and_the_effects_stay_in_their_dimension() {
        assert_eq!(
            upper_block_command(NETHER, [1, 70, -2]),
            "execute in minecraft:the_nether run setblock 1 71 -2 wayfarer_stones:wayfarer_stone_top"
        );
        assert_eq!(
            remove_block_command(OW, [1, 71, -2]),
            "execute in minecraft:overworld run setblock 1 71 -2 minecraft:air"
        );
        let w = reg().place(NETHER, [3, 70, -4], &actor("a", 0), 0);
        let [particles, sound] = arrival_commands(&w.dimension, landing(&w));
        assert!(particles.starts_with("execute in minecraft:the_nether run particle minecraft:"));
        assert!(sound.contains("positioned 3.500 73.000 -3.500 run playsound minecraft:"));
        assert!(sound.contains("@a[distance=..32]"));
        for c in departure_commands(OW, [0.5, 64.0, 0.5])
            .iter()
            .chain(&activation_commands(&w))
        {
            assert!(c.starts_with("execute in minecraft:"), "{c}");
        }
    }

    #[test]
    fn here_is_a_known_stone_on_or_beside_which_the_player_stands() {
        let mut r = reg();
        let w = r.place(OW, [10, 64, 10], &actor("a", 0), 0);
        // On top of its two blocks, and on the ground beside them.
        assert_eq!(here_of(&r, "a", OW, [10.5, 66.0, 10.5]), Some(w.id));
        assert_eq!(here_of(&r, "a", OW, [11.5, 64.0, 10.5]), Some(w.id));
        assert_eq!(here_of(&r, "a", OW, [30.5, 65.0, 10.5]), None);
        assert_eq!(here_of(&r, "a", NETHER, [10.5, 65.0, 10.5]), None);
        assert_eq!(
            here_of(&r, "b", OW, [10.5, 65.0, 10.5]),
            None,
            "unknown to b"
        );
    }

    fn measure(text: &str) -> f32 {
        text.chars().count() as f32 * 6.0
    }

    fn entry(id: u64, name: &str) -> Entry {
        Entry {
            id,
            name: name.into(),
            dimension: OW.into(),
            pos: [0, 64, 0],
            owner_name: "a".into(),
            global: false,
            cost: 1,
            distance: Some(10.0),
            here: false,
            can_rename: true,
        }
    }

    fn ui(entries: Vec<Entry>) -> Ui {
        let total = entries.len();
        let mut u = Ui::new(ListMessage {
            open: true,
            entries,
            total,
            notice: None,
            scroll: false,
            cooldown_ms: 0,
        });
        u.window = [480.0, 270.0];
        u
    }

    fn centre(r: [f32; 4]) -> [f32; 2] {
        [r[0] + r[2] / 2.0, r[1] + r[3] / 2.0]
    }

    #[test]
    fn entries_say_what_the_player_may_do() {
        let mut r = reg();
        let (a, b) = (actor("a", 0), actor("b", 0));
        let w = r.place(OW, [0, 64, 0], &a, 0);
        r.activate("b", w.id);
        let e = entries(&r, &a, OW, [0.5, 65.0, 0.5], Some(w.id));
        assert!(e[0].here && e[0].can_rename);
        assert!(!entries(&r, &b, OW, [50.0, 65.0, 0.5], None)[0].can_rename);
    }

    #[test]
    fn a_long_list_fits_in_one_message_and_says_how_many_there_are() {
        let all: Vec<Entry> = (0..1000)
            .map(|i| entry(i, "A rather long stone name"))
            .collect();
        let bytes = encode_list(ListMessage {
            open: true,
            entries: all,
            total: 0,
            notice: None,
            scroll: false,
            cooldown_ms: 0,
        });
        assert!(bytes.len() <= MAX_MESSAGE, "{}", bytes.len());
        let back = decode_list(&bytes).unwrap();
        assert_eq!(back.total, 1000);
        assert!(back.entries.len() > 50 && back.entries.len() < 1000);
    }

    #[test]
    fn a_clients_request_round_trips_and_garbage_is_refused() {
        for act in [
            ClientAct::Travel(7),
            ClientAct::Rename {
                id: 7,
                name: "Дом".into(),
            },
        ] {
            assert_eq!(decode_act(&encode_act(&act)), Some(act));
        }
        assert_eq!(decode_act(b"{\"Travel\":\"x\"}"), None);
        assert_eq!(decode_list(b"not json"), None);
    }

    #[test]
    fn names_refuse_control_and_formatting_characters() {
        let mut r = reg();
        let a = actor("a", 0);
        let w = r.place(OW, [0, 64, 0], &a, 0);
        assert!(r.rename(w.id, "bad\nname", &a).is_err());
        assert!(r.rename(w.id, "\u{a7}cred", &a).is_err());
        assert!(r.rename(w.id, "Дом", &a).is_ok());
    }

    #[test]
    fn a_click_on_a_row_travels_and_closes() {
        let mut u = ui(vec![entry(1, "Alpha"), entry(2, "Beta")]);
        let g = Geometry::new(u.window);
        u.click(centre(g.row(1)), 0);
        assert_eq!(
            u.outbox,
            vec![UiAction::Send(ClientAct::Travel(2)), UiAction::Close]
        );
    }

    #[test]
    fn nothing_happens_for_the_stone_underfoot_an_empty_row_or_the_right_button() {
        let mut here = entry(1, "Alpha");
        here.here = true;
        let mut u = ui(vec![here, entry(2, "Beta")]);
        let g = Geometry::new(u.window);
        u.click(centre(g.row(0)), 0);
        u.click(centre(g.row(5)), 0);
        u.click(centre(g.row(1)), 1);
        assert!(u.outbox.is_empty());
    }

    #[test]
    fn the_close_button_closes() {
        let mut u = ui(vec![]);
        let g = Geometry::new(u.window);
        u.click(centre(g.close), 0);
        assert_eq!(u.outbox, vec![UiAction::Close]);
    }

    #[test]
    fn the_search_filters_in_any_case_and_keys_stay_on_the_screen() {
        let mut u = ui(vec![entry(1, "Дом"), entry(2, "Beta"), entry(3, "Better")]);
        for c in "BET".chars() {
            assert!(
                u.key("key.keyboard.e"),
                "a letter must not close the screen"
            );
            u.typed(c);
        }
        assert_eq!(u.visible().len(), 2);
        u.key("key.keyboard.backspace");
        u.key("key.keyboard.backspace");
        u.key("key.keyboard.backspace");
        u.typed('д');
        assert_eq!(u.visible()[0].id, 1);
        // One result: Enter travels there.
        u.key("key.keyboard.enter");
        assert_eq!(u.outbox[0], UiAction::Send(ClientAct::Travel(1)));
    }

    #[test]
    fn enter_with_several_results_does_nothing() {
        let mut u = ui(vec![entry(1, "Alpha"), entry(2, "Beta")]);
        u.key("key.keyboard.enter");
        assert!(u.outbox.is_empty());
    }

    #[test]
    fn renaming_saves_on_enter_and_leaves_on_escape() {
        let mut u = ui(vec![entry(1, "Alpha"), entry(2, "Beta")]);
        let g = Geometry::new(u.window);
        u.click(centre(g.pencil(1)), 0);
        assert_eq!(u.rename, Some((2, "Beta".to_owned())));
        assert!(u.outbox.is_empty(), "the pencil doesn't travel");
        u.key("key.keyboard.backspace");
        u.typed(' ');
        u.typed('!');
        u.key("key.keyboard.enter");
        assert_eq!(u.rename, None);
        assert_eq!(
            u.outbox,
            vec![UiAction::Send(ClientAct::Rename {
                id: 2,
                name: "Bet !".to_owned()
            })]
        );
        // Escape leaves the rename, and only then closes the screen.
        u.click(centre(g.pencil(0)), 0);
        assert!(u.key("key.keyboard.escape"));
        assert_eq!(u.rename, None);
        assert!(!u.key("key.keyboard.escape"));
    }

    #[test]
    fn a_click_elsewhere_leaves_a_rename_without_travelling() {
        let mut u = ui(vec![entry(1, "Alpha"), entry(2, "Beta")]);
        let g = Geometry::new(u.window);
        u.click(centre(g.pencil(0)), 0);
        u.click(centre(g.row(1)), 0);
        assert_eq!(u.rename, None);
        assert!(u.outbox.is_empty());
    }

    #[test]
    fn typed_names_stop_at_the_limit() {
        let mut u = ui(vec![entry(1, "")]);
        u.rename = Some((1, String::new()));
        for _ in 0..50 {
            u.typed('я');
        }
        u.typed('\n');
        assert_eq!(u.rename.unwrap().1.chars().count(), MAX_NAME);
    }

    #[test]
    fn the_wheel_scrolls_within_the_list_and_adds_up_a_touchpad() {
        let mut u = ui((0..20).map(|i| entry(i, "W")).collect());
        let rows = Geometry::new(u.window).rows;
        u.wheel(5.0);
        assert_eq!(u.scroll, 0);
        u.wheel(-0.4);
        u.wheel(-0.4);
        assert_eq!(u.scroll, 0);
        u.wheel(-0.4);
        assert_eq!(u.scroll, 1);
        u.wheel(-100.0);
        assert_eq!(u.scroll, 20 - rows);
        // A click lands on the row shown, not the first of the list.
        let g = Geometry::new(u.window);
        u.click(centre(g.row(0)), 0);
        assert_eq!(
            u.outbox[0],
            UiAction::Send(ClientAct::Travel((20 - rows) as u64))
        );
    }

    #[test]
    fn a_refresh_keeps_the_screen_consistent() {
        let mut u = ui((0..20).map(|i| entry(i, "W")).collect());
        u.wheel(-100.0);
        u.rename = Some((19, "x".into()));
        let no = msg("wayfarer_stones.error.no_such", &[]);
        u.refresh(ListMessage {
            open: false,
            entries: vec![entry(1, "Only")],
            total: 1,
            notice: Some(no.clone()),
            scroll: true,
            cooldown_ms: 0,
        });
        assert_eq!(u.scroll, 0);
        assert_eq!(u.rename, None, "the stone being renamed is gone");
        assert_eq!(u.notice, Some(no));
        assert!(u.needs_scroll);
    }

    #[test]
    fn the_panel_fits_small_windows_and_grows_to_a_limit() {
        for window in [
            [320.0, 120.0],
            [320.0, 240.0],
            [480.0, 270.0],
            [960.0, 540.0],
        ] {
            let g = Geometry::new(window);
            assert!(g.rows >= 1 && g.rows <= MAX_ROWS);
            assert!(g.panel[0] >= 0.0 && g.panel[1] >= 0.0);
            assert!(g.panel[0] + g.panel[2] <= window[0]);
        }
        assert_eq!(Geometry::new([320.0, 240.0]).rows, 6);
        assert_eq!(Geometry::new([960.0, 540.0]).rows, MAX_ROWS);
        // The tiniest window still shows all of a one-row panel if it can.
        assert!(Geometry::new([320.0, 120.0]).panel[3] <= 120.0);
    }

    #[test]
    fn text_is_cut_to_fit() {
        assert_eq!(fit("short", 100.0, &measure), "short");
        let cut = fit("a very long name indeed", 60.0, &measure);
        assert!(cut.ends_with(".."));
        assert!(measure(&cut) <= 60.0);
        assert_eq!(fit("Дом милый дом", 30.0, &measure), "Дом..");
    }

    #[test]
    fn drawing_shows_the_row_and_stays_on_the_panel() {
        let mut far = entry(2, "Beta");
        far.global = true;
        far.cost = 3;
        let mut u = ui(vec![entry(1, "Alpha"), far]);
        u.window = [480.0, 270.0];
        let g = Geometry::new(u.window);
        let draws = u.render(centre(g.row(0)), true, &measure, &english);
        let texts: Vec<&str> = draws
            .iter()
            .filter_map(|d| match d {
                Draw::Text(t, ..) => Some(t.as_str()),
                _ => None,
            })
            .collect();
        for wanted in [
            "Wayfarer Stones",
            "2 known",
            "Alpha",
            "Beta",
            "1 level",
            "3 levels",
            "10 m",
        ] {
            assert!(texts.contains(&wanted), "{wanted} in {texts:?}");
        }
        assert!(draws.contains(&Draw::Fill(g.row(0), ROW_HOVER)));
        assert!(draws.contains(&Draw::Item(BLOCK, [g.list[0] + 4.0, g.list[1] + 4.0], 16.0)));
        for d in &draws {
            if let Draw::Text(t, at, _) = d {
                assert!(at[0] >= g.panel[0], "{t} left of the panel");
                assert!(
                    at[0] + measure(t) <= g.panel[0] + g.panel[2],
                    "{t} right of the panel"
                );
                // A line of the font is 8 high, and stops short of the carved edge.
                assert!(
                    at[1] + 8.0 <= g.panel[1] + g.panel[3] - BORDER,
                    "{t} on the bottom edge"
                );
            }
        }
    }

    #[test]
    fn the_last_line_says_what_a_click_costs() {
        let line = |u: &Ui| {
            u.render([0.0, 0.0], false, &measure, &english)
                .into_iter()
                .filter_map(|d| match d {
                    Draw::Text(t, ..) => Some(t),
                    _ => None,
                })
                .last()
        };
        let mut u = ui(vec![entry(1, "Alpha")]);
        assert_eq!(line(&u), Some(english(&msg("wayfarer_stones.screen.hint", &[]))));
        u.needs_scroll = true;
        assert_eq!(
            line(&u),
            Some(english(&msg("wayfarer_stones.screen.hint_scroll", &[])))
        );
    }

    #[test]
    fn an_empty_screen_says_why() {
        let draws = ui(vec![]).render([0.0, 0.0], false, &measure, &english);
        let line = english(&msg("wayfarer_stones.screen.empty", &[]));
        let line = fit(&line, Geometry::new([480.0, 270.0]).list[2] - 8.0, &measure);
        assert!(draws.iter().any(|d| matches!(d, Draw::Text(t, ..) if *t == line)));
    }

    #[test]
    fn commands_open_list_travel_and_rename() {
        let mut r = reg();
        let (a, b, op) = (actor("a", 0), actor("b", 0), actor("op", 2));
        let w = r.place(OW, [10, 64, 10], &a, 0);
        let at = [10.5, 65.0, 10.5];
        assert_eq!(r.command(&a, OW, at, "wayfarer"), vec![Out::Open]);
        assert!(r.command(&a, OW, at, "pay x 1").is_empty(), "not ours");
        // Rename by the owner standing near; a stranger can't.
        assert_eq!(
            r.command(&a, OW, at, "wayfarer name  Home Base "),
            vec![Out::Reply(msg("wayfarer_stones.renamed", &[&"Home Base"]))]
        );
        assert!(matches!(
            r.command(&b, OW, at, "wayfarer name Theirs")[0],
            Out::Error(_)
        ));
        assert!(matches!(
            r.command(&a, OW, [100.0, 64.0, 0.0], "wayfarer name Far")[0],
            Out::Error(_)
        ));
        // Travel needs activation; b activates and goes.
        assert!(matches!(
            r.command(&b, OW, at, "wayfarer to home base")[0],
            Out::Error(_)
        ));
        assert!(r.activate("b", w.id));
        let far = [500.5, 65.0, 500.5];
        assert_eq!(
            r.command(&b, OW, far, "wayfarer to home base"),
            vec![Out::Travel {
                id: w.id,
                scroll: true
            }]
        );
        assert_eq!(r.command(&b, OW, at, "wayfarer list").len(), 2);
        // Forget, global (operators only), remove.
        assert_eq!(
            r.command(&b, OW, at, "wayfarer forget Home Base"),
            vec![Out::Reply(msg("wayfarer_stones.forgot", &[&"Home Base"]))]
        );
        assert!(matches!(
            r.command(&b, OW, at, "wayfarer global Home Base")[0],
            Out::Error(_)
        ));
        assert!(matches!(
            r.command(&op, OW, at, "wayfarer global Home Base")[0],
            Out::Reply(_)
        ));
        assert!(r.knows("b", w.id), "global: everyone knows it");
        assert!(matches!(
            r.command(&b, OW, at, "wayfarer forget Home Base")[0],
            Out::Error(_)
        ));
        assert!(matches!(
            r.command(&op, OW, at, "wayfarer remove Home Base")[0],
            Out::Reply(_)
        ));
        assert!(r.data.stones.is_empty());
    }

    #[test]
    fn a_travel_away_from_a_stone_takes_a_scroll() {
        let mut r = reg();
        let a = actor("a", 0);
        let home = r.place(OW, [0, 64, 0], &a, 0);
        let mine = r.place(OW, [100, 64, 0], &a, 0);
        let (at_home, far) = ([0.5, 66.0, 0.5], [500.5, 65.0, 0.5]);
        let to = format!("wayfarer to {}", mine.name);
        // At a stone, levels alone.
        assert_eq!(
            r.command(&a, OW, at_home, &to),
            vec![Out::Travel {
                id: mine.id,
                scroll: false
            }]
        );
        assert!(!r.needs_scroll(here_of(&r, "a", OW, at_home)));
        // Away from one, a scroll.
        assert_eq!(
            r.command(&a, OW, far, &to),
            vec![Out::Travel {
                id: mine.id,
                scroll: true
            }]
        );
        assert!(r.needs_scroll(here_of(&r, "a", OW, far)));
        // A stone the player doesn't know is no place to start from.
        let b = actor("b", 0);
        r.activate("b", mine.id);
        assert_eq!(
            r.command(&b, OW, at_home, &to),
            vec![Out::Travel {
                id: mine.id,
                scroll: true
            }]
        );
        // Without scrolls in this world, only from a stone.
        r.config.wayfarers_scroll = false;
        assert_eq!(
            r.command(&a, OW, far, &to),
            vec![Out::Error(msg("wayfarer_stones.error.stand_at_stone", &[]))]
        );
        // The stone underfoot isn't a travel.
        let here = format!("wayfarer to {}", home.name);
        assert_eq!(
            r.command(&a, OW, at_home, &here),
            vec![Out::Error(msg("wayfarer_stones.error.already_here", &[]))]
        );
        // The config may open it all up, as 0.3 was.
        r.config.travel_anywhere = true;
        assert_eq!(
            r.command(&a, OW, far, &to),
            vec![Out::Travel {
                id: mine.id,
                scroll: false
            }]
        );
    }

    #[test]
    fn a_village_stone_joins_with_no_owner_and_is_anybodys_to_break() {
        let mut r = reg();
        let (a, op) = (actor("a", 0), actor("op", 2));
        let w = r.discover(OW, [5, 70, 5], 9);
        assert_eq!(w.owner, NO_OWNER);
        assert!(!r.knows("a", w.id), "a player still activates it");
        assert!(r.changed);
        assert_eq!(
            r.rename(w.id, "Mine", &a),
            Err(msg("wayfarer_stones.error.only_operator_renames", &[]))
        );
        assert!(r.rename(w.id, "Square", &op).is_ok());
        assert_eq!(r.at(OW, [5, 70, 5]).map(|w| w.name.as_str()), Some("Square"));
        assert!(r.broken(OW, [5, 70, 5], &a).unwrap().is_some());
        // Another stone never gets the same id.
        assert_ne!(r.discover(OW, [6, 70, 5], 9).id, w.id);
    }

    #[test]
    fn the_limit_counts_only_the_owners_stones() {
        let mut r = reg();
        r.config.max_stones_per_player = 1;
        let (a, op) = (actor("a", 0), actor("op", 2));
        assert!(r.may_place(&a).is_ok());
        r.place(OW, [0, 0, 0], &a, 0);
        assert!(r.may_place(&a).is_err());
        assert!(r.may_place(&op).is_ok(), "operators are not limited");
        r.config.max_stones_per_player = 0;
        assert!(r.may_place(&a).is_ok());
    }

    #[test]
    fn one_click_opens_one_menu() {
        let click = |uuid, here| OpenRequest { uuid, here };
        let mut q = OpenQueue::default();
        assert!(q.is_empty());
        // Both hands in one tick, and the scroll's request after them: the first stands.
        q.request(click(7, Some(1)));
        q.request(click(7, Some(1)));
        q.request(click(7, None));
        q.request(click(8, None));
        assert!(!q.is_empty());
        assert_eq!(q.take(100), vec![click(7, Some(1)), click(8, None)]);
        assert!(q.is_empty());
        // The same click reaching the hook on the next tick opens nothing.
        q.request(click(7, Some(1)));
        assert_eq!(q.take(101), vec![]);
        // A later click does.
        q.request(click(7, Some(1)));
        assert_eq!(q.take(100 + REOPEN_TICKS), vec![click(7, Some(1))]);
    }

    #[test]
    fn warmups_notice_a_move() {
        let from = (OW.to_owned(), [0.0, 64.0, 0.0]);
        assert!(!moved(&from, (OW, [0.5, 64.0, 0.5])));
        assert!(moved(&from, (OW, [2.0, 64.0, 0.0])));
        assert!(moved(&from, (NETHER, [0.0, 64.0, 0.0])));
    }

    #[test]
    fn config_round_trips_with_defaults_for_missing_keys() {
        let c: Config = serde_json::from_str(r#"{"xp_levels_cost": 5}"#).unwrap();
        assert_eq!(c.xp_levels_cost, 5);
        assert_eq!(c.cross_dimension_multiplier, 3);
        assert!(!c.name_pool.is_empty());
        let back: Config = serde_json::from_str(&serde_json::to_string(&c).unwrap()).unwrap();
        assert_eq!(back, c);
    }
}
