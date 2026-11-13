// Map Preview for Horizon: a filled map's picture in its tooltip, and a small
// copy of it in its inventory slot instead of the plain map icon. Inspired by
// the Map Tooltip and Map in Slot mods, written from scratch on Horizon's hook
// points (none of their code). This file is both the mod's Cargo library (its
// tests) and the module the bundle adds to the game.
//
// The pictures come from the maps this client already has (the ones the
// server sent, as for any map you carry); a map the client hasn't been sent
// shows no preview. They are drawn as filled rectangles, one per run of
// same-coloured pixels.
use std::cell::RefCell;

/// The mod's id (its settings folder), name and version.
pub const ID: &str = "map_preview";
pub const NAME: &str = "Map Preview";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "A filled map's picture in its tooltip and in its inventory slot, so you can tell your maps apart.";

/// A map's side, in pixels.
pub const MAP_SIZE: usize = 128;
/// The map paper behind the picture (vanilla's map background).
pub const PAPER: u32 = 0xFF_DCC9A0;
/// The paper's darker edge.
pub const PAPER_EDGE: u32 = 0xFF_9C8459;
/// Space above the tooltip picture, in GUI pixels.
pub const TOOLTIP_GAP: f32 = 2.0;
/// A slot's icon, in GUI pixels.
pub const SLOT_SIZE: f32 = 16.0;

/// One option of the settings page: its file, its label, the values a click
/// cycles through (with their names) and the default.
pub struct Opt {
    pub key: &'static str,
    pub label: &'static str,
    pub choices: &'static [(u16, &'static str)],
    pub default: u16,
}

const ON_OFF: &[(u16, &str)] = &[(0, "OFF"), (1, "ON")];
const TOOLTIP: &[(u16, &str)] = &[(0, "OFF"), (1, "Always"), (2, "Hold Shift")];
/// The tooltip picture's side in GUI pixels.
const TOOLTIP_SIZE: &[(u16, &str)] = &[(64, "Small"), (96, "Medium"), (128, "Large")];
const SLOT_STYLE: &[(u16, &str)] = &[(0, "Full slot"), (1, "With paper edge")];

pub const OPTS: [Opt; 5] = [
    Opt { key: "tooltip", label: "Tooltip preview", choices: TOOLTIP, default: 1 },
    Opt { key: "tooltip_size", label: "Tooltip size", choices: TOOLTIP_SIZE, default: 96 },
    Opt { key: "slot", label: "Slot preview", choices: ON_OFF, default: 1 },
    Opt { key: "slot_style", label: "Slot style", choices: SLOT_STYLE, default: 1 },
    Opt { key: "paper", label: "Paper behind the map", choices: ON_OFF, default: 1 },
];

/// Indexes into `OPTS`.
pub mod o {
    pub const TOOLTIP: usize = 0;
    pub const TOOLTIP_SIZE: usize = 1;
    pub const SLOT: usize = 2;
    pub const SLOT_STYLE: usize = 3;
    pub const PAPER: usize = 4;
}

/// The label of the button after the options.
pub const RESET_LABEL: &str = "Reset to defaults";

/// The settings, one file each in the mod's data folder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    values: [u16; OPTS.len()],
}

impl Default for Settings {
    fn default() -> Self {
        Self { values: OPTS.map(|opt| opt.default) }
    }
}

impl Settings {
    /// From the stored values (`get` reads one key's file), the defaults for
    /// missing or unknown ones.
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let mut s = Self::default();
        for (i, opt) in OPTS.iter().enumerate() {
            let value = get(opt.key).and_then(|v| v.trim().parse::<u16>().ok());
            if let Some(v) = value
                && opt.choices.iter().any(|c| c.0 == v)
            {
                s.values[i] = v;
            }
        }
        s
    }

    pub fn get(&self, i: usize) -> u16 {
        self.values[i]
    }

    /// The tooltip picture's side, if the tooltip shows one now (`shift`:
    /// whether Shift is held).
    pub fn tooltip_side(&self, shift: bool) -> Option<f32> {
        match self.get(o::TOOLTIP) {
            1 => Some(f32::from(self.get(o::TOOLTIP_SIZE))),
            2 if shift => Some(f32::from(self.get(o::TOOLTIP_SIZE))),
            _ => None,
        }
    }

    /// The settings page's buttons: the options, then reset.
    pub fn labels(&self) -> Vec<String> {
        let mut labels: Vec<String> = OPTS
            .iter()
            .zip(self.values)
            .map(|(opt, v)| {
                let name = opt.choices.iter().find(|c| c.0 == v).map_or("?", |c| c.1);
                format!("{}: {name}", opt.label)
            })
            .collect();
        labels.push(RESET_LABEL.into());
        labels
    }

    /// A click on button `i` of `labels`: what to store, key by key.
    pub fn click(&mut self, i: usize) -> Vec<(&'static str, String)> {
        if i == OPTS.len() {
            *self = Self::default();
            return OPTS.iter().zip(self.values).map(|(opt, v)| (opt.key, v.to_string())).collect();
        }
        let Some(opt) = OPTS.get(i) else {
            return Vec::new();
        };
        let at = opt.choices.iter().position(|c| c.0 == self.values[i]).unwrap_or(0);
        self.values[i] = opt.choices[(at + 1) % opt.choices.len()].0;
        vec![(opt.key, self.values[i].to_string())]
    }
}

/// A map pixel's colour as ARGB (`MapColor.getColorFromPackedId`): the top six
/// bits pick `palette`'s base colour (`MapColor.col`), the low two its shade.
/// None for the empty colour, which shows the paper.
pub fn argb(packed: u8, palette: &[u32]) -> Option<u32> {
    let id = usize::from(packed >> 2);
    if id == 0 || id >= palette.len() {
        return None;
    }
    let modifier = [180u32, 220, 255, 135][usize::from(packed & 3)];
    let col = palette[id];
    let ch = |s: u32| (col >> s & 255) * modifier / 255;
    Some(0xFF00_0000 | ch(16) << 16 | ch(8) << 8 | ch(0))
}

/// One filled rectangle: `[x, y, w, h]` in GUI pixels, and its ARGB colour.
pub type Rect = ([f32; 4], u32);

/// The map's picture at `[x, y]`, `side` GUI pixels square, sampled to
/// `cells` x `cells` (each the map pixel at its centre), as rectangles: one
/// per run of same-coloured cells in a row. Empty pixels are left out.
pub fn picture(colors: &[u8], palette: &[u32], at: [f32; 2], side: f32, cells: usize) -> Vec<Rect> {
    let mut rects = Vec::new();
    if colors.len() < MAP_SIZE * MAP_SIZE || cells == 0 {
        return rects;
    }
    let cell = side / cells as f32;
    let source = |c: usize| (c * MAP_SIZE + MAP_SIZE / 2) / cells;
    for row in 0..cells {
        let sy = source(row);
        let mut run: Option<(usize, u32)> = None;
        for col in 0..=cells {
            let color = (col < cells).then(|| argb(colors[sy * MAP_SIZE + source(col)], palette)).flatten();
            match run {
                Some((_, c)) if Some(c) == color => continue,
                Some((start, c)) => {
                    let x = at[0] + start as f32 * cell;
                    rects.push(([x, at[1] + row as f32 * cell, (col - start) as f32 * cell, cell], c));
                    run = None;
                }
                None => {}
            }
            if let Some(c) = color {
                run = Some((col, c));
            }
        }
    }
    rects
}

/// The tooltip's picture: optional paper, then the map. `at` is the top-left
/// of the space the tooltip gave (the gap comes first).
pub fn tooltip_rects(colors: &[u8], palette: &[u32], at: [f32; 2], side: f32, paper: bool) -> Vec<Rect> {
    let top = [at[0], at[1] + TOOLTIP_GAP];
    let mut rects = Vec::new();
    if paper {
        rects.push(([top[0], top[1], side, side], PAPER));
    }
    // One cell per GUI pixel at most: a map pixel smaller than that is lost anyway.
    let cells = (side.round() as usize).min(MAP_SIZE);
    rects.extend(picture(colors, palette, top, side, cells));
    rects
}

/// A slot's picture at the icon's top-left `[x, y]`: the whole slot, or
/// inside a one-pixel paper edge.
pub fn slot_rects(colors: &[u8], palette: &[u32], at: [f32; 2], edged: bool, paper: bool) -> Vec<Rect> {
    let mut rects = Vec::new();
    let (inner, side) = if edged {
        rects.push(([at[0], at[1], SLOT_SIZE, SLOT_SIZE], PAPER_EDGE));
        ([at[0] + 1.0, at[1] + 1.0], SLOT_SIZE - 2.0)
    } else {
        (at, SLOT_SIZE)
    };
    if paper || edged {
        rects.push(([inner[0], inner[1], side, side], PAPER));
    }
    rects.extend(picture(colors, palette, inner, side, side as usize));
    rects
}

/// The tooltip space for a map, `[width, height]`.
pub fn tooltip_space(side: f32) -> [f32; 2] {
    [side, side + TOOLTIP_GAP]
}

#[derive(Default)]
struct State {
    settings: Option<Settings>,
    shift: [bool; 2],
    /// The map whose tooltip space was asked for this frame, and its size:
    /// the draw hook only draws what this mod measured.
    claimed: Option<(i32, [f32; 2])>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

/// The settings, read once through `get`, then kept.
pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    STATE.with_borrow_mut(|s| *s.settings.get_or_insert_with(|| Settings::read(get)))
}

/// Replaces the settings (the settings page changed them).
pub fn set_settings(settings: Settings) {
    STATE.with_borrow_mut(|s| s.settings = Some(settings));
}

/// A key went down or up: Shift is followed for "Hold Shift".
pub fn key(key: &str, pressed: bool) {
    let side = match key {
        "key.keyboard.left.shift" => 0,
        "key.keyboard.right.shift" => 1,
        _ => return,
    };
    STATE.with_borrow_mut(|s| s.shift[side] = pressed);
}

pub fn shift() -> bool {
    STATE.with_borrow(|s| s.shift[0] || s.shift[1])
}

/// The tooltip space this mod asked for map `id`.
pub fn claim(id: i32, size: [f32; 2]) {
    STATE.with_borrow_mut(|s| s.claimed = Some((id, size)));
}

/// Whether the tooltip space `size` for map `id` is this mod's to draw in.
pub fn claimed(id: i32, size: [f32; 2]) -> bool {
    STATE.with_borrow(|s| s.claimed == Some((id, size)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const PALETTE: [u32; 4] = [0, 0x7F_B238, 0x40_40FF, 0xFF_FFFF];

    fn stored<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        |key| pairs.iter().find(|p| p.0 == key).map(|p| p.1.to_owned())
    }

    #[test]
    fn options_have_their_defaults_among_their_choices() {
        for opt in &OPTS {
            assert!(opt.choices.iter().any(|c| c.0 == opt.default), "{}", opt.key);
        }
    }

    #[test]
    fn settings_read_click_and_reset() {
        assert_eq!(Settings::read(stored(&[])), Settings::default());
        let mut s = Settings::read(stored(&[("tooltip", "2"), ("tooltip_size", "128\n"), ("slot", "7")]));
        assert_eq!(s.tooltip_side(false), None);
        assert_eq!(s.tooltip_side(true), Some(128.0));
        assert_eq!(s.get(o::SLOT), 1, "not a choice: the default");
        assert_eq!(s.click(o::TOOLTIP), [("tooltip", "0".to_owned())]);
        assert_eq!(s.tooltip_side(true), None);
        assert_eq!(s.labels()[o::TOOLTIP], "Tooltip preview: OFF");
        assert_eq!(s.labels()[OPTS.len()], RESET_LABEL);
        assert_eq!(s.click(OPTS.len()).len(), OPTS.len());
        assert_eq!(s, Settings::default());
        assert_eq!(s.tooltip_side(false), Some(96.0));
        assert!(s.click(42).is_empty());
    }

    #[test]
    fn packed_colours_follow_vanilla_shades() {
        assert_eq!(argb(0, &PALETTE), None);
        assert_eq!(argb(3, &PALETTE), None, "id 0 is empty whatever its shade");
        // Grass, brightest shade (255): its own colour.
        assert_eq!(argb(1 << 2 | 2, &PALETTE), Some(0xFF7F_B238));
        // White at shade 0 (180/255).
        assert_eq!(argb(3 << 2, &PALETTE), Some(0xFFB4_B4B4));
        assert_eq!(argb(9 << 2, &PALETTE), None, "past the palette");
    }

    fn map(f: impl Fn(usize, usize) -> u8) -> Vec<u8> {
        (0..MAP_SIZE * MAP_SIZE).map(|i| f(i % MAP_SIZE, i / MAP_SIZE)).collect()
    }

    #[test]
    fn picture_merges_runs_and_skips_empty() {
        // Left half grass, right half water.
        let colors = map(|x, _| if x < 64 { 1 << 2 | 2 } else { 2 << 2 | 2 });
        let rects = picture(&colors, &PALETTE, [10.0, 20.0], 64.0, 64);
        assert_eq!(rects.len(), 128, "two runs a row");
        assert_eq!(rects[0], ([10.0, 20.0, 32.0, 1.0], 0xFF7F_B238));
        assert_eq!(rects[1], ([42.0, 20.0, 32.0, 1.0], 0xFF40_40FF));
        // An empty map: nothing but the paper.
        assert!(picture(&map(|_, _| 0), &PALETTE, [0.0; 2], 64.0, 64).is_empty());
        assert!(picture(&[1, 2, 3], &PALETTE, [0.0; 2], 64.0, 64).is_empty());
    }

    #[test]
    fn tooltip_and_slot_layouts() {
        let colors = map(|_, _| 1 << 2 | 2);
        let t = tooltip_rects(&colors, &PALETTE, [5.0, 5.0], 96.0, true);
        assert_eq!(t[0], ([5.0, 5.0 + TOOLTIP_GAP, 96.0, 96.0], PAPER));
        assert_eq!(t.len(), 1 + 96, "paper, then a run a row");
        assert_eq!(tooltip_space(96.0), [96.0, 98.0]);
        let s = slot_rects(&colors, &PALETTE, [0.0, 0.0], true, false);
        assert_eq!(s[0], ([0.0, 0.0, 16.0, 16.0], PAPER_EDGE));
        assert_eq!(s[1], ([1.0, 1.0, 14.0, 14.0], PAPER));
        assert_eq!(s[2], ([1.0, 1.0, 14.0, 1.0], 0xFF7F_B238));
        let full = slot_rects(&colors, &PALETTE, [0.0, 0.0], false, false);
        assert_eq!(full[0], ([0.0, 0.0, 16.0, 1.0], 0xFF7F_B238));
    }

    #[test]
    fn shift_and_claims() {
        key("key.keyboard.left.shift", true);
        assert!(shift());
        key("key.keyboard.left.shift", false);
        key("key.keyboard.a", true);
        assert!(!shift());
        claim(7, [96.0, 98.0]);
        assert!(claimed(7, [96.0, 98.0]));
        assert!(!claimed(8, [96.0, 98.0]));
        assert!(!claimed(7, [64.0, 66.0]));
    }
}
