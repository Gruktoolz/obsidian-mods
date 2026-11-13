// Chop! (c) 2026 Mayonesu0. All rights reserved, except the grants in LICENSE: https://github.com/Mayonesu0/chop/blob/main/LICENSE

// Chop! — the rules (game::chop_core).
//
// Which blocks make the tree, which way it falls, how it moves and what it hits.
// Nothing here knows Horizon: the game's side (game::chop_server) reads the level
// through `World` and carries out what these functions decide, and the tests run
// them on worlds built by hand.
//
// A felling: the cut block shakes and pops; the trunk drops into the gap it left,
// then turns about the front edge of its base under gravity as one rigid body. It
// starts slowly, speeds up and stops where its trunk meets the ground or another
// trunk (which may fall in turn), crushing the leaves caught under it; it bounces a
// little, lies there, then pops away block by block from the base to the top.

pub use chop::*;

#[allow(dead_code)]
mod chop {
    use std::collections::{HashMap, HashSet, VecDeque};
    use std::f64::consts::PI;
    use std::hash::{BuildHasherDefault, Hasher};

    pub type Pos = [i32; 3];

    /// A quick hash for block positions (the contact search reads hundreds of
    /// thousands of them).
    #[derive(Clone, Copy, Default)]
    pub struct QuickHash(u64);

    impl QuickHash {
        fn mix(&mut self, v: u64) {
            self.0 = (self.0.rotate_left(5) ^ v).wrapping_mul(0x51_7C_C1_B7_27_22_0A_95);
        }
    }

    impl Hasher for QuickHash {
        fn finish(&self) -> u64 {
            self.0
        }
        fn write(&mut self, bytes: &[u8]) {
            for b in bytes {
                self.mix(u64::from(*b));
            }
        }
        fn write_i32(&mut self, i: i32) {
            self.mix(u64::from(i as u32));
        }
        fn write_usize(&mut self, i: usize) {
            self.mix(i as u64);
        }
    }

    pub type QuickMap<K, V> = HashMap<K, V, BuildHasherDefault<QuickHash>>;
    pub type QuickSet<K> = HashSet<K, BuildHasherDefault<QuickHash>>;

    // -----------------------------------------------------------------------------
    // Settings
    // -----------------------------------------------------------------------------

    /// The menu's pages.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Section {
        Fall,
        Effects,
        Damage,
        Rules,
    }

    impl Section {
        pub const ALL: [Section; 4] = [
            Section::Fall,
            Section::Effects,
            Section::Damage,
            Section::Rules,
        ];
        /// The word of `/chop config <page>`.
        pub fn word(self) -> &'static str {
            match self {
                Section::Fall => "chute",
                Section::Effects => "effets",
                Section::Damage => "degats",
                Section::Rules => "regles",
            }
        }
        pub fn title(self) -> &'static str {
            match self {
                Section::Fall => "Chute",
                Section::Effects => "Effets",
                Section::Damage => "Dégâts",
                Section::Rules => "Règles",
            }
        }
        pub fn from_word(word: &str) -> Option<Section> {
            Section::ALL.into_iter().find(|s| s.word() == word)
        }
    }

    /// How a setting's value reads.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Unit {
        Switch,
        Percent,
        Degrees,
        /// Tenths of a second.
        Tenths,
        /// Health points (half hearts).
        Health,
        Count,
        Blocks,
        /// Trees in a row, 0 for none.
        Trees,
        Choice(&'static [&'static str]),
    }

    impl Unit {
        pub fn show(self, v: i32) -> String {
            match self {
                Unit::Switch => (if v != 0 { "oui" } else { "non" }).to_owned(),
                Unit::Percent => format!("{v} %"),
                Unit::Degrees => format!("±{v}°"),
                Unit::Tenths => {
                    if v % 10 == 0 {
                        format!("{} s", v / 10)
                    } else {
                        format!("{},{} s", v / 10, v % 10)
                    }
                }
                Unit::Health => {
                    if v % 2 == 0 {
                        format!("{} ❤", v / 2)
                    } else {
                        format!("{},5 ❤", v / 2)
                    }
                }
                Unit::Count => v.to_string(),
                Unit::Trees => match v {
                    0 => "non".to_owned(),
                    1 => "1 arbre".to_owned(),
                    n => format!("{n} arbres"),
                },
                Unit::Blocks => format!("{v} blocs"),
                Unit::Choice(names) => names.get(v as usize).copied().unwrap_or("?").to_owned(),
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum Key {
        Active,
        Animation,
        Speed,
        Aim,
        Cone,
        Rest,
        Volume,
        Particles,
        Leaves,
        Dust,
        Damage,
        Strength,
        Monsters,
        Animals,
        Allies,
        Players,
        Axe,
        Sneak,
        Wear,
        Loot,
        MaxLogs,
        MaxBlocks,
        Radius,
        Protect,
        Chain,
        Shake,
        Replant,
        Preview,
        ShakeTree,
        Fatigue,
        Bees,
        Birds,
        Apples,
        Fungi,
    }

    /// One setting: its command word, its line in the menu and its range.
    #[derive(Clone, Copy, Debug)]
    pub struct Opt {
        pub key: Key,
        pub word: &'static str,
        pub label: &'static str,
        pub help: &'static str,
        pub section: Section,
        pub min: i32,
        pub max: i32,
        pub step: i32,
        pub default: i32,
        pub unit: Unit,
    }

    pub const PARTICLES: &[&str] = &["aucune", "peu", "normales", "beaucoup"];
    pub const SNEAK: &[&str] = &[
        "accroupi = une bûche",
        "accroupi pour abattre",
        "toujours abattre",
    ];
    pub const LOOT: &[&str] = &["là où il tombe", "aux pieds du joueur", "dans l'inventaire"];
    pub const REPLANT: &[&str] = &["non", "une pousse du butin", "toujours"];

    #[allow(clippy::too_many_arguments)]
    const fn opt(
        key: Key,
        word: &'static str,
        label: &'static str,
        help: &'static str,
        section: Section,
        range: (i32, i32, i32),
        default: i32,
        unit: Unit,
    ) -> Opt {
        Opt {
            key,
            word,
            label,
            help,
            section,
            min: range.0,
            max: range.1,
            step: range.2,
            default,
            unit,
        }
    }

    const SW: (i32, i32, i32) = (0, 1, 1);

    /// Every setting, in `Key` order.
    pub const OPTS: [Opt; 34] = [
        opt(
            Key::Active,
            "actif",
            "Chop! sur ce monde",
            "Non : plus aucun arbre n'est abattu, pour tout le monde.",
            Section::Rules,
            SW,
            1,
            Unit::Switch,
        ),
        opt(
            Key::Animation,
            "animation",
            "Animation de chute",
            "Non : l'arbre se brise sur place, d'un coup.",
            Section::Fall,
            SW,
            1,
            Unit::Switch,
        ),
        opt(
            Key::Speed,
            "vitesse",
            "Vitesse de chute",
            "La gravité de la chute : 100 % est réaliste, plus va plus vite.",
            Section::Fall,
            (50, 400, 25),
            200,
            Unit::Percent,
        ),
        opt(
            Key::Aim,
            "devant",
            "Tombe devant le joueur",
            "Non : l'arbre tombe du côté le plus dégagé, où qu'il soit.",
            Section::Fall,
            SW,
            1,
            Unit::Switch,
        ),
        opt(
            Key::Cone,
            "angle",
            "Angle de chute",
            "Écart permis avec le regard du joueur : l'arbre prend la direction la plus dégagée dans cet angle. 0 : exactement où il regarde.",
            Section::Fall,
            (0, 90, 5),
            30,
            Unit::Degrees,
        ),
        opt(
            Key::Rest,
            "pose",
            "Temps couché au sol",
            "Combien de temps l'arbre reste couché avant de se briser (en dixièmes de seconde).",
            Section::Fall,
            (0, 100, 5),
            25,
            Unit::Tenths,
        ),
        opt(
            Key::Volume,
            "sons",
            "Volume des sons",
            "Craquement, chute, impact, bris.",
            Section::Effects,
            (0, 200, 25),
            100,
            Unit::Percent,
        ),
        opt(
            Key::Particles,
            "particules",
            "Particules",
            "Copeaux, éclats, bris.",
            Section::Effects,
            (0, 3, 1),
            2,
            Unit::Choice(PARTICLES),
        ),
        opt(
            Key::Leaves,
            "feuilles",
            "Feuilles qui virevoltent",
            "Des feuilles tombent de la couronne après l'impact.",
            Section::Effects,
            SW,
            1,
            Unit::Switch,
        ),
        opt(
            Key::Dust,
            "poussiere",
            "Poussière à l'impact",
            "L'impact soulève le bloc de sol touché (terre, sable, neige...).",
            Section::Effects,
            SW,
            1,
            Unit::Switch,
        ),
        opt(
            Key::Damage,
            "degats",
            "Dégâts de l'arbre",
            "L'arbre qui tombe blesse et repousse ce qu'il percute, selon sa vitesse.",
            Section::Damage,
            SW,
            1,
            Unit::Switch,
        ),
        opt(
            Key::Strength,
            "force",
            "Dégâts maximum",
            "Ce qu'un coup de tronc lancé à pleine vitesse enlève au plus (en points de vie).",
            Section::Damage,
            (1, 40, 1),
            10,
            Unit::Health,
        ),
        opt(
            Key::Monsters,
            "monstres",
            "Blesse les monstres",
            "Zombies, squelettes, creepers, araignées...",
            Section::Damage,
            SW,
            1,
            Unit::Switch,
        ),
        opt(
            Key::Animals,
            "animaux",
            "Blesse les animaux",
            "Les créatures passives et neutres qui ne sont à personne.",
            Section::Damage,
            SW,
            0,
            Unit::Switch,
        ),
        opt(
            Key::Allies,
            "allies",
            "Blesse les alliés",
            "Animaux apprivoisés, golems, villageois, allays.",
            Section::Damage,
            SW,
            0,
            Unit::Switch,
        ),
        opt(
            Key::Players,
            "joueurs",
            "Blesse les joueurs",
            "Le bûcheron compris.",
            Section::Damage,
            SW,
            0,
            Unit::Switch,
        ),
        opt(
            Key::Axe,
            "hache",
            "Hache obligatoire",
            "Non : n'importe quoi en main abat l'arbre.",
            Section::Rules,
            SW,
            1,
            Unit::Switch,
        ),
        opt(
            Key::Sneak,
            "accroupi",
            "Accroupi",
            "Ce que fait le fait d'être accroupi en coupant.",
            Section::Rules,
            (0, 2, 1),
            0,
            Unit::Choice(SNEAK),
        ),
        opt(
            Key::Wear,
            "usure",
            "Usure de la hache",
            "La hache s'use d'un point par bûche (Solidité comprise) ; trop usée, elle ne coupe qu'une bûche.",
            Section::Rules,
            SW,
            1,
            Unit::Switch,
        ),
        opt(
            Key::Loot,
            "butin",
            "Butin",
            "Où va ce que donne l'arbre : là où il tombe, à tes pieds, ou dans ton inventaire (le surplus à tes pieds).",
            Section::Rules,
            (0, 2, 1),
            0,
            Unit::Choice(LOOT),
        ),
        opt(
            Key::MaxLogs,
            "max_buches",
            "Bûches maximum",
            "Un arbre plus grand se coupe bûche par bûche.",
            Section::Rules,
            (16, 512, 16),
            256,
            Unit::Count,
        ),
        opt(
            Key::MaxBlocks,
            "max_blocs",
            "Blocs maximum",
            "Bûches et feuilles comprises.",
            Section::Rules,
            (100, 2000, 100),
            1200,
            Unit::Count,
        ),
        opt(
            Key::Radius,
            "rayon",
            "Rayon maximum",
            "Jusqu'où une branche peut s'écarter de la bûche coupée.",
            Section::Rules,
            (3, 24, 1),
            12,
            Unit::Blocks,
        ),
        opt(
            Key::Protect,
            "protection",
            "Protéger les bûches posées",
            "Une construction en bûches posées par un joueur ne tombe jamais.",
            Section::Rules,
            SW,
            1,
            Unit::Switch,
        ),
        opt(
            Key::Chain,
            "chaine",
            "Réaction en chaîne",
            "Quand le tronc qui tombe percute le tronc d'un autre arbre, celui-ci tombe à son tour, et ainsi de suite : combien d'arbres d'affilée au plus (0 : non).",
            Section::Fall,
            (0, 10, 1),
            3,
            Unit::Trees,
        ),
        opt(
            Key::Shake,
            "secousse",
            "Bûche qui tremble",
            "La bûche tremble et gonfle pendant que tu la coupes, de plus en plus fort, puis éclate quand elle cède.",
            Section::Effects,
            SW,
            1,
            Unit::Switch,
        ),
        opt(
            Key::Replant,
            "replanter",
            "Replanter",
            "Une fois l'arbre parti, une pousse de son essence est replantée où il se tenait (sur de la terre). « Une pousse du butin » la prend dans ce que l'arbre a donné ; « toujours » l'offre.",
            Section::Rules,
            (0, 2, 1),
            1,
            Unit::Choice(REPLANT),
        ),
        opt(
            Key::Preview,
            "apercu",
            "Aperçu de la chute",
            "Pendant que tu coupes, une ligne de petites particules au sol montre où l'arbre va tomber.",
            Section::Effects,
            SW,
            1,
            Unit::Switch,
        ),
        opt(
            Key::ShakeTree,
            "secouer",
            "Secouer les arbres",
            "Un clic droit à main nue sur le tronc secoue l'arbre : quelques pommes, bâtons ou pousses tombent (une fois par arbre toutes les 30 s).",
            Section::Rules,
            SW,
            1,
            Unit::Switch,
        ),
        opt(
            Key::Fatigue,
            "fatigue",
            "Fatigue du bûcheron",
            "Abattre un arbre creuse un peu la faim, selon le nombre de bûches.",
            Section::Rules,
            SW,
            0,
            Unit::Switch,
        ),
        opt(
            Key::Bees,
            "abeilles",
            "Abeilles en colère",
            "Un nid d'abeilles qui tombe sans Toucher de soie relâche ses abeilles, en colère, comme en vanilla.",
            Section::Effects,
            SW,
            1,
            Unit::Switch,
        ),
        opt(
            Key::Birds,
            "oiseaux",
            "Oiseaux qui s'envolent",
            "Quand un grand arbre commence à basculer, des oiseaux s'envolent de la couronne en pépiant.",
            Section::Effects,
            SW,
            1,
            Unit::Switch,
        ),
        opt(
            Key::Apples,
            "pommes",
            "Pommes qui tombent",
            "Les pommes de l'arbre se détachent pendant la chute et roulent au sol.",
            Section::Effects,
            SW,
            1,
            Unit::Switch,
        ),
        opt(
            Key::Fungi,
            "champignons",
            "Champignons géants",
            "Les champignons géants et les arbres du Nether (carmin, biscornus) tombent aussi.",
            Section::Rules,
            SW,
            1,
            Unit::Switch,
        ),
    ];

    impl Key {
        pub fn opt(self) -> &'static Opt {
            &OPTS[self as usize]
        }
    }

    pub fn opt_by_word(word: &str) -> Option<&'static Opt> {
        OPTS.iter().find(|o| o.word == word)
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Config {
        values: [i32; 34],
    }

    impl Default for Config {
        fn default() -> Self {
            let mut values = [0; 34];
            for o in &OPTS {
                values[o.key as usize] = o.default;
            }
            Config { values }
        }
    }

    impl Config {
        pub fn get(&self, key: Key) -> i32 {
            self.values[key as usize]
        }
        pub fn on(&self, key: Key) -> bool {
            self.get(key) != 0
        }
        /// Sets the setting named `word`: the setting, or why not.
        pub fn set(&mut self, word: &str, value: i32) -> Result<&'static Opt, String> {
            let Some(o) = opt_by_word(word) else {
                return Err(format!("pas de réglage « {word} »"));
            };
            if !(o.min..=o.max).contains(&value) {
                return Err(format!("{} va de {} à {}", o.word, o.min, o.max));
            }
            self.values[o.key as usize] = value;
            Ok(o)
        }
        /// The values that differ from the defaults, one `word = value` a line.
        pub fn to_lines(&self) -> Vec<String> {
            OPTS.iter()
                .filter(|o| self.get(o.key) != o.default)
                .map(|o| format!("{} = {}", o.word, self.get(o.key)))
                .collect()
        }
        /// Takes a `word = value` line; anything else is left alone.
        pub fn read_line(&mut self, line: &str) {
            if let Some((k, v)) = line.split_once('=')
                && let Ok(v) = v.trim().parse::<i32>()
            {
                let _ = self.set(k.trim(), v);
            }
        }
    }

    /// Whether a cut fells the tree, from who cuts and how.
    pub fn fells(config: &Config, game_mode: i32, sneaking: bool, axe: bool) -> bool {
        if !config.on(Key::Active) || game_mode == 2 || game_mode == 3 {
            return false;
        }
        let sneak_ok = match config.get(Key::Sneak) {
            0 => !sneaking,
            1 => sneaking,
            _ => true,
        };
        sneak_ok && (axe || !config.on(Key::Axe))
    }

    /// What a tree of logs `wood` (`oak`, `dark_oak`, `crimson`, `mushroom`...) and
    /// leaves or caps `leaves` (`minecraft:azalea_leaves`, `minecraft:red_mushroom_block`...)
    /// grows from: the block planted, also the item.
    pub fn sapling(wood: &str, leaves: Option<&str>) -> Option<&'static str> {
        match leaves.map(|l| l.trim_start_matches("minecraft:")) {
            Some("azalea_leaves") => return Some("minecraft:azalea"),
            Some("flowering_azalea_leaves") => return Some("minecraft:flowering_azalea"),
            _ => {}
        }
        Some(match wood {
            "oak" => "minecraft:oak_sapling",
            "spruce" => "minecraft:spruce_sapling",
            "birch" => "minecraft:birch_sapling",
            "jungle" => "minecraft:jungle_sapling",
            "acacia" => "minecraft:acacia_sapling",
            "dark_oak" => "minecraft:dark_oak_sapling",
            "cherry" => "minecraft:cherry_sapling",
            "pale_oak" => "minecraft:pale_oak_sapling",
            "mangrove" => "minecraft:mangrove_propagule",
            "crimson" => "minecraft:crimson_fungus",
            "warped" => "minecraft:warped_fungus",
            "mushroom" => match leaves.map(|l| l.trim_start_matches("minecraft:")) {
                Some("red_mushroom_block") => "minecraft:red_mushroom",
                Some("brown_mushroom_block") => "minecraft:brown_mushroom",
                _ => return None,
            },
            _ => return None,
        })
    }

    // -----------------------------------------------------------------------------
    // Randomness (effects, wear)
    // -----------------------------------------------------------------------------

    /// xorshift64*.
    #[derive(Clone, Debug)]
    pub struct Rng(u64);

    impl Rng {
        pub fn new(seed: u64) -> Rng {
            Rng(seed | 1)
        }
        pub fn next_u64(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x >> 12;
            x ^= x << 25;
            x ^= x >> 27;
            self.0 = x;
            x.wrapping_mul(0x2545_F491_4F6C_DD1D)
        }
        /// 0 to `n - 1`.
        pub fn below(&mut self, n: u32) -> u32 {
            if n <= 1 {
                0
            } else {
                (self.next_u64() >> 33) as u32 % n
            }
        }
        /// 0 to 1.
        pub fn unit(&mut self) -> f64 {
            (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
        }
        /// `-r` to `r`.
        pub fn spread(&mut self, r: f64) -> f64 {
            (self.unit() * 2.0 - 1.0) * r
        }
    }

    /// The points of durability `logs` logs take with Unbreaking `unbreaking`: each
    /// one is spared with chance level / (level + 1), as a tool's use is.
    pub fn wear(logs: u32, unbreaking: u32, rng: &mut Rng) -> u32 {
        (0..logs).filter(|_| rng.below(unbreaking + 1) == 0).count() as u32
    }

    // -----------------------------------------------------------------------------
    // The level as the rules see it
    // -----------------------------------------------------------------------------

    /// What holds a hanging block in place.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Attach {
        /// Hangs under its support (hanging moss, a hanging propagule).
        Below,
        /// Sits on it (a snow layer).
        Above,
        /// On any face (vines, cocoa, a bee nest, lichen).
        Side,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Cell {
        /// Nothing a falling tree hits: air, plants, flowers, fluids, carpets.
        Open,
        /// Something it does hit.
        Solid,
        /// A log or wood block; `wood` names its species.
        Log { wood: u16 },
        /// Leaves with vanilla's `distance` (1 to 7) and `persistent`.
        Leaves { distance: u8, persistent: bool },
        /// A crown block with no `distance`, all of it joined to the stem: a
        /// nether wart block, a shroomlight, a mushroom cap.
        Cap,
        /// Breaks off when its support goes.
        Hanger(Attach),
    }

    pub trait World {
        /// The cell at `p`; None where the level isn't loaded. Above and below the
        /// world it is `Open`.
        fn cell(&mut self, p: Pos) -> Option<Cell>;
    }

    /// The level, each cell read once.
    pub struct Look<'a> {
        world: &'a mut dyn World,
        seen: QuickMap<Pos, Option<Cell>>,
    }

    impl<'a> Look<'a> {
        pub fn new(world: &'a mut dyn World) -> Look<'a> {
            Look {
                world,
                seen: QuickMap::default(),
            }
        }
        pub fn cell(&mut self, p: Pos) -> Option<Cell> {
            if let Some(c) = self.seen.get(&p) {
                return *c;
            }
            let c = self.world.cell(p);
            self.seen.insert(p, c);
            c
        }
    }

    // -----------------------------------------------------------------------------
    // The tree
    // -----------------------------------------------------------------------------

    /// Fewer natural leaves than this and it is no tree (a log house, a dead trunk).
    pub const MIN_LEAVES: usize = 4;
    const MAX_HANGERS: usize = 512;

    #[derive(Clone, Copy, Debug)]
    pub struct Limits {
        pub max_logs: usize,
        pub max_blocks: usize,
        pub radius: i32,
    }

    impl Limits {
        pub fn of(config: &Config) -> Limits {
            Limits {
                max_logs: config.get(Key::MaxLogs) as usize,
                max_blocks: config.get(Key::MaxBlocks) as usize,
                radius: config.get(Key::Radius),
            }
        }
    }

    /// Why a cut stays a single log.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Refusal {
        /// The cut log stood on nothing: a branch.
        Branch,
        NotATree,
        /// A log a player placed is part of it.
        Placed,
        TooBig,
        Unloaded,
    }

    #[derive(Clone, Debug, Default)]
    pub struct Tree {
        pub cut: Pos,
        pub logs: Vec<Pos>,
        pub leaves: Vec<Pos>,
        /// What breaks off as it goes: vines, cocoa, hanging moss, snow...
        pub hangers: Vec<Pos>,
    }

    impl Tree {
        /// Where the tree stood, to replant it: the cut log's place and the
        /// trunk's other logs at its height (the four of a 2×2 trunk).
        pub fn spots(&self) -> Vec<Pos> {
            let mut out = vec![self.cut];
            for p in self.logs.iter().filter(|p| p[1] == self.cut[1]) {
                if !out.contains(p) {
                    out.push(*p);
                }
            }
            out
        }

        pub fn blocks(&self) -> usize {
            self.logs.len() + self.leaves.len()
        }
    }

    fn add(p: Pos, o: Pos) -> Pos {
        [p[0] + o[0], p[1] + o[1], p[2] + o[2]]
    }

    const FACES: [Pos; 6] = [
        [1, 0, 0],
        [-1, 0, 0],
        [0, 1, 0],
        [0, -1, 0],
        [0, 0, 1],
        [0, 0, -1],
    ];

    fn around() -> impl Iterator<Item = Pos> {
        (-1..=1)
            .flat_map(|x| (-1..=1).flat_map(move |y| (-1..=1).map(move |z| [x, y, z])))
            .filter(|o| *o != [0, 0, 0])
    }

    /// The tree above the log just cut at `cut` (already gone), of species `wood`.
    ///
    /// The trunk is every log of that species joined to the cut (corners count), at
    /// its height or above and within the radius. The crown is the natural leaves
    /// whose nearest log is one of the trunk's: walking out from the trunk, a leaf
    /// `d` steps away belongs to it when vanilla's `distance` says no log is nearer.
    pub fn scan(
        look: &mut Look<'_>,
        placed: &dyn Fn(Pos) -> bool,
        cut: Pos,
        wood: u16,
        limits: &Limits,
    ) -> Result<Tree, Refusal> {
        match look.cell(add(cut, [0, -1, 0])) {
            None => return Err(Refusal::Unloaded),
            Some(Cell::Solid | Cell::Log { .. }) => {}
            Some(_) => return Err(Refusal::Branch),
        }
        // the trunk
        let mut logs = Vec::new();
        let mut seen = QuickSet::default();
        seen.insert(cut);
        let mut queue = VecDeque::from([cut]);
        while let Some(p) = queue.pop_front() {
            for o in around() {
                let q = add(p, o);
                if q[1] < cut[1]
                    || (q[0] - cut[0]).abs() > limits.radius
                    || (q[2] - cut[2]).abs() > limits.radius
                    || !seen.insert(q)
                {
                    continue;
                }
                match look.cell(q) {
                    None => return Err(Refusal::Unloaded),
                    Some(Cell::Log { wood: w }) if w == wood => {
                        if placed(q) {
                            return Err(Refusal::Placed);
                        }
                        logs.push(q);
                        if logs.len() > limits.max_logs {
                            return Err(Refusal::TooBig);
                        }
                        queue.push_back(q);
                    }
                    _ => {}
                }
            }
        }
        if logs.is_empty() {
            return Err(Refusal::NotATree);
        }
        // the crown
        let mut leaves = Vec::new();
        let mut visited: QuickSet<Pos> = logs.iter().copied().collect();
        visited.insert(cut);
        let mut queue: VecDeque<(Pos, u8)> = logs.iter().map(|p| (*p, 0)).collect();
        queue.push_back((cut, 0));
        while let Some((p, d)) = queue.pop_front() {
            if d >= 6 {
                continue;
            }
            for o in FACES {
                let q = add(p, o);
                if visited.contains(&q) {
                    continue;
                }
                if let Some(Cell::Leaves {
                    distance,
                    persistent: false,
                }) = look.cell(q)
                    && distance > d
                    && distance < 7
                {
                    visited.insert(q);
                    leaves.push(q);
                    queue.push_back((q, d + 1));
                }
            }
        }
        // the caps of a huge fungus or mushroom: every cap block joined to the stem
        // (corners count: a mushroom's sides meet its top only by their edges)
        let mut queue: VecDeque<Pos> = logs.iter().copied().collect();
        queue.push_back(cut);
        while let Some(p) = queue.pop_front() {
            for o in around() {
                let q = add(p, o);
                if q[1] < cut[1]
                    || (q[0] - cut[0]).abs() > limits.radius
                    || (q[2] - cut[2]).abs() > limits.radius
                    || visited.contains(&q)
                {
                    continue;
                }
                if let Some(Cell::Cap) = look.cell(q) {
                    visited.insert(q);
                    leaves.push(q);
                    if logs.len() + leaves.len() > limits.max_blocks {
                        return Err(Refusal::TooBig);
                    }
                    queue.push_back(q);
                }
            }
        }
        if leaves.len() < MIN_LEAVES {
            return Err(Refusal::NotATree);
        }
        if logs.len() + leaves.len() > limits.max_blocks {
            return Err(Refusal::TooBig);
        }
        // what hangs on it
        let mut hangers = Vec::new();
        let mut queue: VecDeque<Pos> = logs.iter().chain(&leaves).copied().collect();
        while let Some(p) = queue.pop_front() {
            for o in FACES {
                let q = add(p, o);
                if visited.contains(&q) || hangers.len() >= MAX_HANGERS {
                    continue;
                }
                let held = match look.cell(q) {
                    Some(Cell::Hanger(Attach::Below)) => o == [0, -1, 0],
                    Some(Cell::Hanger(Attach::Above)) => o == [0, 1, 0],
                    Some(Cell::Hanger(Attach::Side)) => true,
                    _ => false,
                };
                if held {
                    visited.insert(q);
                    hangers.push(q);
                    queue.push_back(q);
                }
            }
        }
        Ok(Tree {
            cut,
            logs,
            leaves,
            hangers,
        })
    }

    // -----------------------------------------------------------------------------
    // The falling body
    // -----------------------------------------------------------------------------

    /// Gravity at 100 %, in blocks per second squared.
    pub const GRAVITY: f64 = 9.81;
    const LOG_MASS: f64 = 1.0;
    const LEAF_MASS: f64 = 0.35;
    /// How far the fall may go when nothing stops it.
    pub const MAX_ANGLE: f64 = 120.0 * PI / 180.0;
    /// How far the trunk may drop into the gap the cut left.
    const MAX_DROP: i32 = 3;

    #[derive(Clone, Debug)]
    pub struct Part {
        pub cell: Pos,
        pub leaf: bool,
        /// The centre in the body's frame: along the fall, up, across.
        pub at: [f64; 3],
        /// Distance from the hinge.
        pub r: f64,
        /// A side open to the level (not against another block of the tree).
        pub surface: bool,
        /// A log with a side not against another log: the trunk meets the level there.
        pub edge: bool,
    }

    /// The tree as one rigid body: it drops into the gap the cut left, then turns
    /// about the front edge of its base.
    #[derive(Clone, Debug)]
    pub struct Body {
        pub pivot: [f64; 3],
        /// Where it falls (x, z), unit length.
        pub dir: [f64; 2],
        pub parts: Vec<Part>,
        pub cells: QuickSet<Pos>,
        /// Moment of inertia about the hinge.
        pub inertia: f64,
        /// Mass times its forward and upward offsets, summed.
        pub mu: f64,
        pub mv: f64,
        /// The farthest part from the hinge.
        pub reach: f64,
        pub logs: usize,
        /// How far it drops before it turns (blocks).
        pub sink: f64,
        /// How far it has dropped.
        pub lowered: f64,
    }

    /// The fall direction (x, z) of a yaw in degrees (0 faces south, +z).
    pub fn yaw_dir(yaw: f64) -> [f64; 2] {
        let r = yaw.to_radians();
        [-r.sin(), r.cos()]
    }

    /// The yaw in degrees of a fall direction.
    pub fn dir_yaw(dir: [f64; 2]) -> f64 {
        (-dir[0]).atan2(dir[1]).to_degrees()
    }

    /// How far the trunk drops straight down into the gap the cut left: the open
    /// cells under its lowest logs, the fewest of them. A trunk still standing on
    /// something (a 2×2 trunk cut on one side) doesn't drop.
    pub fn gap(look: &mut Look<'_>, tree: &Tree) -> f64 {
        let Some(y0) = tree.logs.iter().map(|p| p[1]).min() else {
            return 0.0;
        };
        let mut least = MAX_DROP;
        for p in tree.logs.iter().filter(|p| p[1] == y0) {
            let mut n = 0;
            while n < least {
                match look.cell([p[0], y0 - 1 - n, p[2]]) {
                    Some(Cell::Open | Cell::Hanger(_) | Cell::Leaves { .. }) => n += 1,
                    _ => break,
                }
            }
            least = least.min(n);
        }
        f64::from(least)
    }

    impl Body {
        /// The body of `tree` falling toward `dir`, dropping `sink` blocks first.
        /// It starts dropped (as the contact and crushing are worked out);
        /// `Fell::new` lifts it back.
        pub fn new(tree: &Tree, dir: [f64; 2], sink: f64) -> Body {
            let cells: QuickSet<Pos> = tree.logs.iter().chain(&tree.leaves).copied().collect();
            let logs: QuickSet<Pos> = tree.logs.iter().copied().collect();
            let y0 = tree.logs.iter().map(|p| p[1]).min().unwrap_or(tree.cut[1]);
            let base: Vec<&Pos> = tree.logs.iter().filter(|p| p[1] == y0).collect();
            let n = base.len().max(1) as f64;
            let cx = base.iter().map(|p| p[0] as f64 + 0.5).sum::<f64>() / n;
            let cz = base.iter().map(|p| p[2] as f64 + 0.5).sum::<f64>() / n;
            let half = 0.5 * (dir[0].abs() + dir[1].abs());
            let front = base
                .iter()
                .map(|p| (p[0] as f64 + 0.5 - cx) * dir[0] + (p[2] as f64 + 0.5 - cz) * dir[1])
                .fold(0.0f64, f64::max)
                + half;
            let pivot = [cx + dir[0] * front, y0 as f64, cz + dir[1] * front];
            let mut body = Body {
                pivot,
                dir,
                parts: Vec::new(),
                cells,
                inertia: 0.0,
                mu: 0.0,
                mv: 0.0,
                reach: 0.0,
                logs: tree.logs.len(),
                sink,
                lowered: sink,
            };
            let all = tree
                .logs
                .iter()
                .map(|p| (*p, false))
                .chain(tree.leaves.iter().map(|p| (*p, true)));
            for (cell, leaf) in all {
                let at = body.local([
                    cell[0] as f64 + 0.5,
                    cell[1] as f64 + 0.5,
                    cell[2] as f64 + 0.5,
                ]);
                let m = if leaf { LEAF_MASS } else { LOG_MASS };
                body.inertia += m * (at[0] * at[0] + at[1] * at[1] + 1.0 / 6.0);
                body.mu += m * at[0];
                body.mv += m * at[1];
                let r = (at[0] * at[0] + at[1] * at[1]).sqrt();
                body.reach = body.reach.max(r + 0.87);
                let surface = FACES.iter().any(|o| !body.cells.contains(&add(cell, *o)));
                let edge = !leaf && FACES.iter().any(|o| !logs.contains(&add(cell, *o)));
                body.parts.push(Part {
                    cell,
                    leaf,
                    at,
                    r,
                    surface,
                    edge,
                });
            }
            body
        }

        /// A world point in the body's frame (upright, not dropped): along the
        /// fall, up, across.
        pub fn local(&self, w: [f64; 3]) -> [f64; 3] {
            let (dx, dz) = (self.dir[0], self.dir[1]);
            let d = [
                w[0] - self.pivot[0],
                w[1] - self.pivot[1],
                w[2] - self.pivot[2],
            ];
            [d[0] * dx + d[2] * dz, d[1], d[0] * dz - d[2] * dx]
        }

        /// Where the point `at` of the body's frame is when it has turned `theta`
        /// (and dropped as far as it has).
        pub fn world(&self, at: [f64; 3], theta: f64) -> [f64; 3] {
            let (sn, cs) = theta.sin_cos();
            self.turned(at, sn, cs)
        }

        /// `world` with the turn's sine and cosine.
        fn turned(&self, at: [f64; 3], sn: f64, cs: f64) -> [f64; 3] {
            self.place(at[0] * cs + at[1] * sn, -at[0] * sn + at[1] * cs, at[2])
        }

        /// The world point of frame coordinates already turned.
        fn place(&self, u: f64, v: f64, s: f64) -> [f64; 3] {
            let (dx, dz) = (self.dir[0], self.dir[1]);
            [
                self.pivot[0] + u * dx + s * dz,
                self.pivot[1] - self.lowered + v,
                self.pivot[2] + u * dz - s * dx,
            ]
        }

        /// A world vector turned `theta` about the hinge's axis.
        pub fn turn(&self, v: [f64; 3], theta: f64) -> [f64; 3] {
            let (dx, dz) = (self.dir[0], self.dir[1]);
            let (u, s) = (v[0] * dx + v[2] * dz, v[0] * dz - v[2] * dx);
            let (sn, cs) = theta.sin_cos();
            let (u, y) = (u * cs + v[1] * sn, -u * sn + v[1] * cs);
            [u * dx + s * dz, y, u * dz - s * dx]
        }

        /// The turn as a quaternion (x, y, z, w): about the hinge's axis, top toward
        /// the fall.
        pub fn quat(&self, theta: f64) -> [f32; 4] {
            let (sn, cs) = (theta / 2.0).sin_cos();
            [
                (self.dir[1] * sn) as f32,
                0.0,
                (-self.dir[0] * sn) as f32,
                cs as f32,
            ]
        }

        /// A block display's translation from `anchor`, its entity's position, so
        /// that the block of `cell` shows turned `theta` (with `quat` as its rotation).
        pub fn translation(&self, cell: Pos, theta: f64, anchor: [f64; 3]) -> [f32; 3] {
            let corner = self.local([cell[0] as f64, cell[1] as f64, cell[2] as f64]);
            let w = self.world(corner, theta);
            [
                (w[0] - anchor[0]) as f32,
                (w[1] - anchor[1]) as f32,
                (w[2] - anchor[2]) as f32,
            ]
        }

        /// The display pose of part `i` turned `theta`, scaled `scale` about its
        /// centre and lifted `lift` blocks: its translation from `anchor` and rotation.
        pub fn pose(
            &self,
            i: usize,
            theta: f64,
            anchor: [f64; 3],
            scale: f64,
            lift: f64,
        ) -> ([f32; 3], [f32; 4]) {
            let c = self.center(i, theta);
            let h = self.turn([0.5 * scale; 3], theta);
            (
                [
                    (c[0] - h[0] - anchor[0]) as f32,
                    (c[1] + lift - h[1] - anchor[1]) as f32,
                    (c[2] - h[2] - anchor[2]) as f32,
                ],
                self.quat(theta),
            )
        }

        /// The centre of part `i` turned `theta`.
        pub fn center(&self, i: usize, theta: f64) -> [f64; 3] {
            self.world(self.parts[i].at, theta)
        }

        /// The points where the trunk meets the level: the centres of its outer
        /// logs, and with `fine` their corners too, a little inside. Only the
        /// corners on the side a log moves toward: the others never touch first.
        /// Leaves don't hold the tree up: they are crushed (`crush`).
        fn probes(&self, fine: bool) -> Vec<[f64; 3]> {
            let mut out = Vec::new();
            for p in self.parts.iter().filter(|p| p.edge) {
                out.push(p.at);
                if !fine {
                    continue;
                }
                // which way the block moves in the body's frame as it turns
                let (mu, mv) = (p.at[1], -p.at[0]);
                let h = 0.42;
                for x in [-h, h] {
                    for y in [-h, h] {
                        for z in [-h, h] {
                            let w = [
                                p.cell[0] as f64 + 0.5 + x,
                                p.cell[1] as f64 + 0.5 + y,
                                p.cell[2] as f64 + 0.5 + z,
                            ];
                            let at = self.local(w);
                            let lead = (at[0] - p.at[0]) * mu + (at[1] - p.at[1]) * mv;
                            if lead >= -0.05 * p.r {
                                out.push(at);
                            }
                        }
                    }
                }
            }
            out
        }

        /// The cell at world point `w` when something solid not of the body is there.
        fn solid(&self, look: &mut Look<'_>, w: [f64; 3]) -> Option<Pos> {
            let cell = [
                w[0].floor() as i32,
                w[1].floor() as i32,
                w[2].floor() as i32,
            ];
            if self.cells.contains(&cell) {
                return None;
            }
            matches!(look.cell(cell), None | Some(Cell::Solid | Cell::Log { .. })).then_some(cell)
        }

        /// How far the body turns before its trunk meets the level.
        ///
        /// Each probe is swept along its circle in steps that move it at most a
        /// third of a block (with `fine`), only as far as the lowest contact found
        /// so far, and its first contact is then narrowed down.
        pub fn contact(&self, look: &mut Look<'_>, fine: bool) -> Contact {
            let probes = self.probes(fine);
            let mut best: Option<(f64, f64)> = None; // (last free, first blocked)
            for at in &probes {
                let r = at[0].hypot(at[1]).max(1.0);
                let step = if fine {
                    (1.5f64).to_radians().min(0.35 / r)
                } else {
                    (3.0f64).to_radians()
                };
                let limit = best.map_or(MAX_ANGLE, |b| b.1);
                let (sn, cs) = step.sin_cos();
                let (mut u, mut v) = (at[0], at[1]);
                let mut theta = 0.0;
                while theta + step < limit {
                    (u, v) = (u * cs + v * sn, -u * sn + v * cs);
                    theta += step;
                    if self.solid(look, self.place(u, v, at[2])).is_none() {
                        continue;
                    }
                    let (mut lo, mut hi) = (theta - step, theta);
                    if fine {
                        for _ in 0..8 {
                            let mid = 0.5 * (lo + hi);
                            if self.solid(look, self.world(*at, mid)).is_some() {
                                hi = mid;
                            } else {
                                lo = mid;
                            }
                        }
                    }
                    if best.is_none_or(|b| hi < b.1) {
                        best = Some((lo, hi));
                    }
                    break;
                }
            }
            let Some((lo, hi)) = best else {
                return Contact {
                    theta: MAX_ANGLE,
                    ground: false,
                    points: Vec::new(),
                };
            };
            let mut points = Vec::new();
            if fine {
                for at in &probes {
                    let w = self.world(*at, hi);
                    if let Some(cell) = self.solid(look, w) {
                        points.push((w, cell));
                    }
                }
            }
            Contact {
                theta: lo.max(0.0),
                ground: true,
                points,
            }
        }

        /// For each part, the angle on the way to `stop` where a leaf runs into
        /// the level and is crushed (between the trunk and the ground, against a
        /// wall); None for logs and for leaves that stay clear.
        pub fn crush(&self, look: &mut Look<'_>, stop: f64) -> Vec<Option<f64>> {
            let mut out = Vec::with_capacity(self.parts.len());
            for p in &self.parts {
                if !p.leaf {
                    out.push(None);
                    continue;
                }
                let step = (1.5f64).to_radians().min(0.35 / p.r.max(1.0));
                let mut theta: f64 = 0.0;
                let mut hit = None;
                loop {
                    if self.solid(look, self.world(p.at, theta)).is_some() {
                        hit = Some(theta);
                        break;
                    }
                    if theta >= stop {
                        break;
                    }
                    theta = (theta + step).min(stop);
                }
                out.push(hit);
            }
            out
        }

        /// The fastest part over the box `lo`..`hi` at `theta`, turning at `omega`:
        /// its speed (blocks per second) and whether it is a leaf. Logs count first.
        pub fn strike(
            &self,
            theta: f64,
            omega: f64,
            lo: [f64; 3],
            hi: [f64; 3],
            alive: &[bool],
        ) -> Option<(f64, bool)> {
            let mut best: Option<(f64, bool)> = None;
            for (i, p) in self.parts.iter().enumerate() {
                if !alive.get(i).copied().unwrap_or(false) {
                    continue;
                }
                let c = self.world(p.at, theta);
                let inside = (0..3).all(|k| c[k] + 0.55 > lo[k] && c[k] - 0.55 < hi[k]);
                if !inside {
                    continue;
                }
                let speed = omega.abs() * p.r;
                let weight = |s: f64, leaf: bool| if leaf { s * 0.3 } else { s };
                if best.is_none_or(|(s, l)| weight(speed, p.leaf) > weight(s, l)) {
                    best = Some((speed, p.leaf));
                }
            }
            best
        }
    }

    #[derive(Clone, Debug)]
    pub struct Contact {
        /// The angle where it stops.
        pub theta: f64,
        /// Whether it stops on something (else it would go on falling: a cliff).
        pub ground: bool,
        /// Where the trunk touches, and the block touched.
        pub points: Vec<([f64; 3], Pos)>,
    }

    /// Which way it falls: within `cone` degrees of where the player looks (`aim`),
    /// else anywhere, toward the side where its trunk goes farthest before it meets
    /// something, closest to the player's look on a tie.
    pub fn choose_dir(
        tree: &Tree,
        look: &mut Look<'_>,
        yaw: f64,
        aim: bool,
        cone: i32,
        sink: f64,
    ) -> [f64; 2] {
        let offsets: Vec<f64> = if aim {
            let cone = cone.clamp(0, 90) as f64;
            let n = (cone / 10.0).ceil() as i32;
            if n == 0 {
                vec![0.0]
            } else {
                (-n..=n).map(|k| cone * k as f64 / n as f64).collect()
            }
        } else {
            (0..12)
                .map(|k| {
                    if k <= 6 {
                        k as f64 * 30.0
                    } else {
                        (k - 12) as f64 * 30.0
                    }
                })
                .collect()
        };
        let penalty = if aim { 0.25 } else { 0.05 };
        let mut best = (f64::MIN, yaw_dir(yaw));
        for off in offsets {
            let dir = yaw_dir(yaw + off);
            let body = Body::new(tree, dir, sink);
            let reach = body.contact(look, false).theta.min(95f64.to_radians());
            let score = reach.to_degrees() - penalty * off.abs();
            if score > best.0 + 1e-9 {
                best = (score, dir);
            }
        }
        best.1
    }

    /// The pose of the cut block's ghost on tick `k` of its shake, its entity at
    /// the block's centre: it jiggles side to side, wobbles about the vertical and
    /// swells to `scale`. Its translation and rotation.
    pub fn shake_pose(k: u32, scale: f64) -> ([f32; 3], [f32; 4]) {
        let sign = if k.is_multiple_of(2) { 1.0 } else { -1.0 };
        let fade = 1.0 - f64::from(k) / f64::from(SHAKE_TICKS + 2);
        let jitter = [0.06 * sign * fade, 0.0, -0.04 * sign * fade];
        let yaw = (7.0f64).to_radians() * sign * fade;
        let h = 0.5 * scale;
        // the half diagonal (h, h, h) turned about the vertical
        let (ys, yc) = yaw.sin_cos();
        let (hx, hz) = (h * yc + h * ys, -h * ys + h * yc);
        let (qs, qc) = (yaw / 2.0).sin_cos();
        (
            [
                (jitter[0] - hx) as f32,
                (jitter[1] - h) as f32,
                (jitter[2] - hz) as f32,
            ],
            [0.0, qs as f32, 0.0, qc as f32],
        )
    }

    /// The pose of a log while a player digs it: tick `k` of the dig, `strength`
    /// 0 to 1 as the dig goes on. Its display stands at the centre of the cell
    /// in front of the face being dug (`face` points out of the block, so it is
    /// lit as that side is), and shows the log swelling a little and jiggling
    /// harder and harder, always covering the real block. Its translation and
    /// scale.
    pub fn tremble_pose(k: u32, strength: f64, face: Pos) -> ([f32; 3], f32) {
        let amp = 0.012 + 0.03 * strength.clamp(0.0, 1.0);
        let scale = 1.0 + 2.0 * amp + 0.01;
        let a = if k.is_multiple_of(2) { 1.0 } else { -1.0 };
        let b = if k % 4 < 2 { 1.0 } else { -1.0 };
        let jitter = [amp * a, amp * 0.4 * b, -amp * 0.7 * a];
        let h = scale / 2.0;
        (
            [0, 1, 2].map(|i| (jitter[i] - f64::from(face[i]) - h) as f32),
            scale as f32,
        )
    }

    // -----------------------------------------------------------------------------
    // The motion
    // -----------------------------------------------------------------------------

    pub const TICK: f64 = 0.05;
    const SUBSTEPS: u32 = 8;
    /// The cut's push, in radians per second at 100 %.
    const START_SPIN: f64 = 0.5;
    /// The least angular acceleration until the body is past its balance point
    /// (the hinge holds it like a felling wedge would).
    const WEDGE: f64 = 1.6;
    const RESTITUTION: f64 = 0.28;
    /// Below this (radians per second) a bounce dies out.
    const SETTLE: f64 = 0.3;
    const MAX_BOUNCES: u32 = 3;

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Impact {
        /// Angular speed when it hit (radians per second).
        pub omega: f64,
        /// The first touch.
        pub first: bool,
        /// It settles now.
        pub last: bool,
        /// False when it stopped in the air at the end of its swing.
        pub ground: bool,
    }

    #[derive(Clone, Debug)]
    pub struct Motion {
        pub theta: f64,
        pub omega: f64,
        pub stop: f64,
        pub ground: bool,
        pub bounces: u32,
        pub settled: bool,
        ku: f64,
        kv: f64,
        balance: f64,
        wedge: f64,
    }

    impl Motion {
        /// The motion of `body` until `contact`, gravity at `speed` percent.
        pub fn new(body: &Body, contact: &Contact, speed: i32) -> Motion {
            let s = (speed.max(1) as f64) / 100.0;
            let g = GRAVITY * s;
            let inertia = body.inertia.max(1e-6);
            Motion {
                theta: 0.0,
                omega: START_SPIN * s.sqrt(),
                stop: contact.theta,
                ground: contact.ground,
                bounces: 0,
                settled: contact.theta <= 0.0,
                ku: g * body.mu / inertia,
                kv: g * body.mv / inertia,
                balance: if body.mu < 0.0 {
                    (-body.mu).atan2(body.mv)
                } else {
                    0.0
                },
                wedge: WEDGE * s,
            }
        }

        /// It goes on from where it is toward a new `contact` (what stopped it
        /// gave way), turning at `omega`.
        pub fn resume(&mut self, contact: &Contact, omega: f64) {
            self.stop = contact.theta.max(self.theta);
            self.ground = contact.ground;
            self.omega = omega.max(0.0);
            self.bounces = 0;
            self.settled = false;
        }

        /// One tick.
        pub fn step(&mut self) -> Option<Impact> {
            if self.settled {
                return None;
            }
            let dt = TICK / SUBSTEPS as f64;
            for _ in 0..SUBSTEPS {
                let (sn, cs) = self.theta.sin_cos();
                let mut a = self.ku * cs + self.kv * sn;
                if self.theta < self.balance + 0.05 && self.omega >= 0.0 {
                    a = a.max(self.wedge);
                }
                self.omega += a * dt;
                self.theta += self.omega * dt;
                if self.theta < 0.0 {
                    self.theta = 0.0;
                    self.omega = self.omega.max(0.0);
                }
                if self.theta >= self.stop {
                    self.theta = self.stop;
                    let hit = self.omega;
                    if !self.ground {
                        self.omega = 0.0;
                        self.settled = true;
                        return Some(Impact {
                            omega: hit,
                            first: true,
                            last: true,
                            ground: false,
                        });
                    }
                    self.bounces += 1;
                    self.omega = -hit * RESTITUTION;
                    let last = self.omega.abs() < SETTLE || self.bounces >= MAX_BOUNCES;
                    if last {
                        self.omega = 0.0;
                        self.settled = true;
                    }
                    return Some(Impact {
                        omega: hit,
                        first: self.bounces == 1,
                        last,
                        ground: true,
                    });
                }
            }
            None
        }
    }

    // -----------------------------------------------------------------------------
    // A felling, tick by tick
    // -----------------------------------------------------------------------------

    /// Ticks the cut block shakes before it pops.
    pub const SHAKE_TICKS: u32 = 8;
    /// Ticks over which the blocks start popping, from the base to the top.
    pub const BREAK_TICKS: usize = 24;
    /// A popping block's scale and lift (blocks), one tick each, before it goes.
    pub const POP: [(f64, f64); 4] = [(1.18, 0.08), (1.1, 0.2), (0.7, 0.32), (0.28, 0.4)];
    /// The steps the breaking's pops climb through, base to top (semitones of a
    /// pentatonic scale over two octaves: the server turns them into pitches).
    pub const TUNE: [i32; 11] = [0, 2, 4, 7, 9, 12, 14, 16, 19, 21, 24];
    /// A fall never lasts longer than this before it breaks.
    pub const MAX_FALL_TICKS: u32 = 400;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Phase {
        /// The cut block shakes, ticks left.
        Shaking(u32),
        /// The trunk drops into the gap the cut left.
        Dropping,
        Falling,
        /// Lying on the ground, ticks left.
        Lying(u32),
        Breaking,
        Done,
    }

    #[derive(Clone, Debug, Default)]
    pub struct Frame {
        pub theta: f64,
        /// The whole tree moved: every display gets its new pose.
        pub moved: bool,
        /// The cut block's shake: its tick and scale.
        pub shake: Option<(u32, f64)>,
        /// The cut block pops away (the shake's last tick).
        pub popped: bool,
        /// The trunk came down on the stump, this fast (blocks per second).
        pub landed: Option<f64>,
        pub impact: Option<Impact>,
        /// Leaves crushed between the trunk and the ground.
        pub crushed: Vec<usize>,
        /// Blocks puffing up before they go: part, scale, lift.
        pub popping: Vec<(usize, f64, f64)>,
        /// Blocks that go this tick.
        pub broke: Vec<usize>,
        /// A pop's step (an index in `TUNE`), rising as the breaking climbs.
        pub note: Option<usize>,
        pub done: bool,
    }

    #[derive(Clone, Debug)]
    pub struct Fell {
        pub body: Body,
        pub motion: Motion,
        pub phase: Phase,
        pub alive: Vec<bool>,
        pub age: u32,
        crush: Vec<Option<f64>>,
        order: Vec<usize>,
        next: usize,
        per_tick: usize,
        lie: u32,
        gravity: f64,
        drop_speed: f64,
        pops: Vec<(usize, usize)>,
        fall_age: u32,
    }

    impl Fell {
        /// `crush` from `Body::crush`; `shake`: the cut block shakes first;
        /// `speed`: the gravity setting.
        pub fn new(
            mut body: Body,
            motion: Motion,
            crush: Vec<Option<f64>>,
            lie_ticks: u32,
            shake: bool,
            speed: i32,
        ) -> Fell {
            let mut order: Vec<usize> = (0..body.parts.len()).collect();
            // from the hinge up to the top of the crown
            order.sort_by(|a, b| {
                let (pa, pb) = (&body.parts[*a], &body.parts[*b]);
                pa.r.total_cmp(&pb.r).then(pa.leaf.cmp(&pb.leaf))
            });
            let n = order.len();
            let drops = body.sink > 0.0;
            if drops {
                body.lowered = 0.0;
            }
            let phase = if shake {
                Phase::Shaking(SHAKE_TICKS)
            } else if drops {
                Phase::Dropping
            } else {
                Phase::Falling
            };
            Fell {
                alive: vec![true; n],
                per_tick: n.div_ceil(BREAK_TICKS).max(1),
                order,
                next: 0,
                body,
                motion,
                phase,
                age: 0,
                crush,
                lie: lie_ticks,
                gravity: GRAVITY * f64::from(speed.max(1)) / 100.0,
                drop_speed: 0.0,
                pops: Vec::new(),
                fall_age: 0,
            }
        }

        pub fn theta(&self) -> f64 {
            self.motion.theta
        }

        /// What stopped the trunk gave way: it falls on toward `contact`, turning
        /// at `omega`, its leaves crushed at the new `crush` angles.
        pub fn resume(&mut self, contact: &Contact, crush: Vec<Option<f64>>, omega: f64) {
            self.motion.resume(contact, omega);
            self.crush = crush;
            self.phase = Phase::Falling;
            self.fall_age = 0;
        }

        pub fn tick(&mut self) -> Frame {
            self.age += 1;
            let mut frame = Frame {
                theta: self.motion.theta,
                ..Frame::default()
            };
            match self.phase {
                Phase::Shaking(n) => {
                    let k = SHAKE_TICKS - n;
                    let last = n <= 1;
                    let scale = if last {
                        1.3
                    } else if k % 2 == 1 {
                        1.05
                    } else {
                        1.0
                    };
                    frame.shake = Some((k, scale));
                    if last {
                        frame.popped = true;
                        self.phase = if self.body.lowered < self.body.sink {
                            Phase::Dropping
                        } else {
                            Phase::Falling
                        };
                    } else {
                        self.phase = Phase::Shaking(n - 1);
                    }
                }
                Phase::Dropping => {
                    let dt = TICK / SUBSTEPS as f64;
                    for _ in 0..SUBSTEPS {
                        self.drop_speed += self.gravity * dt;
                        self.body.lowered += self.drop_speed * dt;
                        if self.body.lowered >= self.body.sink {
                            self.body.lowered = self.body.sink;
                            frame.landed = Some(self.drop_speed);
                            self.phase = Phase::Falling;
                            break;
                        }
                    }
                    frame.moved = true;
                }
                Phase::Falling => {
                    self.fall_age += 1;
                    let before = self.motion.theta;
                    frame.impact = self.motion.step();
                    frame.theta = self.motion.theta;
                    frame.moved = (frame.theta - before).abs() > 1e-9;
                    for (i, c) in self.crush.iter_mut().enumerate() {
                        if c.is_some_and(|c| c <= frame.theta + 1e-9) {
                            *c = None;
                            if self.alive[i] {
                                self.alive[i] = false;
                                frame.crushed.push(i);
                            }
                        }
                    }
                    if self.motion.settled || self.fall_age >= MAX_FALL_TICKS {
                        self.motion.settled = true;
                        self.phase = if self.motion.ground {
                            Phase::Lying(self.lie)
                        } else {
                            Phase::Breaking
                        };
                    }
                }
                Phase::Lying(n) => {
                    self.phase = if n <= 1 {
                        Phase::Breaking
                    } else {
                        Phase::Lying(n - 1)
                    };
                }
                Phase::Breaking => {
                    let mut still = Vec::with_capacity(self.pops.len() + self.per_tick);
                    for &(i, age) in &self.pops {
                        if let Some(&(scale, lift)) = POP.get(age) {
                            frame.popping.push((i, scale, lift));
                            still.push((i, age + 1));
                        } else {
                            self.alive[i] = false;
                            frame.broke.push(i);
                        }
                    }
                    let mut started = 0;
                    while started < self.per_tick && self.next < self.order.len() {
                        let i = self.order[self.next];
                        self.next += 1;
                        if !self.alive[i] {
                            continue;
                        }
                        frame.popping.push((i, POP[0].0, POP[0].1));
                        still.push((i, 1));
                        started += 1;
                    }
                    if started > 0 && self.age.is_multiple_of(2) {
                        let climb = self.next as f64 / self.order.len().max(1) as f64;
                        frame.note = Some((climb * (TUNE.len() - 1) as f64).round() as usize);
                    }
                    self.pops = still;
                    if self.next >= self.order.len() && self.pops.is_empty() {
                        self.phase = Phase::Done;
                        frame.done = true;
                    }
                }
                Phase::Done => frame.done = true,
            }
            frame
        }

        /// The parts still shown.
        pub fn remaining(&self) -> impl Iterator<Item = usize> + '_ {
            (0..self.alive.len()).filter(|i| self.alive[*i])
        }
    }
}
