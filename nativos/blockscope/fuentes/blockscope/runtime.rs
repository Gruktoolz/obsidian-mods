// Blockscope's own half: its settings, the rows of its box, where the box
// goes and how a container's slots lie out. No game types here; `glue.rs`
// converts the game's and draws. This file is also the bundle's Cargo
// library (`native/lib.rs`), so its tests run without a Horizon build.

/// The mod's id (its settings folder, as the WebAssembly mod's), its name
/// and version.
pub const ID: &str = "blockscope";
pub const NAME: &str = "Blockscope";
pub const VERSION: &str = "0.3.1";
pub const DESCRIPTION: &str = "The block under your crosshair: its name, id, distance, icon, hardness and the tool it wants, states, a crop's growth and a container's contents, with reach, placement and what to show on its own settings page. A Jade-style tooltip for Horizon.";

/// How far the crosshair reaches, in blocks.
pub const REACHES: [u32; 4] = [5, 20, 64, 1024];
/// Where the box sits.
pub const PLACES: [&str; 4] = [
    "Beside the crosshair",
    "Top left",
    "Top centre",
    "Top right",
];
/// The background's opacity, in percent.
pub const OPACITIES: [u8; 4] = [100, 75, 50, 25];

/// White name, grey detail.
pub const INK: u32 = 0xFF_FFFF;
pub const INK_DIM: u32 = 0xA0_A0A0;
pub const PAD: f32 = 3.0;
/// A line of text, and the row that carries a 16 px icon beside it.
pub const LINE: f32 = 10.0;
pub const ICON_ROW: f32 = 18.0;
pub const ICON: f32 = 16.0;
pub const ICON_GAP: f32 = 2.0;
/// The HUD atlas's own heart, under the key the atlas build gives it.
pub const HEART: &str = "gui/sprites/hud/heart/full";
pub const HEART_SIZE: f32 = 9.0;
/// A container's slot, and the gap under the text above it.
pub const CELL: f32 = 18.0;
pub const GRID_GAP: f32 = 2.0;
/// Clear of the crosshair, and of the screen's edge.
pub const GAP: f32 = 10.0;
pub const EDGE: f32 = 2.0;

/// The tool a block asks for, by its mineable tag, and the item whose icon
/// shows it.
pub const TOOLS: [(&str, &str); 6] = [
    ("minecraft:mineable/pickaxe", "minecraft:iron_pickaxe"),
    ("minecraft:mineable/axe", "minecraft:iron_axe"),
    ("minecraft:mineable/shovel", "minecraft:iron_shovel"),
    ("minecraft:mineable/hoe", "minecraft:iron_hoe"),
    ("minecraft:mineable/shears", "minecraft:shears"),
    ("minecraft:sword_efficient", "minecraft:iron_sword"),
];

/// The settings' keys, as the WebAssembly mod wrote them.
const KEYS: [&str; 10] = [
    "reach",
    "place",
    "background",
    "coords",
    "icon",
    "harvest",
    "states",
    "growth",
    "containers",
    "entities",
];

/// The age a crop ripens at, for the blocks the game grows by an `age`
/// property; any other block with `age` is a state, not growth.
const RIPENS: [(&str, u32); 11] = [
    ("minecraft:wheat", 7),
    ("minecraft:carrots", 7),
    ("minecraft:potatoes", 7),
    ("minecraft:beetroots", 3),
    ("minecraft:nether_wart", 3),
    ("minecraft:sweet_berry_bush", 3),
    ("minecraft:cocoa", 2),
    ("minecraft:melon_stem", 7),
    ("minecraft:pumpkin_stem", 7),
    ("minecraft:torchflower_crop", 2),
    ("minecraft:pitcher_crop", 4),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    pub reach: usize,
    pub place: usize,
    pub opacity: usize,
    pub coords: bool,
    pub icon: bool,
    pub harvest: bool,
    pub states: bool,
    pub growth: bool,
    pub containers: bool,
    pub entities: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            reach: 1,
            place: 0,
            opacity: 2,
            coords: false,
            icon: true,
            harvest: true,
            states: true,
            growth: true,
            containers: true,
            entities: true,
        }
    }
}

impl Settings {
    /// The settings `read` answers its keys for: a value or a label
    /// (`"20"`, `"Top left"`), and `true`/`false` for the flags. The
    /// WebAssembly mod's files are read as they are.
    pub fn load(read: impl Fn(&str) -> Option<String>) -> Self {
        let mut settings = Self::default();
        let value = |key: &str| read(key).map(|v| v.trim().to_owned());
        let reaches = REACHES.map(|r| r.to_string());
        if let Some(i) = value(KEYS[0]).and_then(|v| pick(&v, &reaches)) {
            settings.reach = i;
        }
        if let Some(i) = value(KEYS[1]).and_then(|v| pick(&v, &PLACES.map(String::from))) {
            settings.place = i;
        }
        let opacities = OPACITIES.map(|o| o.to_string());
        if let Some(i) = value(KEYS[2]).and_then(|v| pick(&v, &opacities)) {
            settings.opacity = i;
        }
        let flags = [
            &mut settings.coords,
            &mut settings.icon,
            &mut settings.harvest,
            &mut settings.states,
            &mut settings.growth,
            &mut settings.containers,
            &mut settings.entities,
        ];
        for (flag, key) in flags.into_iter().zip(&KEYS[3..]) {
            if let Some(v) = value(key) {
                *flag = v != "false";
            }
        }
        settings
    }
    /// The keys with the values to keep, as the WebAssembly mod kept them,
    /// so a player's choice carries over.
    pub fn saved(&self) -> [(&'static str, String); 10] {
        [
            (KEYS[0], REACHES[self.reach].to_string()),
            (KEYS[1], PLACES[self.place].to_owned()),
            (KEYS[2], OPACITIES[self.opacity].to_string()),
            (KEYS[3], self.coords.to_string()),
            (KEYS[4], self.icon.to_string()),
            (KEYS[5], self.harvest.to_string()),
            (KEYS[6], self.states.to_string()),
            (KEYS[7], self.growth.to_string()),
            (KEYS[8], self.containers.to_string()),
            (KEYS[9], self.entities.to_string()),
        ]
    }
    /// Its page's buttons, one click per value.
    pub fn labels(&self) -> Vec<String> {
        vec![
            format!("Reach: {} blocks", REACHES[self.reach]),
            format!("Position: {}", PLACES[self.place]),
            format!("Background: {}%", OPACITIES[self.opacity]),
            format!("Coordinates: {}", on(self.coords)),
            format!("Item icon: {}", on(self.icon)),
            format!("Hardness and tool: {}", on(self.harvest)),
            format!("Block states: {}", on(self.states)),
            format!("Crop growth: {}", on(self.growth)),
            format!("Container contents: {}", on(self.containers)),
            format!("Creatures: {}", on(self.entities)),
        ]
    }
    /// Button `button` of its page went: the next value.
    pub fn click(&mut self, button: usize) {
        match button {
            0 => self.reach = (self.reach + 1) % REACHES.len(),
            1 => self.place = (self.place + 1) % PLACES.len(),
            2 => self.opacity = (self.opacity + 1) % OPACITIES.len(),
            3 => self.coords = !self.coords,
            4 => self.icon = !self.icon,
            5 => self.harvest = !self.harvest,
            6 => self.states = !self.states,
            7 => self.growth = !self.growth,
            8 => self.containers = !self.containers,
            9 => self.entities = !self.entities,
            _ => {}
        }
    }
    /// How far the crosshair reaches now, in blocks.
    pub fn reach(&self) -> f64 {
        f64::from(REACHES[self.reach])
    }
}

fn on(flag: bool) -> &'static str {
    if flag { "ON" } else { "OFF" }
}

/// The index `value` names in `labels`: one of them, or a number below the
/// list's length.
fn pick(value: &str, labels: &[String]) -> Option<usize> {
    if let Some(i) = labels.iter().position(|l| l == value) {
        return Some(i);
    }
    value.parse::<usize>().ok().filter(|i| *i < labels.len())
}

/// The game's own name for a block state (`minecraft:oak_log[axis=y]`) as
/// the line a tooltip shows: `axis=y`, or none for a block without
/// properties.
pub fn properties(state: &str) -> Option<&str> {
    let (_, rest) = state.split_once('[')?;
    let props = rest.strip_suffix(']').unwrap_or(rest);
    (!props.is_empty()).then_some(props)
}

/// `age=3` out of a state's properties.
fn age(properties: &str) -> Option<u32> {
    properties
        .split(',')
        .find_map(|p| p.trim().strip_prefix("age="))
        .and_then(|v| v.parse().ok())
}

/// The crop's age out of `RIPENS` and its state: `(age, ripens at)`, or none
/// for a block that is not a crop.
pub fn growth(block: &str, properties: &str) -> Option<(u32, u32)> {
    let (_, max) = RIPENS.iter().find(|(id, _)| *id == block)?;
    Some((age(properties)?, *max))
}

/// The tool a block asks for, out of `TOOLS`: `carries` answers whether the
/// block is in a mineable tag.
pub fn tool(carries: impl Fn(&str) -> bool) -> Option<&'static str> {
    let (_, tool) = TOOLS.iter().find(|(tag, _)| carries(tag))?;
    Some(tool)
}

/// The hardness and the tool, as Jade's line: an unbreakable block, one that
/// asks for the right tool, and whether what the player holds is one.
pub fn harvest(hardness: f32, requires_tool: bool, correct: bool) -> String {
    if hardness < 0.0 {
        return "Unbreakable".to_owned();
    }
    if requires_tool {
        let tool = if correct {
            "this tool harvests it"
        } else {
            "needs the right tool"
        };
        return format!("Hardness {hardness:.1} · {tool}");
    }
    format!("Hardness {hardness:.1}")
}

/// A container's slots as the grid its own screen shows: a chest is 9 wide,
/// a hopper 5, a dispenser 3.
pub fn grid(slots: usize) -> Option<(usize, usize)> {
    let columns = match slots {
        0 => return None,
        9 => 3,
        5 => 5,
        3 | 4 => slots,
        n if n % 9 == 0 => 9,
        _ => 9,
    };
    Some((columns, slots.div_ceil(columns)))
}

/// Whether a container's answer (`at`, the block it was read from) is the
/// block a tooltip is drawing (`pos`, its own id): the same place is enough
/// for a stale answer to put one container's slots over another block.
pub fn same_block(at: [i32; 3], answered: &str, pos: [i32; 3], block: &str) -> bool {
    at == pos && answered == block
}

/// The box's size: rows of these heights (a line, or a row with an icon),
/// and a container grid of `columns` x `rows` cells under them.
pub fn size(heights: &[f32], text_w: f32, grid: Option<(usize, usize)>) -> (f32, f32) {
    let text_h: f32 = heights.iter().sum();
    let (grid_w, grid_h) = grid.map_or((0.0, 0.0), |(columns, rows)| {
        (columns as f32 * CELL, rows as f32 * CELL + GRID_GAP)
    });
    let w = (PAD * 2.0 + text_w).max(PAD * 2.0 + grid_w);
    (w, PAD * 2.0 + text_h + grid_h)
}

/// Where the box's top left goes: in a corner, centred at the top, or beside
/// the crosshair and flipped left when the screen's right edge is closer.
pub fn place(w: f32, h: f32, gui_w: f32, gui_h: f32, at: usize) -> (f32, f32) {
    match at {
        1 => (EDGE, EDGE),
        2 => (((gui_w - w) / 2.0).max(EDGE), EDGE),
        3 => ((gui_w - w - EDGE).max(EDGE), EDGE),
        _ => {
            let (cx, cy) = (gui_w / 2.0, gui_h / 2.0);
            let mut x = cx + GAP;
            if x + w > gui_w - EDGE {
                x = cx - GAP - w;
            }
            let y = (cy - h / 2.0).clamp(EDGE, (gui_h - h - EDGE).max(EDGE));
            (x, y)
        }
    }
}

/// `50` as `0x7F000000`: the background's opacity is the fill's alpha.
pub fn alpha(percent: u8) -> u32 {
    (u32::from(percent) * 255 / 100) << 24
}

/// `minecraft:grass_block` as "Grass Block", for the blocks with no item.
pub fn pretty(id: &str) -> String {
    let path = id.split(':').nth(1).unwrap_or(id);
    let mut out = String::with_capacity(path.len());
    for (i, word) in path.split('_').enumerate() {
        if i > 0 {
            out.push(' ');
        }
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            out.extend(first.to_uppercase());
            out.push_str(chars.as_str());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_block_with_no_item_gets_a_readable_name() {
        assert_eq!(pretty("minecraft:grass_block"), "Grass Block");
        assert_eq!(pretty("minecraft:water"), "Water");
        assert_eq!(pretty("some_mod:ancient_altar"), "Ancient Altar");
    }

    #[test]
    fn the_box_sits_clear_of_the_crosshair() {
        assert_eq!(place(16.0, 25.0, 1920.0, 1080.0, 0), (970.0, 527.5));
    }

    #[test]
    fn the_box_flips_left_at_the_screen_edge() {
        assert_eq!(place(950.0, 25.0, 1920.0, 1080.0, 0).0, 960.0 - GAP - 950.0);
    }

    #[test]
    fn a_corner_place_pins_the_box_there() {
        assert_eq!(place(100.0, 25.0, 1920.0, 1080.0, 1), (EDGE, EDGE));
        assert_eq!(
            place(100.0, 25.0, 1920.0, 1080.0, 2),
            ((1920.0 - 100.0) / 2.0, EDGE)
        );
        assert_eq!(
            place(100.0, 25.0, 1920.0, 1080.0, 3),
            (1920.0 - 100.0 - EDGE, EDGE)
        );
        // A box wider than the screen stays at the left edge, never negative.
        assert_eq!(place(2000.0, 25.0, 1920.0, 1080.0, 2).0, EDGE);
        assert_eq!(place(2000.0, 25.0, 1920.0, 1080.0, 3).0, EDGE);
    }

    #[test]
    fn the_background_percentage_is_the_alpha() {
        assert_eq!(alpha(100), 0xFF00_0000);
        assert_eq!(alpha(50), 0x7F00_0000);
        assert_eq!(alpha(0), 0);
    }

    #[test]
    fn a_click_walks_the_values_and_wraps() {
        let mut s = Settings::default();
        s.click(0);
        assert_eq!(REACHES[s.reach], 64);
        s.click(0);
        s.click(0);
        assert_eq!(REACHES[s.reach], 5);
        s.click(3);
        assert!(s.coords);
        s.click(3);
        assert!(!s.coords);
        // Every switch on the page flips, and a button past it does nothing.
        for button in 5..10 {
            let before = s;
            s.click(button);
            assert_ne!(s, before, "button {button} does nothing");
        }
        let before = s;
        s.click(10);
        assert_eq!(s, before);
    }

    #[test]
    fn the_webassembly_mods_files_are_read_back() {
        let kept = [
            ("reach", "20"),
            ("place", "Top right"),
            ("background", "25"),
            ("coords", "true"),
            ("icon", "false"),
            ("harvest", "true"),
            ("states", "false"),
            ("growth", "true"),
            ("containers", "false"),
            ("entities", "true"),
        ];
        let s = Settings::load(|key| {
            kept.iter()
                .find(|(k, _)| *k == key)
                .map(|(_, v)| (*v).to_owned())
        });
        assert_eq!(
            s,
            Settings {
                reach: 1,
                place: 3,
                opacity: 3,
                coords: true,
                icon: false,
                harvest: true,
                states: false,
                growth: true,
                containers: false,
                entities: true,
            }
        );
        // Nothing kept: the defaults, and what it writes reads back the same.
        let fresh = Settings::load(|_| None);
        assert_eq!(fresh, Settings::default());
        let again = Settings::load(|key| {
            fresh
                .saved()
                .iter()
                .find(|(k, _)| *k == key)
                .map(|(_, v)| v.clone())
        });
        assert_eq!(again, fresh);
    }

    #[test]
    fn the_pages_labels_say_what_is_kept() {
        let labels = Settings::default().labels();
        assert_eq!(labels.len(), 10);
        assert_eq!(labels[0], "Reach: 20 blocks");
        assert_eq!(labels[2], "Background: 50%");
        assert_eq!(labels[3], "Coordinates: OFF");
        assert_eq!(labels[4], "Item icon: ON");
        assert_eq!(labels[5], "Hardness and tool: ON");
        assert_eq!(labels[9], "Creatures: ON");
    }

    #[test]
    fn a_state_shows_its_properties_only() {
        assert_eq!(properties("minecraft:oak_log[axis=y]"), Some("axis=y"));
        assert_eq!(properties("minecraft:wheat[age=3]"), Some("age=3"));
        assert_eq!(properties("minecraft:stone"), None);
        assert_eq!(properties("minecraft:water[level=0]"), Some("level=0"));
    }

    #[test]
    fn only_a_crop_has_growth() {
        assert_eq!(growth("minecraft:wheat", "age=3"), Some((3, 7)));
        assert_eq!(growth("minecraft:beetroots", "age=2"), Some((2, 3)));
        // A log's `axis` is a state, not growth; an unknown block is no crop.
        assert_eq!(growth("minecraft:oak_log", "axis=y"), None);
        assert_eq!(growth("minecraft:stone", "age=1"), None);
        assert_eq!(growth("minecraft:wheat", "moisture=7"), None);
    }

    #[test]
    fn the_tool_line_says_what_the_hand_can_do() {
        assert_eq!(harvest(0.6, false, false), "Hardness 0.6");
        assert_eq!(
            harvest(3.0, true, false),
            "Hardness 3.0 · needs the right tool"
        );
        assert_eq!(
            harvest(3.0, true, true),
            "Hardness 3.0 · this tool harvests it"
        );
        assert_eq!(harvest(-1.0, false, false), "Unbreakable");
    }

    #[test]
    fn the_icon_is_the_tool_the_block_asks_for() {
        // Stone is mined with a pickaxe, logs with an axe.
        let stone = |tag: &str| tag == "minecraft:mineable/pickaxe";
        assert_eq!(tool(stone), Some("minecraft:iron_pickaxe"));
        let log = |tag: &str| tag == "minecraft:mineable/axe";
        assert_eq!(tool(log), Some("minecraft:iron_axe"));
        let shears = |tag: &str| tag == "minecraft:mineable/shears";
        assert_eq!(tool(shears), Some("minecraft:shears"));
        // A block in no mineable tag (glass, cake) has no tool at all.
        assert_eq!(tool(|_| false), None);
        // Two tags at once: the pickaxe wins, as it comes first in TOOLS.
        assert_eq!(tool(|_| true), Some("minecraft:iron_pickaxe"));
    }

    #[test]
    fn a_containers_slots_lie_out_as_its_own_screen() {
        assert_eq!(grid(27), Some((9, 3)), "a chest");
        assert_eq!(grid(5), Some((5, 1)), "a hopper");
        assert_eq!(grid(9), Some((3, 3)), "a dispenser");
        assert_eq!(grid(3), Some((3, 1)), "a furnace");
        assert_eq!(grid(0), None);
    }

    #[test]
    fn an_answer_belongs_to_one_block() {
        let (at, pos) = ([3, 64, -7], [3, 64, -7]);
        assert!(same_block(at, "minecraft:chest", pos, "minecraft:chest"));
        // The same place with another block: a chest emptied into stone.
        assert!(!same_block(at, "minecraft:chest", pos, "minecraft:stone"));
        // Another place, same block.
        assert!(!same_block(
            at,
            "minecraft:chest",
            [3, 64, -6],
            "minecraft:chest"
        ));
    }

    #[test]
    fn a_line_of_text_sets_the_box_height() {
        let (w, h) = size(&[LINE, LINE], 40.0, None);
        assert_eq!((w, h), (46.0, 26.0));
        // A row with an icon is taller than a line.
        let (w, h) = size(&[ICON_ROW], 40.0, None);
        assert_eq!((w, h), (46.0, 24.0));
    }

    #[test]
    fn a_container_grid_widens_the_box() {
        let (w, h) = size(&[ICON_ROW, LINE, LINE], 40.0, Some((9, 3)));
        assert_eq!(w, PAD * 2.0 + 9.0 * CELL, "nine cells wide, no wider");
        assert_eq!(
            h,
            PAD * 2.0 + ICON_ROW + LINE + LINE + 3.0 * CELL + GRID_GAP
        );
    }
}
