// HUD+ — an ultimate HUD for Horizon: AppleSkin, status-effect timers,
// inventory, armour, coordinates, keystrokes, performance, player vitals,
// held-item info, food tooltips, a drag-and-drop layout editor and a large
// settings screen. The file is both this mod's Cargo library (for its tests)
// and the module the bundle adds to the game as game::official_hud_plus.

use std::path::Path;

pub const ID: &str = "hud_plus";
pub const NAME: &str = "HUD+";
pub const VERSION: &str = "4";
pub const DESCRIPTION: &str = "Ultimate configurable HUD: AppleSkin hunger and saturation, effect timers, inventory, armour, coordinates, keystrokes, FPS, time, speed, vitals, held-item info, food tooltips and a drag-and-drop layout editor.";
pub const CONFIG_FILE: &str = "hud_plus.json";
pub const CONFIG_VERSION: u32 = 1;

// ---------------------------------------------------------------------------
// Small geometry and colour helpers
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }
    pub fn right(self) -> f32 {
        self.x + self.w
    }
    pub fn bottom(self) -> f32 {
        self.y + self.h
    }
    pub fn contains(self, p: [f32; 2]) -> bool {
        p[0] >= self.x && p[0] < self.right() && p[1] >= self.y && p[1] < self.bottom()
    }
}

pub const fn argb(a: u8, r: u8, g: u8, b: u8) -> u32 {
    (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32
}

pub fn with_alpha(color: u32, alpha: u8) -> u32 {
    (color & 0x00FF_FFFF) | ((alpha as u32) << 24)
}

/// Opaque white: the text colour that reads on any theme.
pub const WHITE: u32 = 0xFFFF_FFFF;

// ---------------------------------------------------------------------------
// Enumerations the settings cycle through
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anchor {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Anchor {
    pub const ALL: [Anchor; 9] = [
        Anchor::TopLeft,
        Anchor::TopCenter,
        Anchor::TopRight,
        Anchor::MiddleLeft,
        Anchor::MiddleCenter,
        Anchor::MiddleRight,
        Anchor::BottomLeft,
        Anchor::BottomCenter,
        Anchor::BottomRight,
    ];
    pub fn next(self) -> Self {
        let i = Self::ALL.iter().position(|a| *a == self).unwrap_or(0);
        Self::ALL[(i + 1) % Self::ALL.len()]
    }
    pub fn key(self) -> &'static str {
        match self {
            Anchor::TopLeft => "top_left",
            Anchor::TopCenter => "top_center",
            Anchor::TopRight => "top_right",
            Anchor::MiddleLeft => "middle_left",
            Anchor::MiddleCenter => "middle_center",
            Anchor::MiddleRight => "middle_right",
            Anchor::BottomLeft => "bottom_left",
            Anchor::BottomCenter => "bottom_center",
            Anchor::BottomRight => "bottom_right",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Anchor::TopLeft => "Top left",
            Anchor::TopCenter => "Top centre",
            Anchor::TopRight => "Top right",
            Anchor::MiddleLeft => "Middle left",
            Anchor::MiddleCenter => "Centre",
            Anchor::MiddleRight => "Middle right",
            Anchor::BottomLeft => "Bottom left",
            Anchor::BottomCenter => "Bottom centre",
            Anchor::BottomRight => "Bottom right",
        }
    }
    pub fn parse(s: &str) -> Self {
        Self::ALL
            .iter()
            .copied()
            .find(|a| a.key() == s)
            .unwrap_or(Anchor::TopLeft)
    }
    /// The top-left corner a `size` box takes in a `screen`, before its offset.
    pub fn base(self, size: (f32, f32), screen: (f32, f32)) -> (f32, f32) {
        let (w, h) = size;
        let (sw, sh) = screen;
        let x = match self {
            Anchor::TopLeft | Anchor::MiddleLeft | Anchor::BottomLeft => 0.0,
            Anchor::TopCenter | Anchor::MiddleCenter | Anchor::BottomCenter => (sw - w) * 0.5,
            Anchor::TopRight | Anchor::MiddleRight | Anchor::BottomRight => sw - w,
        };
        let y = match self {
            Anchor::TopLeft | Anchor::TopCenter | Anchor::TopRight => 0.0,
            Anchor::MiddleLeft | Anchor::MiddleCenter | Anchor::MiddleRight => (sh - h) * 0.5,
            Anchor::BottomLeft | Anchor::BottomCenter | Anchor::BottomRight => sh - h,
        };
        (x, y)
    }
    pub fn place(self, size: (f32, f32), offset: [f32; 2], screen: (f32, f32)) -> (f32, f32) {
        let (x, y) = self.base(size, screen);
        (x + offset[0], y + offset[1])
    }
    /// The anchor a box at `at` sits most naturally to.
    #[cfg(test)]
    pub fn nearest(at: (f32, f32), size: (f32, f32), screen: (f32, f32)) -> Self {
        let cx = at.0 + size.0 * 0.5;
        let cy = at.1 + size.1 * 0.5;
        let (sw, sh) = screen;
        let col = if cx < sw / 3.0 {
            0
        } else if cx > sw * 2.0 / 3.0 {
            2
        } else {
            1
        };
        let row = if cy < sh / 3.0 {
            0
        } else if cy > sh * 2.0 / 3.0 {
            2
        } else {
            1
        };
        Self::ALL[row * 3 + col]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
    Transparent,
    Neon,
    HighContrast,
    Amber,
}

impl Theme {
    pub const ALL: [Theme; 6] = [
        Theme::Dark,
        Theme::Light,
        Theme::Transparent,
        Theme::Neon,
        Theme::HighContrast,
        Theme::Amber,
    ];
    pub fn next(self) -> Self {
        let i = Self::ALL.iter().position(|t| *t == self).unwrap_or(0);
        Self::ALL[(i + 1) % Self::ALL.len()]
    }
    pub fn key(self) -> &'static str {
        match self {
            Theme::Dark => "dark",
            Theme::Light => "light",
            Theme::Transparent => "transparent",
            Theme::Neon => "neon",
            Theme::HighContrast => "high_contrast",
            Theme::Amber => "amber",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Theme::Dark => "Dark",
            Theme::Light => "Light",
            Theme::Transparent => "Transparent",
            Theme::Neon => "Neon",
            Theme::HighContrast => "High contrast",
            Theme::Amber => "Amber",
        }
    }
    pub fn parse(s: &str) -> Self {
        Self::ALL
            .iter()
            .copied()
            .find(|t| t.key() == s)
            .unwrap_or(Theme::Dark)
    }
    pub fn colours(self, opacity: u8) -> Colours {
        let a = opacity;
        match self {
            Theme::Dark => Colours {
                panel: argb(a, 12, 15, 21),
                header: argb(a.saturating_add(20), 22, 26, 34),
                text: 0xFFF3F6FA,
                muted: 0xFF9AA7B8,
                accent: 0xFF5CC8FF,
                good: 0xFF6EE07A,
                warn: 0xFFFFCB4D,
                bad: 0xFFFF6E6E,
                border: argb(64, 255, 255, 255),
                slot: argb(a.saturating_add(30).min(235), 45, 52, 66),
                shadow: 0xFF06080C,
            },
            Theme::Light => Colours {
                panel: argb(a, 235, 238, 242),
                header: argb(a.saturating_add(10), 215, 220, 228),
                text: 0xFF161A20,
                muted: 0xFF555D68,
                accent: 0xFF0066CC,
                good: 0xFF157A2B,
                warn: 0xFF8A5A00,
                bad: 0xFFB21E1E,
                border: argb(a.saturating_add(40), 70, 80, 95),
                slot: argb(a.saturating_add(15), 210, 215, 222),
                shadow: 0xFFFFFFFF,
            },
            Theme::Transparent => Colours {
                panel: 0x0000_0000,
                header: 0x2200_0000,
                text: 0xFFFF_FFFF,
                muted: 0xFFC8C8C8,
                accent: 0xFF66E0FF,
                good: 0xFF7CFF7C,
                warn: 0xFFFFE066,
                bad: 0xFFFF7070,
                border: 0x0000_0000,
                slot: 0x0000_0000,
                shadow: 0xFF000000,
            },
            Theme::Neon => Colours {
                panel: argb(a.min(180), 4, 0, 12),
                header: argb(a.min(210), 20, 0, 36),
                text: 0xFFE8FFFF,
                muted: 0xFF8FA8C0,
                accent: 0xFFFF4FD8,
                good: 0xFF6BFF8E,
                warn: 0xFFFFF35A,
                bad: 0xFFFF3355,
                border: argb(230, 255, 79, 216),
                slot: argb(a.saturating_add(20), 28, 0, 48),
                shadow: 0xFF120018,
            },
            Theme::HighContrast => Colours {
                panel: argb(a.max(190), 0, 0, 0),
                header: argb(a.max(210), 0, 0, 0),
                text: 0xFFFF_FFFF,
                muted: 0xFFFF_FFFF,
                accent: 0xFF00FFFF,
                good: 0xFF00FF00,
                warn: 0xFFFFFF00,
                bad: 0xFFFF0000,
                border: 0xFFFF_FFFF,
                slot: argb(a.max(220), 0, 0, 0),
                shadow: 0xFF000000,
            },
            Theme::Amber => Colours {
                panel: argb(a, 24, 14, 0),
                header: argb(a.saturating_add(12), 40, 23, 0),
                text: 0xFFFFC66D,
                muted: 0xFFC79A55,
                accent: 0xFFFFB000,
                good: 0xFF9BE86B,
                warn: 0xFFFFD75A,
                bad: 0xFFFF7043,
                border: argb(a.saturating_add(50), 255, 176, 0),
                slot: argb(a.saturating_add(20), 52, 30, 0),
                shadow: 0xFF120A00,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Colours {
    pub panel: u32,
    pub header: u32,
    pub text: u32,
    pub muted: u32,
    pub accent: u32,
    pub good: u32,
    pub warn: u32,
    pub bad: u32,
    pub border: u32,
    pub slot: u32,
    pub shadow: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Profile {
    Default,
    Minimal,
    Pvp,
    Builder,
    Streamer,
    Custom,
}

impl Profile {
    pub const ALL: [Profile; 6] = [
        Profile::Default,
        Profile::Minimal,
        Profile::Pvp,
        Profile::Builder,
        Profile::Streamer,
        Profile::Custom,
    ];
    pub fn key(self) -> &'static str {
        match self {
            Profile::Default => "default",
            Profile::Minimal => "minimal",
            Profile::Pvp => "pvp",
            Profile::Builder => "builder",
            Profile::Streamer => "streamer",
            Profile::Custom => "custom",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Profile::Default => "Default",
            Profile::Minimal => "Minimal",
            Profile::Pvp => "PvP",
            Profile::Builder => "Builder",
            Profile::Streamer => "Streamer",
            Profile::Custom => "Custom",
        }
    }
    pub fn next(self) -> Self {
        let i = Self::ALL.iter().position(|p| *p == self).unwrap_or(0);
        Self::ALL[(i + 1) % Self::ALL.len()]
    }
    pub fn parse(s: &str) -> Self {
        Self::ALL
            .iter()
            .copied()
            .find(|p| p.key() == s)
            .unwrap_or(Profile::Default)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hotkey {
    H,
    J,
    K,
    B,
    N,
    M,
    O,
    P,
    F6,
    F7,
    None,
}

impl Hotkey {
    pub const ALL: [Hotkey; 11] = [
        Hotkey::H,
        Hotkey::J,
        Hotkey::K,
        Hotkey::B,
        Hotkey::N,
        Hotkey::M,
        Hotkey::O,
        Hotkey::P,
        Hotkey::F6,
        Hotkey::F7,
        Hotkey::None,
    ];
    pub fn next(self) -> Self {
        let i = Self::ALL.iter().position(|h| *h == self).unwrap_or(0);
        Self::ALL[(i + 1) % Self::ALL.len()]
    }
    pub fn key(self) -> &'static str {
        match self {
            Hotkey::H => "key.keyboard.h",
            Hotkey::J => "key.keyboard.j",
            Hotkey::K => "key.keyboard.k",
            Hotkey::B => "key.keyboard.b",
            Hotkey::N => "key.keyboard.n",
            Hotkey::M => "key.keyboard.m",
            Hotkey::O => "key.keyboard.o",
            Hotkey::P => "key.keyboard.p",
            Hotkey::F6 => "key.keyboard.f6",
            Hotkey::F7 => "key.keyboard.f7",
            Hotkey::None => "",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Hotkey::H => "H",
            Hotkey::J => "J",
            Hotkey::K => "K",
            Hotkey::B => "B",
            Hotkey::N => "N",
            Hotkey::M => "M",
            Hotkey::O => "O",
            Hotkey::P => "P",
            Hotkey::F6 => "F6",
            Hotkey::F7 => "F7",
            Hotkey::None => "Off",
        }
    }
    pub fn parse(s: &str) -> Self {
        match s {
            "key.keyboard.h" => Hotkey::H,
            "key.keyboard.j" => Hotkey::J,
            "key.keyboard.k" => Hotkey::K,
            "key.keyboard.b" => Hotkey::B,
            "key.keyboard.n" => Hotkey::N,
            "key.keyboard.m" => Hotkey::M,
            "key.keyboard.o" => Hotkey::O,
            "key.keyboard.p" => Hotkey::P,
            "key.keyboard.f6" => Hotkey::F6,
            "key.keyboard.f7" => Hotkey::F7,
            _ => Hotkey::None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeFormat {
    Clock,
    Seconds,
    Ticks,
    Both,
    Off,
}

impl TimeFormat {
    pub const ALL: [TimeFormat; 5] = [
        TimeFormat::Clock,
        TimeFormat::Seconds,
        TimeFormat::Ticks,
        TimeFormat::Both,
        TimeFormat::Off,
    ];
    pub fn next(self) -> Self {
        match self {
            TimeFormat::Clock => TimeFormat::Seconds,
            TimeFormat::Seconds => TimeFormat::Ticks,
            TimeFormat::Ticks => TimeFormat::Both,
            TimeFormat::Both => TimeFormat::Off,
            TimeFormat::Off => TimeFormat::Clock,
        }
    }
    pub fn key(self) -> &'static str {
        match self {
            TimeFormat::Clock => "clock",
            TimeFormat::Seconds => "seconds",
            TimeFormat::Ticks => "ticks",
            TimeFormat::Both => "both",
            TimeFormat::Off => "off",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            TimeFormat::Clock => "mm:ss",
            TimeFormat::Seconds => "seconds",
            TimeFormat::Ticks => "ticks",
            TimeFormat::Both => "mm:ss + ticks",
            TimeFormat::Off => "Off",
        }
    }
    pub fn parse(s: &str) -> Self {
        match s {
            "seconds" => TimeFormat::Seconds,
            "ticks" => TimeFormat::Ticks,
            "both" => TimeFormat::Both,
            "off" => TimeFormat::Off,
            _ => TimeFormat::Clock,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EffectSort {
    Duration,
    Name,
    Amplifier,
}

impl EffectSort {
    pub const ALL: [EffectSort; 3] = [
        EffectSort::Duration,
        EffectSort::Name,
        EffectSort::Amplifier,
    ];
    pub fn next(self) -> Self {
        match self {
            EffectSort::Duration => EffectSort::Name,
            EffectSort::Name => EffectSort::Amplifier,
            EffectSort::Amplifier => EffectSort::Duration,
        }
    }
    pub fn key(self) -> &'static str {
        match self {
            EffectSort::Duration => "duration",
            EffectSort::Name => "name",
            EffectSort::Amplifier => "amplifier",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            EffectSort::Duration => "Shortest first",
            EffectSort::Name => "Name",
            EffectSort::Amplifier => "Strongest first",
        }
    }
    pub fn parse(s: &str) -> Self {
        match s {
            "name" => EffectSort::Name,
            "amplifier" => EffectSort::Amplifier,
            _ => EffectSort::Duration,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InventoryWhen {
    Always,
    ContainerOpen,
    SurvivalOnly,
}

impl InventoryWhen {
    pub const ALL: [InventoryWhen; 3] = [
        InventoryWhen::Always,
        InventoryWhen::ContainerOpen,
        InventoryWhen::SurvivalOnly,
    ];
    pub fn next(self) -> Self {
        match self {
            InventoryWhen::Always => InventoryWhen::ContainerOpen,
            InventoryWhen::ContainerOpen => InventoryWhen::SurvivalOnly,
            InventoryWhen::SurvivalOnly => InventoryWhen::Always,
        }
    }
    pub fn key(self) -> &'static str {
        match self {
            InventoryWhen::Always => "always",
            InventoryWhen::ContainerOpen => "container",
            InventoryWhen::SurvivalOnly => "survival",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            InventoryWhen::Always => "Always",
            InventoryWhen::ContainerOpen => "With a container",
            InventoryWhen::SurvivalOnly => "Survival only",
        }
    }
    pub fn parse(s: &str) -> Self {
        match s {
            "container" => InventoryWhen::ContainerOpen,
            "survival" => InventoryWhen::SurvivalOnly,
            _ => InventoryWhen::Always,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemDurability {
    Off,
    Bar,
    Percent,
    Both,
}

impl ItemDurability {
    pub const ALL: [ItemDurability; 4] = [
        ItemDurability::Off,
        ItemDurability::Bar,
        ItemDurability::Percent,
        ItemDurability::Both,
    ];
    pub fn next(self) -> Self {
        match self {
            ItemDurability::Off => ItemDurability::Bar,
            ItemDurability::Bar => ItemDurability::Percent,
            ItemDurability::Percent => ItemDurability::Both,
            ItemDurability::Both => ItemDurability::Off,
        }
    }
    pub fn key(self) -> &'static str {
        match self {
            ItemDurability::Off => "off",
            ItemDurability::Bar => "bar",
            ItemDurability::Percent => "percent",
            ItemDurability::Both => "both",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            ItemDurability::Off => "Off",
            ItemDurability::Bar => "Bar",
            ItemDurability::Percent => "Percent",
            ItemDurability::Both => "Bar + percent",
        }
    }
    pub fn parse(s: &str) -> Self {
        match s {
            "bar" => ItemDurability::Bar,
            "percent" => ItemDurability::Percent,
            "both" => ItemDurability::Both,
            _ => ItemDurability::Off,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CountMode {
    Off,
    Stack,
    Always,
}

impl CountMode {
    pub const ALL: [CountMode; 3] = [CountMode::Off, CountMode::Stack, CountMode::Always];
    pub fn next(self) -> Self {
        match self {
            CountMode::Off => CountMode::Stack,
            CountMode::Stack => CountMode::Always,
            CountMode::Always => CountMode::Off,
        }
    }
    pub fn key(self) -> &'static str {
        match self {
            CountMode::Off => "off",
            CountMode::Stack => "stack",
            CountMode::Always => "always",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            CountMode::Off => "Off",
            CountMode::Stack => "Stacks",
            CountMode::Always => "Always",
        }
    }
    pub fn parse(s: &str) -> Self {
        match s {
            "stack" => CountMode::Stack,
            "always" => CountMode::Always,
            _ => CountMode::Off,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModuleId {
    AppleSkin,
    Effects,
    Inventory,
    Armor,
    Coords,
    Keys,
    Info,
    Health,
    Held,
}

impl ModuleId {
    pub const ALL: [ModuleId; 9] = [
        ModuleId::AppleSkin,
        ModuleId::Effects,
        ModuleId::Inventory,
        ModuleId::Armor,
        ModuleId::Coords,
        ModuleId::Keys,
        ModuleId::Info,
        ModuleId::Health,
        ModuleId::Held,
    ];
    pub fn title(self) -> &'static str {
        match self {
            ModuleId::AppleSkin => "AppleSkin",
            ModuleId::Effects => "Effect timers",
            ModuleId::Inventory => "Inventory",
            ModuleId::Armor => "Armour & durability",
            ModuleId::Coords => "Coordinates",
            ModuleId::Keys => "Keystrokes",
            ModuleId::Info => "Performance & time",
            ModuleId::Health => "Player vitals",
            ModuleId::Held => "Held items",
        }
    }
}

/// How much of a module to draw. `Standard` follows the module's own
/// switches; the other three are presets over them, so a player can make a
/// module tiny, tidy or exhaustive without touching a dozen toggles.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum View {
    Minimal,
    Compact,
    Standard,
    Full,
}

impl View {
    pub const ALL: [View; 4] = [View::Minimal, View::Compact, View::Standard, View::Full];
    pub fn key(self) -> &'static str {
        match self {
            View::Minimal => "minimal",
            View::Compact => "compact",
            View::Standard => "standard",
            View::Full => "full",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            View::Minimal => "Minimal",
            View::Compact => "Compact",
            View::Standard => "Standard",
            View::Full => "Full",
        }
    }
    pub fn next(self) -> Self {
        match self {
            View::Minimal => View::Compact,
            View::Compact => View::Standard,
            View::Standard => View::Full,
            View::Full => View::Minimal,
        }
    }
    pub fn parse(s: &str) -> Self {
        match s {
            "minimal" => View::Minimal,
            "compact" => View::Compact,
            "full" => View::Full,
            _ => View::Standard,
        }
    }
}

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ModuleBox {
    pub enabled: bool,
    pub anchor: Anchor,
    pub offset: [f32; 2],
    pub scale: f32,
    pub background: bool,
    pub border: bool,
}

impl ModuleBox {
    /// A module that starts disabled: only the modules a first-time player
    /// needs are on, the rest wait in the layout editor.
    pub const fn new(anchor: Anchor, offset: [f32; 2]) -> Self {
        Self {
            enabled: false,
            anchor,
            offset,
            scale: 1.0,
            background: true,
            border: false,
        }
    }
    pub const fn on(mut self) -> Self {
        self.enabled = true;
        self
    }
    /// Text on the world instead of in a panel, for the modules near the
    /// hotbar where a box would cover the game.
    pub const fn bare(mut self) -> Self {
        self.background = false;
        self
    }
    pub fn next_scale(self, up: bool) -> Self {
        let step = 0.05;
        let scale = if up {
            self.scale + step
        } else {
            self.scale - step
        };
        Self {
            scale: (scale * 100.0).round().clamp(50.0, 250.0) / 100.0,
            ..self
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AppleSkin {
    pub module: ModuleBox,
    pub view: View,
    /// AppleSkin's golden outline over the vanilla hunger icons.
    pub show_saturation: bool,
    /// The hunger and saturation the held item would restore, flashing on
    /// the hunger bar.
    pub show_preview: bool,
    /// The numbers above the bar (`17 / 20   Sat 4.2`), which the original
    /// mod does not draw; off by default.
    pub show_numbers: bool,
    pub show_exhaustion: bool,
    pub show_regen: bool,
    pub low_threshold: i32,
    pub compact: bool,
    /// Draw the hunger icons too, for a HUD that hides vanilla's.
    pub show_bars: bool,
}

impl Default for AppleSkin {
    fn default() -> Self {
        Self {
            view: View::Standard,
            // `Hud` puts the hunger bar's right edge at `w / 2 + 91` and its
            // icons at `h - 39`; the box is 81 wide (ten 9 px icons 8 apart),
            // and the padding keeps the icons on exactly that spot.
            module: ModuleBox::new(Anchor::BottomCenter, [50.5, PADDING - 30.0])
                .on()
                .bare(),
            show_saturation: true,
            show_preview: true,
            show_numbers: false,
            show_exhaustion: false,
            show_regen: false,
            low_threshold: 6,
            compact: false,
            show_bars: false,
        }
    }
}

impl AppleSkin {
    pub fn effective(self) -> Self {
        match self.view {
            View::Minimal => Self {
                show_numbers: false,
                show_exhaustion: false,
                show_regen: false,
                show_bars: false,
                compact: true,
                ..self
            },
            View::Compact => Self {
                show_numbers: true,
                show_exhaustion: false,
                show_regen: false,
                show_bars: false,
                compact: true,
                ..self
            },
            View::Full => Self {
                show_saturation: true,
                show_preview: true,
                show_numbers: true,
                show_exhaustion: true,
                show_regen: true,
                show_bars: true,
                compact: false,
                ..self
            },
            View::Standard => self,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Effects {
    pub module: ModuleBox,
    pub view: View,
    pub hide_vanilla: bool,
    pub icons: bool,
    pub names: bool,
    pub amplifier: bool,
    pub time: TimeFormat,
    pub sort: EffectSort,
    pub warn_seconds: i32,
    pub max_rows: i32,
    pub group: bool,
    pub ambient: bool,
    pub expiring_only: bool,
    pub horizontal: bool,
}

impl Default for Effects {
    fn default() -> Self {
        Self {
            view: View::Standard,
            module: ModuleBox::new(Anchor::TopRight, [-4.0, 4.0]).on(),
            hide_vanilla: true,
            icons: true,
            names: true,
            amplifier: true,
            time: TimeFormat::Clock,
            sort: EffectSort::Duration,
            warn_seconds: 10,
            max_rows: 8,
            group: false,
            ambient: true,
            expiring_only: false,
            horizontal: false,
        }
    }
}

impl Effects {
    pub fn effective(self) -> Self {
        let time = if self.time == TimeFormat::Off {
            TimeFormat::Clock
        } else {
            self.time
        };
        match self.view {
            View::Minimal => Self {
                icons: true,
                names: false,
                amplifier: false,
                ambient: false,
                time: TimeFormat::Off,
                max_rows: if self.max_rows <= 0 {
                    3
                } else {
                    self.max_rows.min(3)
                },
                ..self
            },
            View::Compact => Self {
                icons: true,
                names: true,
                amplifier: false,
                ambient: false,
                time,
                max_rows: if self.max_rows <= 0 {
                    6
                } else {
                    self.max_rows.min(6)
                },
                horizontal: self.horizontal,
                ..self
            },
            View::Full => Self {
                icons: true,
                names: true,
                amplifier: true,
                ambient: true,
                time,
                max_rows: 0,
                ..self
            },
            View::Standard => self,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Inventory {
    pub module: ModuleBox,
    pub view: View,
    pub columns: i32,
    pub hotbar: bool,
    /// The main 27-slot grid; off in the Minimal view, which keeps the hotbar.
    pub grid: bool,
    pub armor: bool,
    pub offhand: bool,
    pub empty: bool,
    pub counts: CountMode,
    pub durability: ItemDurability,
    pub highlight: bool,
    pub when: InventoryWhen,
    pub creative: bool,
    pub slot_size: f32,
    pub gap: f32,
    pub compact_rows: bool,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            view: View::Standard,
            module: ModuleBox::new(Anchor::BottomLeft, [4.0, -26.0]).on(),
            columns: 9,
            hotbar: false,
            grid: true,
            armor: false,
            offhand: true,
            empty: false,
            counts: CountMode::Stack,
            durability: ItemDurability::Bar,
            highlight: true,
            when: InventoryWhen::Always,
            creative: true,
            slot_size: 16.0,
            gap: 1.0,
            compact_rows: true,
        }
    }
}

impl Inventory {
    pub fn effective(self) -> Self {
        match self.view {
            View::Minimal => Self {
                columns: 9,
                hotbar: true,
                grid: false,
                armor: false,
                offhand: false,
                empty: false,
                counts: CountMode::Off,
                durability: ItemDurability::Off,
                highlight: true,
                compact_rows: true,
                ..self
            },
            View::Compact => Self {
                counts: CountMode::Stack,
                durability: ItemDurability::Bar,
                empty: false,
                compact_rows: true,
                ..self
            },
            View::Full => Self {
                columns: 9,
                hotbar: true,
                grid: true,
                armor: true,
                offhand: true,
                empty: true,
                counts: CountMode::Always,
                durability: ItemDurability::Both,
                highlight: true,
                compact_rows: false,
                ..self
            },
            View::Standard => self,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Armor {
    pub module: ModuleBox,
    pub view: View,
    pub names: bool,
    pub percent: bool,
    pub bars: bool,
    pub held: bool,
    pub threshold: i32,
}

impl Default for Armor {
    fn default() -> Self {
        Self {
            view: View::Standard,
            module: ModuleBox::new(Anchor::BottomRight, [-4.0, -26.0]).on(),
            names: true,
            percent: true,
            bars: true,
            // The held item is the Held module's job: showing it here too put
            // a second copy of its name in the corner.
            held: false,
            threshold: 20,
        }
    }
}

impl Armor {
    pub fn effective(self) -> Self {
        match self.view {
            View::Minimal => Self {
                names: false,
                percent: false,
                bars: false,
                held: false,
                ..self
            },
            View::Compact => Self {
                names: false,
                percent: true,
                bars: false,
                held: false,
                ..self
            },
            View::Full => Self {
                names: true,
                percent: true,
                bars: true,
                held: true,
                ..self
            },
            View::Standard => self,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coords {
    pub module: ModuleBox,
    pub view: View,
    pub chunk: bool,
    pub facing: bool,
    pub dimension: bool,
    pub nether: bool,
    pub precision: i32,
    pub yaw_degrees: bool,
}

impl Default for Coords {
    fn default() -> Self {
        Self {
            view: View::Standard,
            module: ModuleBox::new(Anchor::TopLeft, [4.0, 4.0]).on().bare(),
            chunk: false,
            facing: true,
            dimension: false,
            nether: true,
            precision: 1,
            yaw_degrees: false,
        }
    }
}

impl Coords {
    pub fn effective(self) -> Self {
        match self.view {
            View::Minimal => Self {
                chunk: false,
                facing: false,
                dimension: false,
                nether: false,
                yaw_degrees: false,
                precision: 0,
                ..self
            },
            View::Compact => Self {
                chunk: false,
                facing: true,
                dimension: false,
                nether: false,
                yaw_degrees: false,
                precision: self.precision.min(1),
                ..self
            },
            View::Full => Self {
                chunk: true,
                facing: true,
                dimension: true,
                nether: true,
                yaw_degrees: true,
                precision: self.precision.max(2),
                ..self
            },
            View::Standard => self,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Keys {
    pub module: ModuleBox,
    pub view: View,
    pub mouse: bool,
    pub space: bool,
    pub sneak_sprint: bool,
    pub cps: bool,
    pub labels: bool,
    pub box_size: f32,
}

impl Default for Keys {
    fn default() -> Self {
        Self {
            view: View::Standard,
            module: ModuleBox::new(Anchor::MiddleLeft, [4.0, 0.0]).on().bare(),
            mouse: true,
            space: true,
            sneak_sprint: false,
            cps: false,
            labels: true,
            box_size: 17.0,
        }
    }
}

impl Keys {
    pub fn effective(self) -> Self {
        match self.view {
            View::Minimal => Self {
                mouse: false,
                space: false,
                sneak_sprint: false,
                cps: false,
                labels: true,
                ..self
            },
            View::Compact => Self {
                mouse: true,
                space: false,
                sneak_sprint: false,
                cps: false,
                ..self
            },
            View::Full => Self {
                mouse: true,
                space: true,
                sneak_sprint: true,
                cps: true,
                labels: true,
                ..self
            },
            View::Standard => self,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Info {
    pub module: ModuleBox,
    pub view: View,
    pub fps: bool,
    pub frame_ms: bool,
    pub low_fps: bool,
    pub time: bool,
    pub real_time: bool,
    pub speed: bool,
    pub compass: bool,
    pub clock24: bool,
}

impl Default for Info {
    fn default() -> Self {
        Self {
            view: View::Standard,
            module: ModuleBox::new(Anchor::TopCenter, [0.0, 4.0]).bare(),
            fps: true,
            frame_ms: true,
            low_fps: false,
            time: true,
            real_time: false,
            speed: true,
            compass: true,
            clock24: true,
        }
    }
}

impl Info {
    pub fn effective(self) -> Self {
        match self.view {
            View::Minimal => Self {
                fps: true,
                frame_ms: false,
                low_fps: false,
                time: false,
                real_time: false,
                speed: false,
                compass: false,
                ..self
            },
            View::Compact => Self {
                fps: true,
                frame_ms: false,
                low_fps: false,
                time: true,
                real_time: false,
                speed: false,
                compass: false,
                ..self
            },
            View::Full => Self {
                fps: true,
                frame_ms: true,
                low_fps: true,
                time: true,
                real_time: true,
                speed: true,
                compass: true,
                ..self
            },
            View::Standard => self,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub module: ModuleBox,
    pub view: View,
    pub absorption: bool,
    pub armor_value: bool,
    pub food: bool,
    pub air: bool,
    pub xp: bool,
    pub bars: bool,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            view: View::Standard,
            module: ModuleBox::new(Anchor::BottomLeft, [4.0, -44.0]).bare(),
            absorption: true,
            armor_value: true,
            food: true,
            air: true,
            xp: true,
            bars: true,
        }
    }
}

impl Health {
    pub fn effective(self) -> Self {
        match self.view {
            View::Minimal => Self {
                absorption: false,
                armor_value: false,
                food: false,
                air: false,
                xp: false,
                bars: false,
                ..self
            },
            View::Compact => Self {
                absorption: false,
                armor_value: false,
                food: true,
                air: false,
                xp: false,
                bars: true,
                ..self
            },
            View::Full => Self {
                absorption: true,
                armor_value: true,
                food: true,
                air: true,
                xp: true,
                bars: true,
                ..self
            },
            View::Standard => self,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Held {
    pub module: ModuleBox,
    pub view: View,
    pub offhand: bool,
    pub arrows: bool,
    pub totems: bool,
    pub durability: ItemDurability,
    pub counts: bool,
}

impl Default for Held {
    fn default() -> Self {
        Self {
            view: View::Standard,
            module: ModuleBox::new(Anchor::MiddleRight, [-4.0, 62.0]).bare(),
            offhand: true,
            arrows: true,
            totems: true,
            durability: ItemDurability::Both,
            counts: true,
        }
    }
}

impl Held {
    pub fn effective(self) -> Self {
        match self.view {
            View::Minimal => Self {
                offhand: false,
                arrows: false,
                totems: false,
                counts: false,
                durability: ItemDurability::Off,
                ..self
            },
            View::Compact => Self {
                offhand: false,
                arrows: false,
                totems: false,
                counts: true,
                durability: ItemDurability::Percent,
                ..self
            },
            View::Full => Self {
                offhand: true,
                arrows: true,
                totems: true,
                counts: true,
                durability: ItemDurability::Both,
                ..self
            },
            View::Standard => self,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tooltips {
    pub food: bool,
    pub sneak_details: bool,
}

impl Default for Tooltips {
    fn default() -> Self {
        Self {
            food: true,
            sneak_details: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Vanilla HUD layers: move and scale Minecraft's own widgets
// ---------------------------------------------------------------------------

/// Every vanilla HUD layer the client wraps in `gui_layer`. The editor moves
/// and scales each one by transforming the vertices the layer drew.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VanillaId {
    Crosshair,
    Hotbar,
    Effects,
    BossBars,
    ContextualBar,
    PlayerHealth,
    SelectedItemName,
    OverlayMessage,
    Title,
    Scoreboard,
    Subtitle,
    Chat,
    TabList,
    VehicleHealth,
}

impl VanillaId {
    pub const ALL: [VanillaId; 14] = [
        VanillaId::Crosshair,
        VanillaId::Hotbar,
        VanillaId::Effects,
        VanillaId::BossBars,
        VanillaId::ContextualBar,
        VanillaId::PlayerHealth,
        VanillaId::SelectedItemName,
        VanillaId::VehicleHealth,
        VanillaId::OverlayMessage,
        VanillaId::Title,
        VanillaId::Subtitle,
        VanillaId::Scoreboard,
        VanillaId::Chat,
        VanillaId::TabList,
    ];

    pub fn key(self) -> &'static str {
        match self {
            VanillaId::Crosshair => "crosshair",
            VanillaId::Hotbar => "hotbar",
            VanillaId::Effects => "effects",
            VanillaId::BossBars => "boss_bars",
            VanillaId::ContextualBar => "contextual_bar",
            VanillaId::PlayerHealth => "player_health",
            VanillaId::SelectedItemName => "selected_item_name",
            VanillaId::OverlayMessage => "overlay_message",
            VanillaId::Title => "title",
            VanillaId::Scoreboard => "scoreboard",
            VanillaId::Subtitle => "subtitle",
            VanillaId::Chat => "chat",
            VanillaId::TabList => "tab_list",
            VanillaId::VehicleHealth => "vehicle_health",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            VanillaId::Crosshair => "Crosshair",
            VanillaId::Hotbar => "Hotbar",
            VanillaId::Effects => "Effect icons",
            VanillaId::BossBars => "Boss bars",
            VanillaId::ContextualBar => "XP / locator bar",
            VanillaId::PlayerHealth => "Health & hunger",
            VanillaId::SelectedItemName => "Item name",
            VanillaId::OverlayMessage => "Action bar",
            VanillaId::Title => "Title",
            VanillaId::Scoreboard => "Scoreboard",
            VanillaId::Subtitle => "Subtitles",
            VanillaId::Chat => "Chat",
            VanillaId::TabList => "Player list",
            VanillaId::VehicleHealth => "Mount health",
        }
    }

    /// The client's `gui_layer` id.
    pub fn layer(self) -> &'static str {
        match self {
            VanillaId::Crosshair => "minecraft:crosshair",
            VanillaId::Hotbar => "minecraft:hotbar",
            VanillaId::Effects => "minecraft:effects",
            VanillaId::BossBars => "minecraft:boss_overlay",
            VanillaId::ContextualBar => "minecraft:contextual_info_bar",
            VanillaId::PlayerHealth => "minecraft:player_health",
            VanillaId::SelectedItemName => "minecraft:selected_item_name",
            VanillaId::OverlayMessage => "minecraft:overlay_message",
            VanillaId::Title => "minecraft:title",
            VanillaId::Scoreboard => "minecraft:scoreboard_sidebar",
            VanillaId::Subtitle => "minecraft:subtitle_overlay",
            VanillaId::Chat => "minecraft:chat",
            VanillaId::TabList => "minecraft:tab_list",
            VanillaId::VehicleHealth => "minecraft:vehicle_health",
        }
    }

    pub fn from_layer(layer: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|id| id.layer() == layer)
    }

    /// The point of the widget that scaling keeps in place.
    pub fn pivot(self) -> (f32, f32) {
        match self {
            VanillaId::Crosshair | VanillaId::Hotbar | VanillaId::Title => (0.5, 0.5),
            VanillaId::Effects | VanillaId::Scoreboard => (1.0, 0.0),
            VanillaId::TabList => (0.5, 0.0),
            VanillaId::Chat => (0.0, 1.0),
            _ => (0.5, 1.0),
        }
    }

    /// Where vanilla draws it, before any move or scale.
    pub fn default_box(self, screen: (f32, f32)) -> Rect {
        let (w, h) = screen;
        match self {
            VanillaId::Crosshair => Rect::new(w / 2.0 - 8.0, h / 2.0 - 8.0, 16.0, 32.0),
            VanillaId::Hotbar => Rect::new(w / 2.0 - 91.0, h - 22.0, 182.0, 22.0),
            VanillaId::Effects => Rect::new(w - 160.0, 0.0, 160.0, 54.0),
            VanillaId::BossBars => Rect::new(w / 2.0 - 91.0, 10.0, 182.0, 40.0),
            VanillaId::ContextualBar => Rect::new(w / 2.0 - 91.0, h - 39.0, 182.0, 17.0),
            VanillaId::PlayerHealth => Rect::new(w / 2.0 - 91.0, h - 39.0, 182.0, 17.0),
            // `Hud.extractSelectedItemName`: centred, `h - max(barHeight, 59)`.
            VanillaId::SelectedItemName => Rect::new(w / 2.0 - 110.0, h - 61.0, 220.0, 14.0),
            VanillaId::VehicleHealth => Rect::new(w / 2.0 - 91.0, h - 60.0, 182.0, 10.0),
            VanillaId::OverlayMessage => Rect::new(w / 2.0 - 110.0, h - 60.0, 220.0, 12.0),
            VanillaId::Title => Rect::new(w / 2.0 - 130.0, h / 2.0 - 62.0, 260.0, 84.0),
            VanillaId::Scoreboard => Rect::new(w - 160.0, 16.0, 156.0, 140.0),
            VanillaId::Subtitle => {
                // `SubtitleOverlay` right-aligns the captions above the hotbar,
                // not at the middle of the screen. The box follows the real
                // place so the editor's outline sits on the captions.
                let caption_w = 240.0_f32.min(w - 8.0);
                Rect::new(w - caption_w - 1.0, h - 41.0, caption_w, 12.0)
            }
            VanillaId::Chat => Rect::new(0.0, h - 140.0, 320.0, 140.0),
            VanillaId::TabList => Rect::new(w / 2.0 - 160.0, 6.0, 320.0, 140.0),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VanillaElement {
    /// Pixels away from vanilla's own place.
    pub offset: [f32; 2],
    pub scale: f32,
    /// Off leaves vanilla's placement alone.
    pub on: bool,
}

impl Default for VanillaElement {
    fn default() -> Self {
        Self {
            offset: [0.0, 0.0],
            scale: 1.0,
            on: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VanillaHud {
    pub enabled: bool,
    pub elements: [VanillaElement; 15],
}

impl Default for VanillaHud {
    fn default() -> Self {
        Self {
            enabled: true,
            elements: [VanillaElement::default(); 15],
        }
    }
}

impl VanillaHud {
    pub fn get(&self, id: VanillaId) -> VanillaElement {
        self.elements[id.index()]
    }
    pub fn get_mut(&mut self, id: VanillaId) -> &mut VanillaElement {
        &mut self.elements[id.index()]
    }
}

impl VanillaId {
    pub fn index(self) -> usize {
        Self::ALL
            .iter()
            .position(|id| *id == self)
            .unwrap_or_default()
    }
}

/// The box a widget now takes: vanilla's place moved by `offset` and scaled
/// around its pivot.
pub fn vanilla_rect(cfg: &Config, id: VanillaId, screen: (f32, f32)) -> Rect {
    let el = cfg.vanilla.get(id);
    let d = id.default_box(screen);
    let (px, py) = (d.x + d.w * id.pivot().0, d.y + d.h * id.pivot().1);
    let s = el.scale;
    Rect::new(
        px + (d.x - px) * s + el.offset[0],
        py + (d.y - py) * s + el.offset[1],
        d.w * s,
        d.h * s,
    )
}

/// Everything the layer hook needs to transform one widget's vertices.
pub fn vanilla_layer_rects(
    cfg: &Config,
    layer: &str,
    screen: (f32, f32),
) -> Option<(Rect, Rect, f32)> {
    if !cfg.enabled || !cfg.vanilla.enabled {
        return None;
    }
    let id = VanillaId::from_layer(layer)?;
    let el = cfg.vanilla.get(id);
    if !el.on {
        return None;
    }
    let d = id.default_box(screen);
    let t = vanilla_rect(cfg, id, screen);
    Some((d, t, el.scale))
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Config {
    pub version: u32,
    pub enabled: bool,
    pub profile: Profile,
    pub theme: Theme,
    pub opacity: u8,
    pub text_shadow: bool,
    pub editor_hotkey: Hotkey,
    pub settings_hotkey: Hotkey,
    pub snap: bool,
    pub grid: u8,
    pub show_help: bool,
    pub apple_skin: AppleSkin,
    pub effects: Effects,
    pub inventory: Inventory,
    pub armor: Armor,
    pub coords: Coords,
    pub keys: Keys,
    pub info: Info,
    pub health: Health,
    pub held: Held,
    pub tooltips: Tooltips,
    pub vanilla: VanillaHud,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: CONFIG_VERSION,
            enabled: true,
            profile: Profile::Default,
            theme: Theme::Dark,
            opacity: 150,
            text_shadow: true,
            editor_hotkey: Hotkey::H,
            settings_hotkey: Hotkey::N,
            snap: true,
            grid: 8,
            show_help: true,
            apple_skin: AppleSkin::default(),
            effects: Effects::default(),
            inventory: Inventory::default(),
            armor: Armor::default(),
            coords: Coords::default(),
            keys: Keys::default(),
            info: Info::default(),
            health: Health::default(),
            held: Held::default(),
            tooltips: Tooltips::default(),
            vanilla: VanillaHud::default(),
        }
    }
}

impl Config {
    pub fn module(&self, id: ModuleId) -> &ModuleBox {
        match id {
            ModuleId::AppleSkin => &self.apple_skin.module,
            ModuleId::Effects => &self.effects.module,
            ModuleId::Inventory => &self.inventory.module,
            ModuleId::Armor => &self.armor.module,
            ModuleId::Coords => &self.coords.module,
            ModuleId::Keys => &self.keys.module,
            ModuleId::Info => &self.info.module,
            ModuleId::Health => &self.health.module,
            ModuleId::Held => &self.held.module,
        }
    }
    pub fn module_mut(&mut self, id: ModuleId) -> &mut ModuleBox {
        match id {
            ModuleId::AppleSkin => &mut self.apple_skin.module,
            ModuleId::Effects => &mut self.effects.module,
            ModuleId::Inventory => &mut self.inventory.module,
            ModuleId::Armor => &mut self.armor.module,
            ModuleId::Coords => &mut self.coords.module,
            ModuleId::Keys => &mut self.keys.module,
            ModuleId::Info => &mut self.info.module,
            ModuleId::Health => &mut self.health.module,
            ModuleId::Held => &mut self.held.module,
        }
    }
    pub fn view(&self, id: ModuleId) -> View {
        match id {
            ModuleId::AppleSkin => self.apple_skin.view,
            ModuleId::Effects => self.effects.view,
            ModuleId::Inventory => self.inventory.view,
            ModuleId::Armor => self.armor.view,
            ModuleId::Coords => self.coords.view,
            ModuleId::Keys => self.keys.view,
            ModuleId::Info => self.info.view,
            ModuleId::Health => self.health.view,
            ModuleId::Held => self.held.view,
        }
    }
    pub fn view_mut(&mut self, id: ModuleId) -> &mut View {
        match id {
            ModuleId::AppleSkin => &mut self.apple_skin.view,
            ModuleId::Effects => &mut self.effects.view,
            ModuleId::Inventory => &mut self.inventory.view,
            ModuleId::Armor => &mut self.armor.view,
            ModuleId::Coords => &mut self.coords.view,
            ModuleId::Keys => &mut self.keys.view,
            ModuleId::Info => &mut self.info.view,
            ModuleId::Health => &mut self.health.view,
            ModuleId::Held => &mut self.held.view,
        }
    }
    pub fn reset_layout(&mut self) {
        let d = Config::default();
        for id in ModuleId::ALL {
            *self.module_mut(id) = *d.module(id);
        }
    }
    pub fn apply_profile(&mut self, profile: Profile) {
        // Custom means "keep what I arranged": choosing it must not wipe the
        // layout, only name it.
        if profile == Profile::Custom {
            self.profile = Profile::Custom;
            return;
        }
        let d = Config::default();
        self.profile = profile;
        for id in ModuleId::ALL {
            *self.module_mut(id) = *d.module(id);
        }
        let set = |cfg: &mut Config, on: &[ModuleId]| {
            for id in ModuleId::ALL {
                cfg.module_mut(id).enabled = on.contains(&id);
            }
        };
        match profile {
            Profile::Default => {}
            Profile::Minimal => set(
                self,
                &[ModuleId::Coords, ModuleId::Effects, ModuleId::AppleSkin],
            ),
            Profile::Pvp => set(
                self,
                &[
                    ModuleId::AppleSkin,
                    ModuleId::Effects,
                    ModuleId::Armor,
                    ModuleId::Keys,
                    ModuleId::Health,
                ],
            ),
            Profile::Builder => set(
                self,
                &[
                    ModuleId::Coords,
                    ModuleId::Info,
                    ModuleId::Held,
                    ModuleId::Inventory,
                    ModuleId::Keys,
                ],
            ),
            Profile::Streamer => {
                set(
                    self,
                    &[
                        ModuleId::Coords,
                        ModuleId::Info,
                        ModuleId::AppleSkin,
                        ModuleId::Effects,
                    ],
                );
                self.keys.module.enabled = false;
                self.inventory.module.enabled = false;
                self.held.module.enabled = false;
            }
            Profile::Custom => {}
        }
        let view = match profile {
            Profile::Minimal => View::Minimal,
            Profile::Pvp => View::Compact,
            _ => View::Standard,
        };
        for id in ModuleId::ALL {
            *self.view_mut(id) = view;
        }
    }

    pub fn to_json(&self) -> String {
        let json = serde_json::json!({
            "version": self.version,
            "enabled": self.enabled,
            "profile": self.profile.key(),
            "theme": self.theme.key(),
            "opacity": self.opacity,
            "text_shadow": self.text_shadow,
            "editor_hotkey": self.editor_hotkey.key(),
            "settings_hotkey": self.settings_hotkey.key(),
            "snap": self.snap,
            "grid": self.grid,
            "show_help": self.show_help,
            "apple_skin": {
                "view": self.apple_skin.view.key(),
                "module": module_json(&self.apple_skin.module),
                "show_saturation": self.apple_skin.show_saturation,
                "show_exhaustion": self.apple_skin.show_exhaustion,
                "show_numbers": self.apple_skin.show_numbers,
                "show_regen": self.apple_skin.show_regen,
                "show_preview": self.apple_skin.show_preview,
                "show_bars": self.apple_skin.show_bars,
                "low_threshold": self.apple_skin.low_threshold,
                "compact": self.apple_skin.compact,
            },
            "effects": {
                "view": self.effects.view.key(),
                "module": module_json(&self.effects.module),
                "hide_vanilla": self.effects.hide_vanilla,
                "icons": self.effects.icons,
                "names": self.effects.names,
                "amplifier": self.effects.amplifier,
                "time": self.effects.time.key(),
                "sort": self.effects.sort.key(),
                "warn_seconds": self.effects.warn_seconds,
                "max_rows": self.effects.max_rows,
                "group": self.effects.group,
                "ambient": self.effects.ambient,
                "expiring_only": self.effects.expiring_only,
                "horizontal": self.effects.horizontal,
            },
            "inventory": {
                "view": self.inventory.view.key(),
                "module": module_json(&self.inventory.module),
                "columns": self.inventory.columns,
                "hotbar": self.inventory.hotbar,
                "grid": self.inventory.grid,
                "armor": self.inventory.armor,
                "offhand": self.inventory.offhand,
                "empty": self.inventory.empty,
                "counts": self.inventory.counts.key(),
                "durability": self.inventory.durability.key(),
                "highlight": self.inventory.highlight,
                "when": self.inventory.when.key(),
                "creative": self.inventory.creative,
                "slot_size": self.inventory.slot_size,
                "gap": self.inventory.gap,
                "compact_rows": self.inventory.compact_rows,
            },
            "armor": {
                "view": self.armor.view.key(),
                "module": module_json(&self.armor.module),
                "names": self.armor.names,
                "percent": self.armor.percent,
                "bars": self.armor.bars,
                "held": self.armor.held,
                "threshold": self.armor.threshold,
            },
            "coords": {
                "view": self.coords.view.key(),
                "module": module_json(&self.coords.module),
                "chunk": self.coords.chunk,
                "facing": self.coords.facing,
                "dimension": self.coords.dimension,
                "nether": self.coords.nether,
                "precision": self.coords.precision,
                "yaw_degrees": self.coords.yaw_degrees,
            },
            "keys": {
                "view": self.keys.view.key(),
                "module": module_json(&self.keys.module),
                "mouse": self.keys.mouse,
                "space": self.keys.space,
                "sneak_sprint": self.keys.sneak_sprint,
                "cps": self.keys.cps,
                "labels": self.keys.labels,
                "box_size": self.keys.box_size,
            },
            "info": {
                "view": self.info.view.key(),
                "module": module_json(&self.info.module),
                "fps": self.info.fps,
                "frame_ms": self.info.frame_ms,
                "low_fps": self.info.low_fps,
                "time": self.info.time,
                "real_time": self.info.real_time,
                "speed": self.info.speed,
                "compass": self.info.compass,
                "clock24": self.info.clock24,
            },
            "health": {
                "view": self.health.view.key(),
                "module": module_json(&self.health.module),
                "absorption": self.health.absorption,
                "armor_value": self.health.armor_value,
                "food": self.health.food,
                "air": self.health.air,
                "xp": self.health.xp,
                "bars": self.health.bars,
            },
            "held": {
                "view": self.held.view.key(),
                "module": module_json(&self.held.module),
                "offhand": self.held.offhand,
                "arrows": self.held.arrows,
                "totems": self.held.totems,
                "durability": self.held.durability.key(),
                "counts": self.held.counts,
            },
            "tooltips": {
                "food": self.tooltips.food,
                "sneak_details": self.tooltips.sneak_details,
            },
            "vanilla": vanilla_json(&self.vanilla),
        });
        serde_json::to_string_pretty(&json).unwrap_or_default()
    }

    pub fn from_json(text: &str) -> Self {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(text) else {
            return Self::default();
        };
        let mut c = Self::default();
        c.version = CONFIG_VERSION;
        c.enabled = get_bool(&v, "enabled", c.enabled);
        c.profile = Profile::parse(get_str(&v, "profile", c.profile.key()));
        c.theme = Theme::parse(get_str(&v, "theme", c.theme.key()));
        c.opacity = get_num(&v, "opacity", c.opacity as f64).clamp(0.0, 255.0) as u8;
        c.text_shadow = get_bool(&v, "text_shadow", c.text_shadow);
        c.editor_hotkey = Hotkey::parse(get_str(&v, "editor_hotkey", c.editor_hotkey.key()));
        c.settings_hotkey = Hotkey::parse(get_str(&v, "settings_hotkey", c.settings_hotkey.key()));
        c.snap = get_bool(&v, "snap", c.snap);
        c.grid = get_num(&v, "grid", c.grid as f64).clamp(1.0, 64.0) as u8;
        c.show_help = get_bool(&v, "show_help", c.show_help);
        c.apple_skin = apple_skin_from(&v);
        c.effects = effects_from(&v);
        c.inventory = inventory_from(&v);
        c.armor = armor_from(&v);
        c.coords = coords_from(&v);
        c.keys = keys_from(&v);
        c.info = info_from(&v);
        c.health = health_from(&v);
        c.held = held_from(&v);
        c.tooltips = tooltips_from(&v);
        c.vanilla = vanilla_from(&v);
        c
    }

    pub fn load(path: &Path) -> Self {
        std::fs::read_to_string(path)
            .ok()
            .map(|t| Self::from_json(&t))
            .unwrap_or_default()
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let tmp = path.with_extension("json.tmp");
        std::fs::write(&tmp, self.to_json())?;
        std::fs::rename(&tmp, path)
    }
}

fn module_json(m: &ModuleBox) -> serde_json::Value {
    serde_json::json!({
        "enabled": m.enabled,
        "anchor": m.anchor.key(),
        "offset": [m.offset[0], m.offset[1]],
        "scale": m.scale,
        "background": m.background,
        "border": m.border,
    })
}

fn vanilla_json(v: &VanillaHud) -> serde_json::Value {
    let mut elements = serde_json::Map::new();
    for id in VanillaId::ALL {
        let el = v.get(id);
        elements.insert(
            id.key().into(),
            serde_json::json!({
                "x": el.offset[0],
                "y": el.offset[1],
                "scale": el.scale,
                "on": el.on,
            }),
        );
    }
    serde_json::json!({ "enabled": v.enabled, "elements": elements })
}

fn vanilla_from(v: &serde_json::Value) -> VanillaHud {
    let mut out = VanillaHud::default();
    let Some(s) = v.get("vanilla") else {
        return out;
    };
    out.enabled = get_bool(s, "enabled", out.enabled);
    let Some(elements) = s.get("elements").and_then(|x| x.as_object()) else {
        return out;
    };
    for id in VanillaId::ALL {
        let Some(e) = elements.get(id.key()) else {
            continue;
        };
        let el = out.get_mut(id);
        el.offset = [
            get_num(e, "x", 0.0).clamp(-4096.0, 4096.0) as f32,
            get_num(e, "y", 0.0).clamp(-4096.0, 4096.0) as f32,
        ];
        el.scale = get_num(e, "scale", 1.0).clamp(0.25, 4.0) as f32;
        el.on = get_bool(e, "on", true);
    }
    out
}

fn get_bool(v: &serde_json::Value, key: &str, default: bool) -> bool {
    v.get(key).and_then(|x| x.as_bool()).unwrap_or(default)
}
fn get_num(v: &serde_json::Value, key: &str, default: f64) -> f64 {
    v.get(key).and_then(|x| x.as_f64()).unwrap_or(default)
}
fn get_i32(v: &serde_json::Value, key: &str, default: i32) -> i32 {
    get_num(v, key, default as f64).round() as i32
}
fn get_str<'a>(v: &'a serde_json::Value, key: &str, default: &'a str) -> &'a str {
    v.get(key).and_then(|x| x.as_str()).unwrap_or(default)
}

fn module_from(v: &serde_json::Value, default: ModuleBox) -> ModuleBox {
    let Some(m) = v.get("module") else {
        return default;
    };
    ModuleBox {
        enabled: get_bool(m, "enabled", default.enabled),
        anchor: Anchor::parse(get_str(m, "anchor", default.anchor.key())),
        offset: {
            let a = m.get("offset").and_then(|x| x.as_array());
            let x = a
                .and_then(|a| a.first())
                .and_then(|x| x.as_f64())
                .unwrap_or(default.offset[0] as f64) as f32;
            let y = a
                .and_then(|a| a.get(1))
                .and_then(|x| x.as_f64())
                .unwrap_or(default.offset[1] as f64) as f32;
            [x.clamp(-4096.0, 4096.0), y.clamp(-4096.0, 4096.0)]
        },
        scale: get_num(m, "scale", default.scale as f64).clamp(0.5, 2.5) as f32,
        background: get_bool(m, "background", default.background),
        border: get_bool(m, "border", default.border),
    }
}

fn apple_skin_from(v: &serde_json::Value) -> AppleSkin {
    let d = AppleSkin::default();
    let Some(s) = v.get("apple_skin") else {
        return d;
    };
    AppleSkin {
        view: View::parse(get_str(s, "view", d.view.key())),
        module: module_from(s, d.module),
        show_saturation: get_bool(s, "show_saturation", d.show_saturation),
        show_exhaustion: get_bool(s, "show_exhaustion", d.show_exhaustion),
        show_numbers: get_bool(s, "show_numbers", d.show_numbers),
        show_regen: get_bool(s, "show_regen", d.show_regen),
        show_preview: get_bool(s, "show_preview", d.show_preview),
        show_bars: get_bool(s, "show_bars", d.show_bars),
        low_threshold: get_i32(s, "low_threshold", d.low_threshold).clamp(0, 20),
        compact: get_bool(s, "compact", d.compact),
    }
}

fn effects_from(v: &serde_json::Value) -> Effects {
    let d = Effects::default();
    let Some(s) = v.get("effects") else {
        return d;
    };
    Effects {
        view: View::parse(get_str(s, "view", d.view.key())),
        module: module_from(s, d.module),
        hide_vanilla: get_bool(s, "hide_vanilla", d.hide_vanilla),
        icons: get_bool(s, "icons", d.icons),
        names: get_bool(s, "names", d.names),
        amplifier: get_bool(s, "amplifier", d.amplifier),
        time: TimeFormat::parse(get_str(s, "time", d.time.key())),
        sort: EffectSort::parse(get_str(s, "sort", d.sort.key())),
        warn_seconds: get_i32(s, "warn_seconds", d.warn_seconds).clamp(0, 600),
        max_rows: get_i32(s, "max_rows", d.max_rows).clamp(0, 64),
        group: get_bool(s, "group", d.group),
        ambient: get_bool(s, "ambient", d.ambient),
        expiring_only: get_bool(s, "expiring_only", d.expiring_only),
        horizontal: get_bool(s, "horizontal", d.horizontal),
    }
}

fn inventory_from(v: &serde_json::Value) -> Inventory {
    let d = Inventory::default();
    let Some(s) = v.get("inventory") else {
        return d;
    };
    Inventory {
        view: View::parse(get_str(s, "view", d.view.key())),
        module: module_from(s, d.module),
        columns: get_i32(s, "columns", d.columns).clamp(1, 9),
        hotbar: get_bool(s, "hotbar", d.hotbar),
        grid: get_bool(s, "grid", d.grid),
        armor: get_bool(s, "armor", d.armor),
        offhand: get_bool(s, "offhand", d.offhand),
        empty: get_bool(s, "empty", d.empty),
        counts: CountMode::parse(get_str(s, "counts", d.counts.key())),
        durability: ItemDurability::parse(get_str(s, "durability", d.durability.key())),
        highlight: get_bool(s, "highlight", d.highlight),
        when: InventoryWhen::parse(get_str(s, "when", d.when.key())),
        creative: get_bool(s, "creative", d.creative),
        slot_size: get_num(s, "slot_size", d.slot_size as f64).clamp(8.0, 32.0) as f32,
        gap: get_num(s, "gap", d.gap as f64).clamp(0.0, 6.0) as f32,
        compact_rows: get_bool(s, "compact_rows", d.compact_rows),
    }
}

fn armor_from(v: &serde_json::Value) -> Armor {
    let d = Armor::default();
    let Some(s) = v.get("armor") else {
        return d;
    };
    Armor {
        view: View::parse(get_str(s, "view", d.view.key())),
        module: module_from(s, d.module),
        names: get_bool(s, "names", d.names),
        percent: get_bool(s, "percent", d.percent),
        bars: get_bool(s, "bars", d.bars),
        held: get_bool(s, "held", d.held),
        threshold: get_i32(s, "threshold", d.threshold).clamp(0, 100),
    }
}

fn coords_from(v: &serde_json::Value) -> Coords {
    let d = Coords::default();
    let Some(s) = v.get("coords") else {
        return d;
    };
    Coords {
        view: View::parse(get_str(s, "view", d.view.key())),
        module: module_from(s, d.module),
        chunk: get_bool(s, "chunk", d.chunk),
        facing: get_bool(s, "facing", d.facing),
        dimension: get_bool(s, "dimension", d.dimension),
        nether: get_bool(s, "nether", d.nether),
        precision: get_i32(s, "precision", d.precision).clamp(0, 4),
        yaw_degrees: get_bool(s, "yaw_degrees", d.yaw_degrees),
    }
}

fn keys_from(v: &serde_json::Value) -> Keys {
    let d = Keys::default();
    let Some(s) = v.get("keys") else {
        return d;
    };
    Keys {
        view: View::parse(get_str(s, "view", d.view.key())),
        module: module_from(s, d.module),
        mouse: get_bool(s, "mouse", d.mouse),
        space: get_bool(s, "space", d.space),
        sneak_sprint: get_bool(s, "sneak_sprint", d.sneak_sprint),
        cps: get_bool(s, "cps", d.cps),
        labels: get_bool(s, "labels", d.labels),
        box_size: get_num(s, "box_size", d.box_size as f64).clamp(8.0, 32.0) as f32,
    }
}

fn info_from(v: &serde_json::Value) -> Info {
    let d = Info::default();
    let Some(s) = v.get("info") else {
        return d;
    };
    Info {
        view: View::parse(get_str(s, "view", d.view.key())),
        module: module_from(s, d.module),
        fps: get_bool(s, "fps", d.fps),
        frame_ms: get_bool(s, "frame_ms", d.frame_ms),
        low_fps: get_bool(s, "low_fps", d.low_fps),
        time: get_bool(s, "time", d.time),
        real_time: get_bool(s, "real_time", d.real_time),
        speed: get_bool(s, "speed", d.speed),
        compass: get_bool(s, "compass", d.compass),
        clock24: get_bool(s, "clock24", d.clock24),
    }
}

fn health_from(v: &serde_json::Value) -> Health {
    let d = Health::default();
    let Some(s) = v.get("health") else {
        return d;
    };
    Health {
        view: View::parse(get_str(s, "view", d.view.key())),
        module: module_from(s, d.module),
        absorption: get_bool(s, "absorption", d.absorption),
        armor_value: get_bool(s, "armor_value", d.armor_value),
        food: get_bool(s, "food", d.food),
        air: get_bool(s, "air", d.air),
        xp: get_bool(s, "xp", d.xp),
        bars: get_bool(s, "bars", d.bars),
    }
}

fn held_from(v: &serde_json::Value) -> Held {
    let d = Held::default();
    let Some(s) = v.get("held") else {
        return d;
    };
    Held {
        view: View::parse(get_str(s, "view", d.view.key())),
        module: module_from(s, d.module),
        offhand: get_bool(s, "offhand", d.offhand),
        arrows: get_bool(s, "arrows", d.arrows),
        totems: get_bool(s, "totems", d.totems),
        durability: ItemDurability::parse(get_str(s, "durability", d.durability.key())),
        counts: get_bool(s, "counts", d.counts),
    }
}

fn tooltips_from(v: &serde_json::Value) -> Tooltips {
    let d = Tooltips::default();
    let Some(s) = v.get("tooltips") else {
        return d;
    };
    Tooltips {
        food: get_bool(s, "food", d.food),
        sneak_details: get_bool(s, "sneak_details", d.sneak_details),
    }
}

// ---------------------------------------------------------------------------
// What the game hands the renderer, and where it draws
// ---------------------------------------------------------------------------

/// A stack as the HUD needs it: enough to draw its icon, count and wear.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ItemView {
    pub id: String,
    pub name: String,
    pub count: u32,
    pub damage: u32,
    pub max_damage: u32,
}

impl ItemView {
    /// Wear from full to broken: 0 at full, 1 at broken; none for an item
    /// that does not wear.
    pub fn wear(&self) -> Option<f32> {
        (self.max_damage > 0).then(|| {
            let left = self.max_damage.saturating_sub(self.damage) as f32;
            1.0 - (left / self.max_damage as f32)
        })
    }
    pub fn left_percent(&self) -> Option<i32> {
        self.wear().map(|w| ((1.0 - w) * 100.0).round() as i32)
    }
}

/// One status effect as the list shows it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EffectView {
    pub id: String,
    pub name: String,
    pub sprite: String,
    pub amplifier: i32,
    pub duration: i32,
    pub ambient: bool,
    pub beneficial: bool,
}

/// Which vanilla keys and mouse buttons are held, with their labels.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct KeysView {
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub sneak: bool,
    pub sprint: bool,
    pub attack: bool,
    pub use_: bool,
    pub labels: Vec<String>,
    pub mouse: [bool; 3],
}

impl KeysView {
    pub fn label(&self, i: usize) -> &str {
        self.labels.get(i).map_or("", String::as_str)
    }
}

/// Everything a frame needs from the game, gathered once.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Snapshot {
    pub in_world: bool,
    pub screen: &'static str,
    pub position: [f64; 3],
    pub yaw: f32,
    pub pitch: f32,
    pub dimension: String,
    pub health: f32,
    pub max_health: f32,
    pub absorption: f32,
    pub armor_points: i32,
    pub food: i32,
    pub saturation: f32,
    pub exhaustion: f32,
    pub regen_ticks: i32,
    pub air: i32,
    pub max_air: i32,
    pub xp_level: i32,
    pub xp_progress: f32,
    pub game_time: i64,
    pub day_time: i64,
    pub fps: u32,
    pub frame_ms: f32,
    pub low_fps: u32,
    pub speed: f64,
    pub selected: usize,
    /// The player inventory, 0-8 the hotbar.
    pub inventory: Vec<Option<ItemView>>,
    /// Feet, legs, chest, head.
    pub armor_items: Vec<Option<ItemView>>,
    pub offhand: Option<ItemView>,
    pub effects: Vec<EffectView>,
    pub held: Option<ItemView>,
    pub held_food: Option<(u32, f32)>,
    pub held_food_name: String,
    pub arrows: u32,
    pub totems: u32,
    pub survival: bool,
    pub creative: bool,
    pub keys: KeysView,
    pub cps: (u32, u32),
    pub real_time: [u32; 3],
    /// Local real date, for the clock.
    pub real_date: [u32; 3],
}

impl Snapshot {
    pub fn hunger_ratio(&self) -> f32 {
        (self.food as f32 / 20.0).clamp(0.0, 1.0)
    }
    pub fn health_ratio(&self) -> f32 {
        if self.max_health <= 0.0 {
            return 0.0;
        }
        (self.health / self.max_health).clamp(0.0, 1.0)
    }
    pub fn air_ratio(&self) -> f32 {
        if self.max_air <= 0 {
            return 0.0;
        }
        (self.air as f32 / self.max_air as f32).clamp(0.0, 1.0)
    }
    /// The dimension's short name: `Overworld`, `Nether`, `End`, or its path.
    pub fn dimension_name(&self) -> String {
        let id = &self.dimension;
        if id.ends_with("the_nether") {
            "Nether".into()
        } else if id.ends_with("the_end") {
            "The End".into()
        } else if id.ends_with("overworld") {
            "Overworld".into()
        } else {
            id.rsplit(':').next().unwrap_or(id).replace('_', " ")
        }
    }
    pub fn in_nether(&self) -> bool {
        self.dimension.contains("nether")
    }
    /// The same place in the other dimension: the Nether is an eighth.
    pub fn other_dimension(&self) -> ([f64; 3], &'static str) {
        if self.in_nether() {
            (
                [
                    self.position[0] * 8.0,
                    self.position[1],
                    self.position[2] * 8.0,
                ],
                "Overworld",
            )
        } else {
            (
                [
                    self.position[0] / 8.0,
                    self.position[1],
                    self.position[2] / 8.0,
                ],
                "Nether",
            )
        }
    }
    pub fn arrows_in_inventory(&self) -> u32 {
        self.inventory
            .iter()
            .flatten()
            .filter(|i| is_arrow(&i.id))
            .map(|i| i.count)
            .sum()
    }
    pub fn totems_in_inventory(&self) -> u32 {
        self.inventory
            .iter()
            .flatten()
            .filter(|i| i.id.ends_with("totem_of_undying"))
            .map(|i| i.count)
            .sum()
    }
}

pub fn is_arrow(id: &str) -> bool {
    id.ends_with("arrow") && !id.ends_with("arrowhead")
}

// ---------------------------------------------------------------------------
// Painting: the small surface the renderer needs from the game
// ---------------------------------------------------------------------------

pub trait Painter {
    fn screen(&self) -> (f32, f32);
    fn width(&self, text: &str) -> f32;
    fn fill(&mut self, rect: Rect, color: u32);
    fn text(&mut self, text: &str, x: f32, y: f32, color: u32, shadow: bool, scale: f32) -> f32;
    fn sprite(&mut self, name: &str, rect: Rect) -> bool;
    /// Part `src` (`[x, y, w, h]` in the sprite's own pixels) of a sprite,
    /// tinted by `color` (ARGB); a whole sprite without tint is `sprite`.
    fn sprite_region(&mut self, name: &str, src: [f32; 4], rect: Rect, color: u32) -> bool {
        let _ = (src, color);
        self.sprite(name, rect)
    }
    /// A whole sprite tinted by `color` (ARGB).
    fn sprite_tinted(&mut self, name: &str, rect: Rect, color: u32) -> bool {
        let _ = color;
        self.sprite(name, rect)
    }
    fn item(&mut self, item: &ItemView, x: f32, y: f32, size: f32) -> bool;
}

pub const LINE: f32 = 9.0;

fn txt(p: &mut dyn Painter, cfg: &Config, s: &str, x: f32, y: f32, color: u32, scale: f32) -> f32 {
    p.text(s, x, y, color, cfg.text_shadow, scale)
}

fn border(p: &mut dyn Painter, r: Rect, color: u32) {
    p.fill(Rect::new(r.x, r.y, r.w, 1.0), color);
    p.fill(Rect::new(r.x, r.bottom() - 1.0, r.w, 1.0), color);
    p.fill(Rect::new(r.x, r.y, 1.0, r.h), color);
    p.fill(Rect::new(r.right() - 1.0, r.y, 1.0, r.h), color);
}

/// A filled panel with its four corner pixels clipped, which reads as a soft
/// 1 px rounded corner at every GUI scale.
fn rounded_fill(p: &mut dyn Painter, r: Rect, color: u32) {
    if r.w < 3.0 || r.h < 3.0 {
        p.fill(r, color);
        return;
    }
    p.fill(Rect::new(r.x + 1.0, r.y, r.w - 2.0, r.h), color);
    p.fill(Rect::new(r.x, r.y + 1.0, r.w, r.h - 2.0), color);
}

/// The outline of a rounded panel.
fn rounded_border(p: &mut dyn Painter, r: Rect, color: u32) {
    if r.w < 3.0 || r.h < 3.0 {
        border(p, r, color);
        return;
    }
    p.fill(Rect::new(r.x + 1.0, r.y, r.w - 2.0, 1.0), color);
    p.fill(
        Rect::new(r.x + 1.0, r.bottom() - 1.0, r.w - 2.0, 1.0),
        color,
    );
    p.fill(Rect::new(r.x, r.y + 1.0, 1.0, r.h - 2.0), color);
    p.fill(Rect::new(r.right() - 1.0, r.y + 1.0, 1.0, r.h - 2.0), color);
}

fn bar(p: &mut dyn Painter, r: Rect, ratio: f32, fg: u32, bg: u32) {
    if r.w <= 0.0 || r.h <= 0.0 {
        return;
    }
    p.fill(r, bg);
    let w = (r.w * ratio.clamp(0.0, 1.0)).round();
    if w > 0.0 {
        p.fill(Rect::new(r.x, r.y, w, r.h), fg);
    }
}

fn shade(color: u32, factor: f32) -> u32 {
    let [a, r, g, b] = color.to_be_bytes();
    let f = |c: u8| ((c as f32 * factor).clamp(0.0, 255.0)) as u8;
    argb(a, f(r), f(g), f(b))
}

fn panel(p: &mut dyn Painter, cfg: &Config, m: &ModuleBox, r: Rect) -> Colours {
    let col = cfg.theme.colours(cfg.opacity);
    if m.background {
        rounded_fill(p, r, col.panel);
        // A one-pixel highlight along the top edge: enough bevel to separate
        // the panel from the world without a hard frame around every box.
        p.fill(
            Rect::new(r.x + 2.0, r.y + 1.0, (r.w - 4.0).max(1.0), 1.0),
            with_alpha(WHITE, 22),
        );
    }
    if m.border {
        rounded_border(p, r, col.border);
    }
    col
}

fn duration_text(duration: i32, format: TimeFormat) -> String {
    if format == TimeFormat::Off {
        return String::new();
    }
    if duration < 0 {
        return "∞".into();
    }
    let ticks = duration.max(0);
    let secs = ticks / 20;
    match format {
        TimeFormat::Ticks => format!("{ticks}t"),
        TimeFormat::Seconds => format!("{secs}s"),
        TimeFormat::Clock => format!("{}:{:02}", secs / 60, secs % 60),
        TimeFormat::Both => format!("{}:{:02} ({secs}s)", secs / 60, secs % 60),
        TimeFormat::Off => String::new(),
    }
}

fn roman(n: i32) -> String {
    if n <= 0 {
        return String::new();
    }
    let n = n.min(10);
    ["I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X"][(n - 1) as usize].into()
}

fn fmt_coord(v: f64, precision: i32) -> String {
    format!(
        "{v:.precision$}",
        precision = precision.clamp(0, 4) as usize
    )
}

pub fn facing_name(yaw: f32) -> &'static str {
    let i = (yaw / 90.0 + 0.5).floor() as i32 & 3;
    ["South (+Z)", "West (-X)", "North (-Z)", "East (+X)"][i as usize]
}

pub fn facing_short(yaw: f32) -> &'static str {
    let i = (yaw / 90.0 + 0.5).floor() as i32 & 3;
    ["S", "W", "N", "E"][i as usize]
}

/// The in-game clock: `day_time` 0 is 06:00.
pub fn world_clock(day_time: i64) -> (i64, i64, i64) {
    let t = day_time.rem_euclid(24000);
    let day = day_time.div_euclid(24000);
    let hour = (t / 1000 + 6) % 24;
    let minute = (t % 1000) * 60 / 1000;
    (day, hour, minute)
}

pub fn clock_text(day_time: i64) -> String {
    let (day, h, m) = world_clock(day_time);
    format!("Day {day} {:02}:{:02}", h, m)
}

pub fn real_clock(h: u32, m: u32, s: u32, format_24: bool) -> String {
    if format_24 {
        format!("{h:02}:{m:02}:{s:02}")
    } else {
        let (h12, ap) = match h % 12 {
            0 => (12, "AM"),
            v => (v, if h < 12 { "AM" } else { "PM" }),
        };
        format!("{h12}:{m:02} {ap}")
    }
}

/// A readable item name when the language file has none.
pub fn pretty_id(id: &str) -> String {
    let path = id.rsplit(':').next().unwrap_or(id);
    let mut out = String::new();
    for (i, word) in path.split('_').enumerate() {
        if i > 0 {
            out.push(' ');
        }
        let mut chars = word.chars();
        if let Some(c) = chars.next() {
            out.extend(c.to_uppercase());
            out.push_str(chars.as_str());
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Live state: the configuration, the open screen and the editor
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScreenMode {
    #[default]
    Closed,
    Settings,
    Layout,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EditorPage {
    #[default]
    Mods,
    Vanilla,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EditorState {
    pub selected: Option<ModuleId>,
    pub drag: Option<ModuleId>,
    pub selected_vanilla: Option<VanillaId>,
    pub drag_vanilla: Option<VanillaId>,
    pub grab: [f32; 2],
    pub scroll: i32,
    pub page: EditorPage,
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            selected: None,
            drag: None,
            selected_vanilla: None,
            drag_vanilla: None,
            grab: [0.0, 0.0],
            scroll: 0,
            page: EditorPage::Mods,
        }
    }
}

thread_local! {
    static MODE: std::cell::Cell<ScreenMode> = const { std::cell::Cell::new(ScreenMode::Closed) };
    static EDITOR: std::cell::Cell<EditorState> = const { std::cell::Cell::new(EditorState { selected: None, drag: None, selected_vanilla: None, drag_vanilla: None, grab: [0.0; 2], scroll: 0, page: EditorPage::Mods }) };
    static RECTS: std::cell::RefCell<Vec<(ModuleId, Rect)>> = const { std::cell::RefCell::new(Vec::new()) };
    static PENDING: std::cell::Cell<Option<ScreenMode>> = const { std::cell::Cell::new(None) };
}

/// The configuration, read through `load` the first time and kept after.
/// Shared by every thread: a chat command reaches the mod on the server
/// thread while the HUD is drawn on the render thread, and both see the same
/// values.
static CONFIG: std::sync::Mutex<Option<Config>> = std::sync::Mutex::new(None);

pub fn config(load: impl FnOnce() -> Config) -> Config {
    let mut slot = CONFIG.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(cfg) = *slot {
        return cfg;
    }
    let cfg = load();
    *slot = Some(cfg);
    cfg
}

pub fn set_config(cfg: Config) {
    *CONFIG.lock().unwrap_or_else(|e| e.into_inner()) = Some(cfg);
}

/// Changes the configuration and marks the profile custom. False when the
/// change was a no-op.
pub fn edit_config(change: impl FnOnce(&mut Config) -> bool) -> bool {
    let mut slot = CONFIG.lock().unwrap_or_else(|e| e.into_inner());
    let mut cfg = (*slot).unwrap_or_default();
    let mut next = cfg;
    let changed = change(&mut next);
    if changed {
        if next.profile != Profile::Custom {
            next.profile = Profile::Custom;
        }
        cfg = next;
        *slot = Some(cfg);
    }
    changed
}

pub fn mode() -> ScreenMode {
    MODE.with(std::cell::Cell::get)
}

pub fn set_mode(mode: ScreenMode) {
    MODE.with(|m| m.set(mode));
}

pub fn editor() -> EditorState {
    EDITOR.with(std::cell::Cell::get)
}

pub fn edit_editor(change: impl FnOnce(&mut EditorState)) {
    EDITOR.with(|s| {
        let mut e = s.get();
        change(&mut e);
        s.set(e);
    });
}

/// The module rectangles of the last frame, for the editor's hit-testing.
pub fn rects() -> Vec<(ModuleId, Rect)> {
    RECTS.with(|r| r.borrow().clone())
}

pub fn set_rects(r: Vec<(ModuleId, Rect)>) {
    RECTS.with(|v| *v.borrow_mut() = r);
}

pub fn set_pending(mode: ScreenMode) {
    PENDING.with(|p| p.set(Some(mode)));
}

pub fn take_pending() -> Option<ScreenMode> {
    PENDING.with(|p| p.take())
}

// ---------------------------------------------------------------------------
// Layout
// ---------------------------------------------------------------------------

pub const PADDING: f32 = 3.0;

/// Where a module's box goes: its anchor, its offset and its measured size.
pub fn module_rect(cfg: &Config, id: ModuleId, size: (f32, f32), screen: (f32, f32)) -> Rect {
    let m = cfg.module(id);
    let (x, y) = m.anchor.place(size, m.offset, screen);
    // A module dragged past an edge stays on screen: a bad config never
    // hides it for good.
    let max_x = (screen.0 - size.0).max(0.0);
    let max_y = (screen.1 - size.1).max(0.0);
    Rect::new(x.clamp(0.0, max_x), y.clamp(0.0, max_y), size.0, size.1)
}

/// The offset that puts a module's box at `at`, for the editor's dragging.
pub fn offset_for(
    cfg: &Config,
    id: ModuleId,
    size: (f32, f32),
    at: (f32, f32),
    screen: (f32, f32),
) -> [f32; 2] {
    let m = cfg.module(id);
    let (bx, by) = m.anchor.base(size, screen);
    let max_x = (screen.0 - size.0).max(0.0);
    let max_y = (screen.1 - size.1).max(0.0);
    [(at.0.clamp(0.0, max_x)) - bx, (at.1.clamp(0.0, max_y)) - by]
}

/// Snaps an offset around the module's own default, so dragging back to where
/// the module started lands on the exact default offset even when that offset
/// (4, -41, 3, ...) is not a multiple of the grid.
fn snap_offset(grid: u8, base: [f32; 2], offset: [f32; 2]) -> [f32; 2] {
    [
        base[0] + snap_to(grid, offset[0] - base[0]),
        base[1] + snap_to(grid, offset[1] - base[1]),
    ]
}

pub fn snap_to(grid: u8, v: f32) -> f32 {
    let g = f32::from(grid.max(1));
    (v / g).round() * g
}

fn pad(m: &ModuleBox) -> f32 {
    PADDING * m.scale
}

fn line_h(m: &ModuleBox) -> f32 {
    LINE * m.scale + 1.0
}

/// Draws every enabled module and returns their boxes.
pub fn draw_modules(
    p: &mut dyn Painter,
    s: &Snapshot,
    cfg: &Config,
    in_editor: bool,
) -> Vec<(ModuleId, Rect)> {
    let mut out = Vec::new();
    if !cfg.enabled && !in_editor {
        return out;
    }
    let screen = p.screen();
    if in_editor {
        // Dim the world a little: the outlines and drag targets read clearly.
        p.fill(Rect::new(0.0, 0.0, screen.0, screen.1), 0x3300_0000);
    }
    for id in ModuleId::ALL {
        let m = *cfg.module(id);
        if !m.enabled && !in_editor {
            continue;
        }
        if !in_editor && !module_wanted(id, s, cfg) {
            continue;
        }
        let mut size = module_size(id, p, s, cfg);
        if in_editor && (size.0 <= 0.0 || size.1 <= 0.0) {
            size = (96.0, 13.0 + 2.0 * pad(cfg.module(id)));
        }
        if size.0 <= 0.0 || size.1 <= 0.0 {
            continue;
        }
        let mut r = module_rect(cfg, id, size, screen);
        if id == ModuleId::AppleSkin && !in_editor {
            r = apple_skin_follow(cfg, r, screen);
        }
        let col = panel(p, cfg, &m, r);
        draw_body(id, p, s, cfg, r, &col);
        if in_editor && !m.enabled {
            // A module that is off is drawn as a ghost: visible to place,
            // clearly not part of the HUD.
            p.fill(r, 0x9900_0000);
        }
        if in_editor {
            let e = editor();
            let selected = e.selected == Some(id);
            let edge = if !m.enabled {
                with_alpha(col.bad, 200)
            } else if selected {
                col.accent
            } else {
                with_alpha(col.border, 110)
            };
            border(p, r, edge);
            if selected {
                let label = format!(
                    "{}  scale {:.2}  ({:.0}, {:.0})",
                    id.title(),
                    m.scale,
                    r.x,
                    r.y
                );
                // Keep the tag on screen even when the module hugs an edge.
                let w = (p.width(&label) + 6.0).min(screen.0 - 4.0);
                let x = r.x.min(screen.0 - w - 2.0).max(2.0);
                p.fill(
                    Rect::new(x, r.y - 11.0, w, 11.0),
                    with_alpha(col.header, 230),
                );
                txt(p, cfg, &label, x + 3.0, r.y - 9.0, col.text, 1.0);
            }
        }
        out.push((id, r));
    }
    set_rects(out.clone());
    if in_editor {
        draw_editor_footer(p, cfg, &out);
    }
    out
}

/// The inventory module's measured size, for drawing it beside a container.
pub fn inventory_size(p: &dyn Painter, s: &Snapshot, cfg: &Config) -> (f32, f32) {
    size_inventory(p, s, cfg)
}

/// One module's panel and body at `r`, for a module drawn outside the usual
/// layout (the inventory beside an open container).
pub fn draw_module_at(p: &mut dyn Painter, s: &Snapshot, cfg: &Config, id: ModuleId, r: Rect) {
    let m = *cfg.module(id);
    let col = panel(p, cfg, &m, r);
    draw_body(id, p, s, cfg, r, &col);
}

fn module_wanted(id: ModuleId, s: &Snapshot, cfg: &Config) -> bool {
    match id {
        ModuleId::Inventory => {
            if !s.survival && !cfg.inventory.creative {
                return false;
            }
            match cfg.inventory.when {
                InventoryWhen::Always => true,
                InventoryWhen::SurvivalOnly => s.survival,
                InventoryWhen::ContainerOpen => s.screen == "inventory" || s.screen == "creative",
            }
        }
        ModuleId::Effects => !s.effects.is_empty() || cfg.effects.expiring_only,
        // The overlay hangs on the hunger bar, which a non-survival game
        // does not draw.
        ModuleId::AppleSkin => s.survival || cfg.apple_skin.show_bars,
        ModuleId::Info => s.in_world && !info_lines(s, &cfg.info).is_empty(),
        _ => s.in_world,
    }
}

fn module_size(id: ModuleId, p: &dyn Painter, s: &Snapshot, cfg: &Config) -> (f32, f32) {
    match id {
        ModuleId::AppleSkin => size_apple_skin(p, s, cfg),
        ModuleId::Effects => size_effects(p, s, cfg),
        ModuleId::Inventory => size_inventory(p, s, cfg),
        ModuleId::Armor => size_armor(p, s, cfg),
        ModuleId::Coords => size_coords(p, s, cfg),
        ModuleId::Keys => size_keys(p, s, cfg),
        ModuleId::Info => size_info(p, s, cfg),
        ModuleId::Health => size_health(p, s, cfg),
        ModuleId::Held => size_held(p, s, cfg),
    }
}

fn draw_body(
    id: ModuleId,
    p: &mut dyn Painter,
    s: &Snapshot,
    cfg: &Config,
    r: Rect,
    col: &Colours,
) {
    match id {
        ModuleId::AppleSkin => draw_apple_skin(p, s, cfg, r, col),
        ModuleId::Effects => draw_effects(p, s, cfg, r, col),
        ModuleId::Inventory => draw_inventory(p, s, cfg, r, col),
        ModuleId::Armor => draw_armor(p, s, cfg, r, col),
        ModuleId::Coords => draw_coords(p, s, cfg, r, col),
        ModuleId::Keys => draw_keys(p, s, cfg, r, col),
        ModuleId::Info => draw_info(p, s, cfg, r, col),
        ModuleId::Health => draw_health(p, s, cfg, r, col),
        ModuleId::Held => draw_held(p, s, cfg, r, col),
    }
}

// ---------------------------------------------------------------------------
// Coordinates
// ---------------------------------------------------------------------------

pub fn coords_lines(s: &Snapshot, c: &Coords) -> Vec<String> {
    let c = &c.effective();
    let p = c.precision;
    let mut out = vec![format!(
        "XYZ {} {} {}",
        fmt_coord(s.position[0], p),
        fmt_coord(s.position[1], p),
        fmt_coord(s.position[2], p)
    )];
    if c.facing {
        out.push(if c.yaw_degrees {
            format!(
                "Facing {}  ({}°)",
                facing_name(s.yaw),
                s.yaw.rem_euclid(360.0).round() as i32
            )
        } else {
            format!("Facing {}", facing_name(s.yaw))
        });
    }
    if c.chunk {
        out.push(format!(
            "Chunk {} {}",
            (s.position[0].floor() as i64).div_euclid(16),
            (s.position[2].floor() as i64).div_euclid(16)
        ));
    }
    if c.dimension {
        out.push(s.dimension_name());
    }
    if c.nether {
        let (at, name) = s.other_dimension();
        out.push(format!(
            "{name} {} {} {}",
            fmt_coord(at[0], p),
            fmt_coord(at[1], p),
            fmt_coord(at[2], p)
        ));
    }
    out
}

fn size_coords(p: &dyn Painter, s: &Snapshot, cfg: &Config) -> (f32, f32) {
    let c = &cfg.coords.effective();
    let m = &c.module;
    lines_size(p, &coords_lines(s, c), m)
}

fn draw_coords(p: &mut dyn Painter, s: &Snapshot, cfg: &Config, r: Rect, col: &Colours) {
    let c = &cfg.coords.effective();
    let m = &c.module;
    let lines = coords_lines(s, c);
    draw_lines(p, cfg, m, r, &lines, col.text, col.muted);
}

fn lines_size(p: &dyn Painter, lines: &[String], m: &ModuleBox) -> (f32, f32) {
    let w = lines.iter().map(|l| p.width(l)).fold(0.0, f32::max) * m.scale;
    let h = lines.len() as f32 * line_h(m) - 1.0;
    (w + 2.0 * pad(m), h + 2.0 * pad(m))
}

fn draw_lines(
    p: &mut dyn Painter,
    cfg: &Config,
    m: &ModuleBox,
    r: Rect,
    lines: &[String],
    first: u32,
    rest: u32,
) {
    let pad = pad(m);
    let lh = line_h(m);
    for (i, line) in lines.iter().enumerate() {
        let color = if i == 0 { first } else { rest };
        txt(
            p,
            cfg,
            line,
            r.x + pad,
            r.y + pad + lh * i as f32,
            color,
            m.scale,
        );
    }
}

// ---------------------------------------------------------------------------
// Performance and time
// ---------------------------------------------------------------------------

pub fn info_lines(s: &Snapshot, c: &Info) -> Vec<String> {
    let c = &c.effective();
    let mut out = Vec::new();
    if c.fps || c.frame_ms || c.low_fps {
        let mut line = String::new();
        if c.fps {
            line.push_str(&format!("{} fps", s.fps));
        }
        if c.frame_ms {
            if !line.is_empty() {
                line.push_str("  ");
            }
            line.push_str(&format!("{:.1} ms", s.frame_ms));
        }
        if c.low_fps {
            if !line.is_empty() {
                line.push_str("  ");
            }
            line.push_str(&format!("1% low {}", s.low_fps));
        }
        out.push(line);
    }
    if c.time {
        out.push(clock_text(s.day_time));
    }
    if c.real_time {
        let [h, m, sec] = s.real_time;
        out.push(format!(
            "{}  {:04}-{:02}-{:02}",
            real_clock(h, m, sec, c.clock24),
            s.real_date[0],
            s.real_date[1],
            s.real_date[2]
        ));
    }
    if c.speed {
        out.push(format!("Speed {:.2} m/s", s.speed));
    }
    if c.compass {
        out.push(format!(
            "{} {}°",
            facing_short(s.yaw),
            s.yaw.rem_euclid(360.0).round() as i32
        ));
    }
    out
}

fn size_info(p: &dyn Painter, s: &Snapshot, cfg: &Config) -> (f32, f32) {
    let c = &cfg.info.effective();
    let m = &c.module;
    lines_size(p, &info_lines(s, c), m)
}

fn draw_info(p: &mut dyn Painter, s: &Snapshot, cfg: &Config, r: Rect, col: &Colours) {
    let c = &cfg.info.effective();
    let m = &c.module;
    draw_lines(p, cfg, m, r, &info_lines(s, c), col.accent, col.text);
}

// ---------------------------------------------------------------------------
// Player vitals
// ---------------------------------------------------------------------------

pub struct VitalRow {
    pub label: String,
    pub value: String,
    pub ratio: f32,
    pub color: u32,
    pub icon: Option<&'static str>,
}

pub fn health_rows(s: &Snapshot, c: &Health) -> Vec<VitalRow> {
    let c = &c.effective();
    let mut out = Vec::new();
    out.push(VitalRow {
        label: "Health".into(),
        value: format!("{:.1} / {:.0}", s.health, s.max_health),
        ratio: s.health_ratio(),
        color: if s.health_ratio() < 0.3 {
            0xFFFF6B6B
        } else {
            0xFF65E572
        },
        icon: Some("hud/heart/full"),
    });
    if c.absorption && s.absorption > 0.0 {
        out.push(VitalRow {
            label: "Absorption".into(),
            value: format!("{:.1}", s.absorption),
            ratio: (s.absorption / s.max_health.max(1.0)).clamp(0.0, 1.0),
            color: 0xFFFFD75A,
            icon: Some("hud/heart/absorbing_full"),
        });
    }
    if c.armor_value {
        out.push(VitalRow {
            label: "Armour".into(),
            value: format!("{}", s.armor_points),
            ratio: (s.armor_points as f32 / 20.0).clamp(0.0, 1.0),
            color: 0xFFA8B0BC,
            icon: Some("hud/armor_full"),
        });
    }
    if c.food {
        out.push(VitalRow {
            label: "Food".into(),
            value: format!("{} / 20", s.food),
            ratio: s.hunger_ratio(),
            color: 0xFFE8A15A,
            icon: Some("hud/food_full"),
        });
    }
    if c.air && (s.air < s.max_air || s.air <= 0) {
        out.push(VitalRow {
            label: "Air".into(),
            value: format!("{} / {}", s.air.max(0), s.max_air),
            ratio: s.air_ratio(),
            color: 0xFF66C8FF,
            icon: Some("hud/air"),
        });
    }
    if c.xp {
        out.push(VitalRow {
            label: "XP".into(),
            value: format!("{}  {:.0}%", s.xp_level, s.xp_progress * 100.0),
            ratio: s.xp_progress.clamp(0.0, 1.0),
            color: 0xFF9BE86B,
            icon: None,
        });
    }
    out
}

fn size_health(p: &dyn Painter, s: &Snapshot, cfg: &Config) -> (f32, f32) {
    let c = &cfg.health.effective();
    let m = &c.module;
    let rows = health_rows(s, c);
    let icon = 9.0 * m.scale + 2.0;
    let w = rows
        .iter()
        .map(|r| {
            let icon = if r.icon.is_some() { icon } else { 0.0 };
            icon + p.width(&r.label) * m.scale + 6.0 + p.width(&r.value) * m.scale
        })
        .fold(0.0, f32::max);
    let bar = if c.bars { 2.0 * m.scale + 1.0 } else { 0.0 };
    let h = rows.len() as f32 * (line_h(m) + bar) - 1.0;
    (w + 2.0 * pad(m), h + 2.0 * pad(m))
}

fn draw_health(p: &mut dyn Painter, s: &Snapshot, cfg: &Config, r: Rect, col: &Colours) {
    let c = &cfg.health.effective();
    let m = &c.module;
    let pad = pad(m);
    let lh = line_h(m);
    let bar_h = if c.bars { 2.0 * m.scale } else { 0.0 };
    let icon_size = 9.0 * m.scale;
    let mut y = r.y + pad;
    for row in health_rows(s, c) {
        let mut x = r.x + pad;
        if let Some(icon) = row.icon {
            p.sprite(icon, Rect::new(x, y, icon_size, icon_size));
            x += icon_size + 2.0 * m.scale;
        }
        txt(p, cfg, &row.label, x, y, col.muted, m.scale);
        let vw = p.width(&row.value) * m.scale;
        txt(
            p,
            cfg,
            &row.value,
            r.right() - pad - vw,
            y,
            row.color,
            m.scale,
        );
        let lw = r.w - 2.0 * pad;
        if c.bars {
            bar(
                p,
                Rect::new(r.x + pad, y + lh - 1.0, lw, bar_h),
                row.ratio,
                row.color,
                with_alpha(shade(col.slot, 0.55), 180),
            );
        }
        y += lh + bar_h;
    }
}

// ---------------------------------------------------------------------------
// Armour and held items
// ---------------------------------------------------------------------------

pub struct DurabilityRow {
    pub name: String,
    pub detail: String,
    pub color: u32,
    pub item: Option<ItemView>,
}

fn durability_color(percent: Option<i32>, warn: i32) -> u32 {
    match percent {
        Some(p) if p <= warn.max(1) => 0xFFFF6B6B,
        Some(p) if p <= warn.max(1) * 2 => 0xFFFFD75A,
        _ => 0xFFA8B0BC,
    }
}

pub fn armor_rows(s: &Snapshot, c: &Armor) -> Vec<DurabilityRow> {
    let c = &c.effective();
    let names = ["Helmet", "Chestplate", "Leggings", "Boots"];
    let mut out = Vec::new();
    for (i, slot) in s.armor_items.iter().enumerate().take(4) {
        if let Some(item) = slot {
            let percent = item.left_percent();
            let detail = if c.percent {
                percent.map(|p| format!("{p}%")).unwrap_or_default()
            } else {
                String::new()
            };
            out.push(DurabilityRow {
                name: if c.view == View::Minimal {
                    String::new()
                } else if c.names {
                    format!("{}: {}", names[i], item.name)
                } else {
                    item.name.clone()
                },
                detail,
                color: durability_color(percent, c.threshold),
                item: Some(item.clone()),
            });
        }
    }
    if c.held {
        if let Some(item) = &s.held {
            let percent = item.left_percent();
            out.push(DurabilityRow {
                name: item.name.clone(),
                detail: percent.map(|p| format!("{p}%")).unwrap_or_default(),
                color: durability_color(percent, c.threshold),
                item: Some(item.clone()),
            });
        }
        if let Some(item) = &s.offhand {
            let percent = item.left_percent();
            out.push(DurabilityRow {
                name: format!("Off: {}", item.name),
                detail: percent.map(|p| format!("{p}%")).unwrap_or_default(),
                color: durability_color(percent, c.threshold),
                item: Some(item.clone()),
            });
        }
    }
    out
}

fn size_armor(p: &dyn Painter, s: &Snapshot, cfg: &Config) -> (f32, f32) {
    let c = &cfg.armor.effective();
    let m = &c.module;
    let rows = armor_rows(s, c);
    if rows.is_empty() {
        return (0.0, 0.0);
    }
    let icon = 9.0 * m.scale + 2.0;
    let w = rows
        .iter()
        .map(|r| icon + p.width(&r.name) * m.scale + 6.0 + p.width(&r.detail) * m.scale)
        .fold(0.0, f32::max);
    let h = rows.len() as f32 * line_h(m) - 1.0;
    (w + 2.0 * pad(m), h + 2.0 * pad(m))
}

fn draw_armor(p: &mut dyn Painter, s: &Snapshot, cfg: &Config, r: Rect, col: &Colours) {
    let c = &cfg.armor.effective();
    let m = &c.module;
    let pad = pad(m);
    let lh = line_h(m);
    let icon = 9.0 * m.scale;
    for (i, row) in armor_rows(s, c).iter().enumerate() {
        let y = r.y + pad + lh * i as f32;
        let mut x = r.x + pad;
        if let Some(item) = &row.item {
            p.item(item, x, y, icon);
            x += icon + 2.0 * m.scale;
        }
        if !row.name.is_empty() {
            txt(p, cfg, &row.name, x, y, col.text, m.scale);
        }
        let dw = p.width(&row.detail) * m.scale;
        txt(
            p,
            cfg,
            &row.detail,
            r.right() - pad - dw,
            y,
            row.color,
            m.scale,
        );
        if c.bars
            && let Some(item) = &row.item
            && let Some(wear) = item.wear()
        {
            bar(
                p,
                Rect::new(
                    r.x + pad + icon + 2.0,
                    y + lh - 2.0,
                    r.w - 2.0 * pad - icon - 4.0,
                    1.0 * m.scale,
                ),
                1.0 - wear,
                row.color,
                shade(col.panel, 0.5),
            );
        }
    }
}

pub fn held_rows(s: &Snapshot, c: &Held) -> Vec<DurabilityRow> {
    let c = &c.effective();
    let mut out = Vec::new();
    let mut push = |item: &ItemView, prefix: &str| {
        let percent = item.left_percent();
        let mut detail = String::new();
        if c.counts && item.count > 1 {
            detail.push_str(&format!("x{}", item.count));
        }
        match c.durability {
            ItemDurability::Off => {}
            ItemDurability::Bar => {}
            ItemDurability::Percent | ItemDurability::Both => {
                if let Some(p) = percent {
                    if !detail.is_empty() {
                        detail.push(' ');
                    }
                    detail.push_str(&format!("{p}%"));
                }
            }
        }
        out.push(DurabilityRow {
            name: format!("{prefix}{}", item.name),
            detail,
            color: match c.durability {
                ItemDurability::Off | ItemDurability::Bar => 0xFFA8B0BC,
                _ => durability_color(percent, 15),
            },
            item: Some(item.clone()),
        });
    };
    if let Some(item) = &s.held {
        push(item, "");
    }
    if c.offhand
        && let Some(item) = &s.offhand
    {
        push(item, "Off: ");
    }
    if c.arrows && s.arrows > 0 {
        out.push(DurabilityRow {
            name: "Arrows".into(),
            detail: format!("{}", s.arrows),
            color: 0xFFA8B0BC,
            item: None,
        });
    }
    if c.totems && s.totems > 0 {
        out.push(DurabilityRow {
            name: "Totems".into(),
            detail: format!("{}", s.totems),
            color: 0xFF9BE86B,
            item: None,
        });
    }
    out
}

fn size_held(p: &dyn Painter, s: &Snapshot, cfg: &Config) -> (f32, f32) {
    let c = &cfg.held.effective();
    let m = &c.module;
    let rows = held_rows(s, c);
    if rows.is_empty() {
        return (0.0, 0.0);
    }
    let icon = 9.0 * m.scale + 2.0;
    let w = rows
        .iter()
        .map(|r| icon + p.width(&r.name) * m.scale + 6.0 + p.width(&r.detail) * m.scale)
        .fold(0.0, f32::max);
    let h = rows.len() as f32 * line_h(m) - 1.0;
    (w + 2.0 * pad(m), h + 2.0 * pad(m))
}

fn draw_held(p: &mut dyn Painter, s: &Snapshot, cfg: &Config, r: Rect, col: &Colours) {
    let c = &cfg.held.effective();
    let m = &c.module;
    let pad = pad(m);
    let lh = line_h(m);
    let icon = 9.0 * m.scale;
    for (i, row) in held_rows(s, c).iter().enumerate() {
        let y = r.y + pad + lh * i as f32;
        let mut x = r.x + pad;
        if let Some(item) = &row.item {
            p.item(item, x, y, icon);
            x += icon + 2.0 * m.scale;
        }
        txt(p, cfg, &row.name, x, y, col.text, m.scale);
        let dw = p.width(&row.detail) * m.scale;
        txt(
            p,
            cfg,
            &row.detail,
            r.right() - pad - dw,
            y,
            row.color,
            m.scale,
        );
        if matches!(c.durability, ItemDurability::Bar | ItemDurability::Both)
            && let Some(item) = &row.item
            && let Some(percent) = item.left_percent()
        {
            let w = r.w - 2.0 * pad;
            bar(
                p,
                Rect::new(r.x + pad, y + lh - 2.0, w, 1.0 * m.scale),
                percent as f32 / 100.0,
                row.color,
                shade(col.panel, 0.5),
            );
        }
    }
}

// ---------------------------------------------------------------------------
// AppleSkin: hunger, saturation, exhaustion, regeneration and food preview
// ---------------------------------------------------------------------------

pub fn apple_text(s: &Snapshot, c: &AppleSkin) -> Vec<(String, u32)> {
    let c = &c.effective();
    let mut out = Vec::new();
    if !c.show_numbers {
        return out;
    }
    let mut line = format!("{} / 20", s.food);
    if c.show_saturation {
        line.push_str(&format!("   Sat {:.1}", s.saturation));
    }
    if c.show_exhaustion && !c.compact {
        line.push_str(&format!("   Exh {:.1}", s.exhaustion));
    }
    out.push((line, 0));
    if !c.compact && c.show_regen && s.regen_ticks > 0 {
        out.push((format!("Regen in {:.1}s", s.regen_ticks as f32 / 20.0), 0));
    }
    out
}

/// The hunger bar's width: ten 9 px icons eight apart.
pub const HUNGER_BAR: f32 = 81.0;

/// The atlas slot a native mod's own picture is uploaded into.
pub const APPLE_ICONS: &str = "horizon/modimage";

/// The four saturation outlines of AppleSkin's `icons.png` (the mod is public
/// domain), 9x9 each side by side, as RGBA.
pub const APPLE_ICONS_HEX: &str = "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000e9d262ff00000000000000000000000000000000000000000000000000000000e9d262ffe9d262ff00000000000000000000000000000000000000000000000000000000e9d262ffe9d262ff000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ddc241ff0000000000000000000000000000000000000000000000000000000000000000ddc241ff0000000000000000000000000000000000000000ddc241ff0000000000000000ddc241ff000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c2a100ff0000000000000000000000000000000000000000000000000000000000000000c2a100ff000000000000000000000000c2a100ff00000000000000000000000000000000c2a100ff000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c2a100ff0000000000000000000000000000000000000000000000000000000000000000c2a100ff0000000000000000c2a100ff0000000000000000000000000000000000000000c2a100ff0000000000000000000000000000000000000000000000000000000000000000c2a100ff0000000000000000000000000000000000000000000000000000000000000000c2a100ff0000000000000000000000000000000000000000000000000000000000000000c2a100ff0000000000000000000000009f8609ff00000000000000000000000000000000c2a100ff0000000000000000000000000000000000000000000000000000000000000000c2a100ff0000000000000000000000000000000000000000000000000000000000000000c2a100ff000000000000000000000000000000009f8609ff000000000000000000000000c2a100ff000000000000000000000000000000009f8609ff000000000000000000000000c2a100ff000000000000000000000000000000000000000000000000000000009f8609ff00000000c2a100ffc2a100ff000000000000000000000000000000009f8609ff9f8609ff00000000c2a100ffc2a100ff0000000000000000000000009f8609ff9f8609ff9f8609ff00000000c2a100ffc2a100ff0000000000000000000000009f8609ff9f8609ff9f8609ff00000000c2a100ffc2a100ff0000000000000000000000000000000000000000000000009f8609ff00000000c2a100ff0000000000000000000000000000000000000000000000009f8609ff00000000c2a100ff0000000000000000000000000000000000000000000000009f8609ff00000000c2a100ff0000000000000000000000000000000000000000000000009f8609ff00000000c2a100ff0000000000000000000000000000000000000000000000009f8609ff9f8609ff000000000000000000000000000000000000000000000000000000009f8609ff9f8609ff000000000000000000000000000000000000000000000000000000009f8609ff9f8609ff000000000000000000000000000000000000000000000000000000009f8609ff9f8609ff00000000
";

/// The outlines as bytes, decoded once.
pub fn apple_icons_rgba() -> &'static [u8] {
    static BYTES: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
    BYTES.get_or_init(|| {
        // The literal wraps across a source line, so only the hex digits are data.
        let text: Vec<u8> = APPLE_ICONS_HEX
            .bytes()
            .filter(u8::is_ascii_hexdigit)
            .collect();
        let mut out = Vec::with_capacity(text.len() / 2);
        for pair in text.chunks(2) {
            let digit = |b: u8| (b as char).to_digit(16).unwrap_or(0) as u8;
            out.push(digit(pair[0]) << 4 | digit(pair[1]));
        }
        out
    })
}

/// White tinted by `alpha`, the colour a sprite is multiplied by.
fn tint(alpha: f32) -> u32 {
    ((alpha.clamp(0.0, 1.0) * 255.0).round() as u32) << 24 | 0x00FF_FFFF
}

/// AppleSkin's flash: `unclampedFlashAlpha` steps by 1/8 a tick and
/// ping-pongs between -1/2 and 3/2, clamped to 0..1 — eight ticks up, eight
/// held, eight down, eight off.
pub fn flash_alpha(tick: i64) -> f32 {
    let t = tick.rem_euclid(32) as f32;
    if t < 8.0 {
        t / 8.0
    } else if t < 16.0 {
        1.0
    } else if t < 24.0 {
        (24.0 - t) / 8.0
    } else {
        0.0
    }
}

fn hunger_sprite(half: bool, hunger: bool) -> &'static str {
    match (hunger, half) {
        (true, true) => "hud/food_half_hunger",
        (true, false) => "hud/food_full_hunger",
        (false, true) => "hud/food_half",
        (false, false) => "hud/food_full",
    }
}

/// `drawSaturationOverlay`: the gold outlines from icon `from` (0 = the whole
/// bar) up to `level` saturation, over the icons `right` ends at.
#[allow(clippy::too_many_arguments)]
fn saturation_outlines(
    p: &mut dyn Painter,
    level: f32,
    from: f32,
    alpha: f32,
    right: f32,
    y: f32,
    icon: f32,
    step: f32,
) {
    let total = level.clamp(0.0, 20.0);
    if total <= 0.0 || alpha <= 0.0 {
        return;
    }
    let first = if from > 0.0 { (from / 2.0) as i32 } else { 0 };
    let last = (total / 2.0).ceil() as i32;
    for i in first..last {
        let effective = total / 2.0 - i as f32;
        let tile = if effective >= 1.0 {
            3.0
        } else if effective > 0.5 {
            2.0
        } else if effective > 0.25 {
            1.0
        } else {
            0.0
        };
        let at = Rect::new(right - i as f32 * step - icon, y, icon, icon);
        p.sprite_region(APPLE_ICONS, [tile * 9.0, 0.0, 9.0, 9.0], at, tint(alpha));
    }
}

/// The right edge of the hunger icons, which must not move when the numbers
/// make the module's box wider than the bar.
fn apple_skin_icon_right(m: &ModuleBox, bar_scale: f32, pad: f32, screen: (f32, f32)) -> f32 {
    let bar_w = HUNGER_BAR * bar_scale + 2.0 * pad;
    let (x, _) = m.anchor.place((bar_w, 0.0), m.offset, screen);
    x + bar_w - pad
}

fn size_apple_skin(p: &dyn Painter, s: &Snapshot, cfg: &Config) -> (f32, f32) {
    let c = &cfg.apple_skin.effective();
    let m = &c.module;
    let pad = pad(m);
    // The bar part follows the vanilla layer's scale; the numbers and the
    // padding are the module's own.
    let bar_scale = m.scale * apple_skin_scale(cfg);
    let text = apple_text(s, c);
    let text_w = text
        .iter()
        .map(|(t, _)| p.width(t) * m.scale)
        .fold(0.0, f32::max);
    let text_h = text.len() as f32 * line_h(m);
    let h = 9.0 * bar_scale + text_h + 2.0 * pad + if text.is_empty() { 0.0 } else { 2.0 };
    ((HUNGER_BAR * bar_scale).max(text_w) + 2.0 * pad, h)
}

fn draw_apple_skin(p: &mut dyn Painter, s: &Snapshot, cfg: &Config, r: Rect, col: &Colours) {
    let c = &cfg.apple_skin.effective();
    let m = &c.module;
    let pad = pad(m);
    let bar_scale = m.scale * apple_skin_scale(cfg);
    let icon = 9.0 * bar_scale;
    let step = 8.0 * bar_scale;
    // The icons sit on the box's bottom line, so the numbers can grow above
    // them without moving them off the vanilla bar.
    let pinned = cfg.vanilla.enabled
        && cfg.vanilla.get(VanillaId::PlayerHealth).on
        && (cfg.vanilla.get(VanillaId::PlayerHealth).offset != [0.0, 0.0]
            || (cfg.vanilla.get(VanillaId::PlayerHealth).scale - 1.0).abs() >= 0.001);
    let right = if pinned {
        r.right() - pad
    } else {
        apple_skin_icon_right(m, bar_scale, pad, p.screen())
    };
    let y = r.bottom() - pad - icon;
    let hunger = s.effects.iter().any(|e| e.id == "hunger");
    // AppleSkin draws rotten flesh's preview with the hunger textures.
    let rotten = s
        .held
        .as_ref()
        .is_some_and(|i| i.id.ends_with("rotten_flesh"));
    let preview_hunger = hunger || rotten;
    let at = |i: i32| Rect::new(right - i as f32 * step - icon, y, icon, icon);

    if c.show_bars {
        for i in 0..10 {
            let value = s.food - i * 2;
            let name = if value >= 2 {
                hunger_sprite(false, hunger)
            } else if value == 1 {
                hunger_sprite(true, hunger)
            } else if hunger {
                "hud/food_empty_hunger"
            } else {
                "hud/food_empty"
            };
            p.sprite(name, at(i));
        }
    }

    let flash = flash_alpha(s.game_time);
    if c.show_saturation {
        saturation_outlines(p, s.saturation, 0.0, 1.0, right, y, icon, step);
    }
    if c.show_preview
        && let Some((food, saturation)) = s.held_food
    {
        let new_food = (s.food + food as i32).min(20);
        let new_saturation = (s.saturation + saturation).min(new_food as f32);
        if c.show_saturation && new_saturation > s.saturation {
            saturation_outlines(p, new_saturation, s.saturation, flash, right, y, icon, step);
        }
        // `drawHungerOverlay`: a faint empty icon under the flash, then the
        // full or half icon the food would add.
        let first = (s.food / 2).max(0);
        let last = (new_food + 1) / 2;
        for i in first..last {
            let empty = if preview_hunger {
                "hud/food_empty_hunger"
            } else {
                "hud/food_empty"
            };
            p.sprite_tinted(empty, at(i), tint(flash * 0.25));
            let half = i * 2 + 1 == new_food;
            p.sprite_tinted(hunger_sprite(half, preview_hunger), at(i), tint(flash));
        }
    }

    let lines = apple_text(s, c);
    if !lines.is_empty() {
        let lh = line_h(m);
        let low = s.food <= c.low_threshold;
        // Centre the numbers over the icons: the box grows to fit a long
        // line, and left-aligning would push the text off the bar's centre.
        let inner = r.w - 2.0 * pad;
        let mut ty = y - 2.0 - lh * lines.len() as f32;
        for (line, _) in &lines {
            let x = r.x + pad + (inner - p.width(line) * m.scale).max(0.0) / 2.0;
            txt(
                p,
                cfg,
                line,
                x,
                ty,
                if low { col.bad } else { col.text },
                m.scale,
            );
            ty += lh;
        }
    }
}

// ---------------------------------------------------------------------------
// Keystrokes
// ---------------------------------------------------------------------------

fn key_geom(m: &ModuleBox, c: &Keys) -> (f32, f32) {
    (c.box_size * m.scale, 2.0 * m.scale)
}

fn key_rows(c: &Keys) -> usize {
    2 + usize::from(c.space) + usize::from(c.sneak_sprint) + usize::from(c.mouse)
}

fn size_keys(p: &dyn Painter, s: &Snapshot, cfg: &Config) -> (f32, f32) {
    let c = &cfg.keys.effective();
    let m = &c.module;
    let (k, g) = key_geom(m, c);
    let pad = pad(m);
    let rows = key_rows(c);
    let full = 3.0 * k + 2.0 * g;
    let mut w = full;
    let mut h = rows as f32 * k + (rows as f32 - 1.0) * g;
    if c.cps {
        let cps = format!("{} | {} cps", s.cps.0, s.cps.1);
        if c.mouse {
            w += 4.0 * m.scale + p.width(&cps) * m.scale;
        } else {
            w = w.max(p.width(&cps) * m.scale);
            h += g + LINE * m.scale;
        }
    }
    (w + 2.0 * pad, h + 2.0 * pad)
}

fn key_box(
    p: &mut dyn Painter,
    cfg: &Config,
    col: &Colours,
    r: Rect,
    lit: bool,
    label: &str,
    c: &Keys,
) {
    // A lit key is a glow, not a block of paint: a translucent accent fill
    // leaves the world visible and the white label readable.
    rounded_fill(
        p,
        r,
        if lit {
            with_alpha(col.accent, 150)
        } else {
            col.slot
        },
    );
    rounded_border(
        p,
        r,
        if lit {
            col.accent
        } else {
            shade(col.slot, 1.35)
        },
    );
    if c.labels && !label.is_empty() {
        // A localized key name can be longer than its box ("Left Ctrl" in a
        // half-row key): shrink it to fit, and only then shorten it, so no two
        // labels ever overlap.
        let max = (r.w - 4.0).max(1.0);
        let mut scale = c.module.scale;
        let mut text = label.to_owned();
        let mut tw = p.width(&text) * scale;
        if tw > max {
            scale = (scale * max / tw).max(0.4);
            tw = p.width(&text) * scale;
        }
        while tw > max && text.chars().count() > 1 {
            text.pop();
            tw = p.width(&text) * scale;
        }
        let th = LINE * scale;
        txt(
            p,
            cfg,
            &text,
            r.x + (r.w - tw) / 2.0,
            r.y + (r.h - th) / 2.0,
            col.text,
            scale,
        );
    }
}

fn draw_keys(p: &mut dyn Painter, s: &Snapshot, cfg: &Config, r: Rect, col: &Colours) {
    let c = &cfg.keys.effective();
    let m = &c.module;
    let (k, g) = key_geom(m, c);
    let pad = pad(m);
    let x = r.x + pad;
    let mut y = r.y + pad;
    let keys = &s.keys;
    let labels = |i: usize, fallback: &str| -> String {
        let l = keys.label(i);
        if l.is_empty() {
            fallback.into()
        } else {
            l.into()
        }
    };
    let full = 3.0 * k + 2.0 * g;
    // W above S.
    key_box(
        p,
        cfg,
        col,
        Rect::new(x + k + g, y, k, k),
        keys.forward,
        &labels(0, "W"),
        c,
    );
    y += k + g;
    key_box(
        p,
        cfg,
        col,
        Rect::new(x, y, k, k),
        keys.left,
        &labels(2, "A"),
        c,
    );
    key_box(
        p,
        cfg,
        col,
        Rect::new(x + k + g, y, k, k),
        keys.back,
        &labels(1, "S"),
        c,
    );
    key_box(
        p,
        cfg,
        col,
        Rect::new(x + 2.0 * (k + g), y, k, k),
        keys.right,
        &labels(3, "D"),
        c,
    );
    y += k + g;
    if c.space {
        key_box(
            p,
            cfg,
            col,
            Rect::new(x, y, full, k),
            keys.jump,
            &labels(4, "Space"),
            c,
        );
        y += k + g;
    }
    if c.sneak_sprint {
        let half = (full - g) / 2.0;
        key_box(
            p,
            cfg,
            col,
            Rect::new(x, y, half, k),
            keys.sneak,
            &labels(5, "Shift"),
            c,
        );
        key_box(
            p,
            cfg,
            col,
            Rect::new(x + half + g, y, half, k),
            keys.sprint,
            &labels(6, "Ctrl"),
            c,
        );
        y += k + g;
    }
    if c.mouse {
        let half = (full - g) / 2.0;
        key_box(
            p,
            cfg,
            col,
            Rect::new(x, y, half, k),
            keys.mouse[0],
            "LMB",
            c,
        );
        key_box(
            p,
            cfg,
            col,
            Rect::new(x + half + g, y, half, k),
            keys.mouse[2],
            "RMB",
            c,
        );
        if c.cps {
            let cps = format!("{} | {} cps", s.cps.0, s.cps.1);
            let tw = p.width(&cps) * m.scale;
            txt(
                p,
                cfg,
                &cps,
                r.right() - pad - tw,
                y + (k - LINE * m.scale) / 2.0,
                col.text,
                m.scale,
            );
        }
        y += k + g;
    }
    if c.cps && !c.mouse {
        let cps = format!("{} | {} cps", s.cps.0, s.cps.1);
        txt(p, cfg, &cps, x, y + g * 0.5, col.text, m.scale);
    }
}

// ---------------------------------------------------------------------------
// Status effects
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct EffectCell {
    pub name: String,
    pub amplifier: String,
    pub time: String,
    pub color: u32,
    pub sprite: String,
    pub ambient: bool,
    pub beneficial: bool,
}

pub fn effect_cells(s: &Snapshot, c: &Effects) -> Vec<EffectCell> {
    let c = &c.effective();
    let mut list: Vec<EffectView> = s
        .effects
        .iter()
        .filter(|e| {
            !c.expiring_only || (e.duration >= 0 && e.duration <= c.warn_seconds.max(1) * 20)
        })
        .cloned()
        .collect();
    list.sort_by(|a, b| {
        if c.group && a.beneficial != b.beneficial {
            return b.beneficial.cmp(&a.beneficial);
        }
        match c.sort {
            EffectSort::Duration => {
                let ia = a.duration < 0;
                let ib = b.duration < 0;
                ia.cmp(&ib)
                    .then(a.duration.cmp(&b.duration))
                    .then_with(|| a.name.cmp(&b.name))
            }
            EffectSort::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            EffectSort::Amplifier => b
                .amplifier
                .cmp(&a.amplifier)
                .then_with(|| a.name.cmp(&b.name)),
        }
    });
    list.into_iter()
        .map(|e| {
            let warn = e.duration >= 0 && e.duration <= c.warn_seconds.max(1) * 20;
            EffectCell {
                name: e.name,
                amplifier: roman(e.amplifier + 1),
                time: duration_text(e.duration, c.time),
                color: if warn {
                    0xFFFF6B6B
                } else if e.beneficial {
                    0xFF9BE86B
                } else {
                    0xFFA8B0BC
                },
                sprite: e.sprite,
                ambient: e.ambient,
                beneficial: e.beneficial,
            }
        })
        .collect()
}

fn effects_grid(n: usize, c: &Effects) -> (usize, usize) {
    if n == 0 {
        return (0, 0);
    }
    let cap = if c.max_rows <= 0 {
        n
    } else {
        (c.max_rows as usize).min(n)
    };
    if c.horizontal {
        let per_col = cap.max(1);
        let cols = n.div_ceil(per_col);
        (cols, per_col)
    } else {
        (1, cap)
    }
}

fn effect_icon(m: &ModuleBox) -> f32 {
    9.0 * m.scale
}

/// How many effects the grid shows: the vertical layout stops at the row
/// limit, the horizontal one grows sideways instead.
fn effects_shown(n: usize, c: &Effects) -> usize {
    if c.horizontal || c.max_rows <= 0 {
        n
    } else {
        n.min(c.max_rows as usize)
    }
}

fn effects_cell_width(p: &dyn Painter, cells: &[EffectCell], c: &Effects, icon: f32) -> f32 {
    cells
        .iter()
        .map(|cell| {
            let icon = if c.icons {
                icon + 2.0 * c.module.scale
            } else {
                0.0
            };
            icon + if c.names {
                p.width(&cell.name) * c.module.scale
            } else {
                0.0
            } + if c.amplifier && !cell.amplifier.is_empty() {
                (p.width(&cell.amplifier) + 2.0) * c.module.scale
            } else {
                0.0
            } + 6.0 * c.module.scale
                + p.width(&cell.time) * c.module.scale
        })
        .fold(0.0, f32::max)
}

fn size_effects(p: &dyn Painter, s: &Snapshot, cfg: &Config) -> (f32, f32) {
    let c = &cfg.effects.effective();
    let m = &c.module;
    let cells = effect_cells(s, c);
    let shown = effects_shown(cells.len(), c);
    if shown == 0 {
        return (0.0, 0.0);
    }
    let cells = &cells[..shown];
    let pad = pad(m);
    let icon = effect_icon(m);
    let gap = 3.0 * m.scale;
    let row_h = (icon + 1.0 * m.scale).max(line_h(m));
    let cell_w = effects_cell_width(p, cells, c, icon);
    let (cols, rows) = effects_grid(shown, c);
    let w = cols as f32 * (cell_w + gap) - gap;
    let mut h = rows as f32 * (row_h + gap) - gap;
    if shown < cells.len() {
        h += line_h(m);
    }
    (w + 2.0 * pad, h + 2.0 * pad)
}

fn draw_effects(p: &mut dyn Painter, s: &Snapshot, cfg: &Config, r: Rect, col: &Colours) {
    let c = &cfg.effects.effective();
    let m = &c.module;
    let all = effect_cells(s, c);
    let shown = effects_shown(all.len(), c);
    if shown == 0 {
        return;
    }
    let cells = &all[..shown];
    let pad = pad(m);
    let icon = effect_icon(m);
    let gap = 3.0 * m.scale;
    let row_h = (icon + 1.0 * m.scale).max(line_h(m));
    let (_cols, rows) = effects_grid(shown, c);
    let cell_w = effects_cell_width(p, cells, c, icon);
    for (i, cell) in cells.iter().enumerate() {
        let col_i = if c.horizontal { i / rows.max(1) } else { 0 };
        let row_i = if c.horizontal { i % rows.max(1) } else { i };
        let mut x = r.x + pad + col_i as f32 * (cell_w + gap);
        let y = r.y + pad + row_i as f32 * (row_h + gap);
        if c.icons {
            if !p.sprite(&cell.sprite, Rect::new(x, y, icon, icon)) {
                p.fill(Rect::new(x, y, icon, icon), with_alpha(col.accent, 90));
            }
            x += icon + 2.0 * m.scale;
        }
        let name_color = if cell.ambient && c.ambient {
            with_alpha(cell.color, 170)
        } else {
            cell.color
        };
        if c.names {
            txt(
                p,
                cfg,
                &cell.name,
                x,
                y + (icon - LINE * m.scale) / 2.0,
                name_color,
                m.scale,
            );
            x += p.width(&cell.name) * m.scale;
        }
        if c.amplifier && !cell.amplifier.is_empty() {
            x += 2.0 * m.scale;
            txt(
                p,
                cfg,
                &cell.amplifier,
                x,
                y + (icon - LINE * m.scale) / 2.0,
                col.text,
                m.scale,
            );
            x += p.width(&cell.amplifier) * m.scale;
        }
        let tw = p.width(&cell.time) * m.scale;
        txt(
            p,
            cfg,
            &cell.time,
            r.x + pad + col_i as f32 * (cell_w + gap) + cell_w - tw,
            y + (icon - LINE * m.scale) / 2.0,
            cell.color,
            m.scale,
        );
        let _ = x;
    }
    let extra = all.len() - shown;
    if extra > 0 {
        let y = r.y + pad + rows as f32 * (row_h + gap);
        txt(
            p,
            cfg,
            &format!("+{extra} more"),
            r.x + pad,
            y,
            col.muted,
            m.scale,
        );
    }
}

// ---------------------------------------------------------------------------
// Inventory
// ---------------------------------------------------------------------------

pub struct SlotCell {
    pub item: Option<ItemView>,
    pub selected: bool,
}

pub fn inventory_cells(s: &Snapshot, c: &Inventory) -> Vec<SlotCell> {
    let c = &c.effective();
    let mut out = Vec::new();
    let n = s.inventory.len().min(36);
    for i in 0..n {
        let hotbar = i < 9;
        if hotbar && !c.hotbar {
            continue;
        }
        if !hotbar && !c.grid {
            continue;
        }
        out.push(SlotCell {
            item: s.inventory.get(i).and_then(|i| i.clone()),
            selected: i == s.selected,
        });
    }
    out
}

/// The grid cells the module actually draws. With "empty slots" off only the
/// occupied slots are shown, packed into rows, so an always-on inventory HUD
/// stays a compact list of what the player carries rather than a full grid of
/// holes.
fn shown_inventory_cells(s: &Snapshot, c: &Inventory) -> Vec<SlotCell> {
    let c = &c.effective();
    let mut cells = inventory_cells(s, c);
    if !c.empty {
        cells.retain(|cell| cell.item.is_some());
    }
    cells
}

pub fn armor_cells(s: &Snapshot, c: &Inventory) -> Vec<Option<ItemView>> {
    let c = &c.effective();
    let mut out = Vec::new();
    if !c.armor {
        return out;
    }
    for item in s.armor_items.iter().take(4) {
        out.push(item.clone());
    }
    if c.offhand {
        out.push(s.offhand.clone());
    }
    out
}

fn inv_geom(m: &ModuleBox, c: &Inventory) -> (f32, f32) {
    (c.slot_size * m.scale, 1.0 * m.scale)
}

fn inventory_size_parts(s: &Snapshot, c: &Inventory, p: &dyn Painter) -> (f32, f32, usize) {
    let c = &c.effective();
    let cells = shown_inventory_cells(s, c);
    let cols = c.columns.clamp(1, 9) as usize;
    let rows = if cells.is_empty() {
        0
    } else {
        cells.len().div_ceil(cols).max(1)
    };
    let m = &c.module;
    let (slot, gap) = inv_geom(m, c);
    let gap = if c.compact_rows { gap * 0.5 } else { gap };
    let pad = pad(m);
    // A one-row inventory is one slot wide, not nine: the panel hugs what it
    // holds instead of leaving a long empty bar.
    let used_cols = cols.min(cells.len().max(1));
    let grid_w = if rows == 0 {
        0.0
    } else {
        used_cols as f32 * (slot + gap) - gap
    };
    let grid_h = if rows == 0 {
        0.0
    } else {
        rows as f32 * (slot + gap) - gap
    };
    let armor = armor_cells(s, c);
    let armor_w = if armor.is_empty() {
        0.0
    } else {
        2.0 * gap + slot
    };
    let armor_h = if armor.is_empty() {
        0.0
    } else {
        armor.len() as f32 * (slot + gap) - gap
    };
    let _ = p;
    (
        grid_w + armor_w + 2.0 * pad,
        grid_h.max(armor_h) + 2.0 * pad,
        rows,
    )
}

fn size_inventory(p: &dyn Painter, s: &Snapshot, cfg: &Config) -> (f32, f32) {
    let (w, h, _) = inventory_size_parts(s, &cfg.inventory, p);
    (w, h)
}

fn draw_inventory(p: &mut dyn Painter, s: &Snapshot, cfg: &Config, r: Rect, col: &Colours) {
    let c = &cfg.inventory.effective();
    let m = &c.module;
    let pad = pad(m);
    let (slot, gap) = inv_geom(m, c);
    let gap = if c.compact_rows { gap * 0.5 } else { gap };
    let cols = c.columns.clamp(1, 9) as usize;
    let cells = shown_inventory_cells(s, c);
    let armor = armor_cells(s, c);
    let armor_w = if armor.is_empty() {
        0.0
    } else {
        2.0 * gap + slot
    };
    let mut x0 = r.x + pad + armor_w;
    let y0 = r.y + pad;
    // Armour and offhand, left of the grid.
    if !armor.is_empty() {
        for (i, item) in armor.iter().enumerate() {
            let y = y0 + i as f32 * (slot + gap);
            slot_cell(
                p,
                cfg,
                col,
                Rect::new(r.x + pad, y, slot, slot),
                item.as_ref(),
                false,
                c,
            );
        }
        x0 += 0.0;
    }
    for (i, cell) in cells.iter().enumerate() {
        let cx = x0 + (i % cols) as f32 * (slot + gap);
        let cy = y0 + (i / cols) as f32 * (slot + gap);
        let cold = if c.highlight && cell.selected {
            with_alpha(col.accent, 80)
        } else {
            col.slot
        };
        slot_cell(
            p,
            cfg,
            col,
            Rect::new(cx, cy, slot, slot),
            cell.item.as_ref(),
            cell.selected && c.highlight,
            c,
        );
        let _ = cold;
    }
}

fn slot_cell(
    p: &mut dyn Painter,
    cfg: &Config,
    col: &Colours,
    r: Rect,
    item: Option<&ItemView>,
    selected: bool,
    c: &Inventory,
) {
    if item.is_some() || c.empty {
        rounded_fill(p, r, col.slot);
    }
    if selected {
        rounded_border(p, r, col.accent);
    }
    let Some(item) = item else {
        return;
    };
    let inset = (2.0 * c.module.scale).min(r.w / 4.0);
    p.item(item, r.x + inset, r.y + inset, r.w - 2.0 * inset);
    if let Some(percent) = item.left_percent() {
        let ratio = percent as f32 / 100.0;
        let color = durability_color(Some(percent), 15);
        if matches!(c.durability, ItemDurability::Bar | ItemDurability::Both) {
            bar(
                p,
                Rect::new(r.x + 1.0, r.bottom() - 3.0, r.w - 2.0, 2.0 * c.module.scale),
                ratio,
                color,
                0xFF20_2020,
            );
        }
        if matches!(c.durability, ItemDurability::Percent | ItemDurability::Both) {
            let text = format!("{percent}");
            let scale = 0.5 * c.module.scale;
            txt(
                p,
                cfg,
                &text,
                r.x + 1.0,
                r.bottom() - LINE * scale - 1.0,
                0xFFFF_FFFF,
                scale,
            );
        }
    }
    let show = match c.counts {
        CountMode::Off => false,
        CountMode::Stack => item.count > 1,
        CountMode::Always => item.count > 0,
    };
    if show {
        let text = item.count.to_string();
        let scale = 0.75 * c.module.scale;
        let tw = p.width(&text) * scale;
        txt(
            p,
            cfg,
            &text,
            r.right() - tw - 1.0,
            r.bottom() - LINE * scale - 1.0,
            0xFFFF_FFFF,
            scale,
        );
    }
}

// ---------------------------------------------------------------------------
// The layout editor's footer
// ---------------------------------------------------------------------------

fn draw_editor_footer(p: &mut dyn Painter, cfg: &Config, rects: &[(ModuleId, Rect)]) {
    let col = cfg.theme.colours(cfg.opacity);
    let (sw, sh) = p.screen();
    let _ = rects;
    // The hotbar sits in the middle of the bottom edge, so the keys live in
    // the corner beside it: the live HUD stays visible while it is placed.
    let hint = "Drag  ·  Wheel scales  ·  RMB on/off";
    let hint2 = "Arrows nudge  ·  Space  ·  R reset  ·  Esc";
    let mut w = p.width(hint) * 0.78;
    if cfg.show_help {
        w = w.max(p.width(hint2) * 0.78);
    }
    let w = (w + 10.0).clamp(140.0, (sw * 0.34).max(140.0));
    let h = if cfg.show_help { 24.0 } else { 13.0 };
    let r = Rect::new(4.0, sh - h - 4.0, w, h);
    rounded_fill(p, r, with_alpha(col.header, 220));
    rounded_border(p, r, with_alpha(col.border, 150));
    txt(p, cfg, hint, r.x + 5.0, r.y + 2.0, col.muted, 0.78);
    if cfg.show_help {
        txt(p, cfg, hint2, r.x + 5.0, r.y + 13.0, col.muted, 0.78);
    }
}

// ---------------------------------------------------------------------------
// Settings: the model behind the settings screen and the Options > Mods page
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModField {
    Enabled,
    Anchor,
    Scale,
    Background,
    Border,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flag {
    Enabled,
    AppleSaturation,
    AppleExhaustion,
    AppleNumbers,
    AppleRegen,
    ApplePreview,
    AppleBars,
    AppleCompact,
    EffectsHideVanilla,
    EffectsIcons,
    EffectsNames,
    EffectsAmplifier,
    EffectsGroup,
    EffectsAmbient,
    EffectsExpiring,
    EffectsHorizontal,
    InvHotbar,
    InvArmor,
    InvOffhand,
    InvEmpty,
    InvHighlight,
    InvCreative,
    InvCompact,
    ArmorNames,
    ArmorPercent,
    ArmorBars,
    ArmorHeld,
    CoordsChunk,
    CoordsFacing,
    CoordsDimension,
    CoordsNether,
    CoordsYaw,
    KeysMouse,
    KeysSpace,
    KeysSneakSprint,
    KeysCps,
    KeysLabels,
    InfoFps,
    InfoFrameMs,
    InfoLowFps,
    InfoTime,
    InfoRealTime,
    InfoSpeed,
    InfoCompass,
    InfoClock24,
    HealthAbsorption,
    HealthArmor,
    HealthFood,
    HealthAir,
    HealthXp,
    HealthBars,
    HeldOffhand,
    HeldArrows,
    HeldTotems,
    HeldCounts,
    TooltipFood,
    TooltipSneak,
    TextShadow,
    Snap,
    ShowHelp,
    VanillaHud,
}

impl Flag {
    pub fn get(self, c: &Config) -> bool {
        use Flag::*;
        match self {
            Enabled => c.enabled,
            AppleSaturation => c.apple_skin.show_saturation,
            AppleExhaustion => c.apple_skin.show_exhaustion,
            AppleNumbers => c.apple_skin.show_numbers,
            AppleRegen => c.apple_skin.show_regen,
            ApplePreview => c.apple_skin.show_preview,
            AppleBars => c.apple_skin.show_bars,
            AppleCompact => c.apple_skin.compact,
            EffectsHideVanilla => c.effects.hide_vanilla,
            EffectsIcons => c.effects.icons,
            EffectsNames => c.effects.names,
            EffectsAmplifier => c.effects.amplifier,
            EffectsGroup => c.effects.group,
            EffectsAmbient => c.effects.ambient,
            EffectsExpiring => c.effects.expiring_only,
            EffectsHorizontal => c.effects.horizontal,
            InvHotbar => c.inventory.hotbar,
            InvArmor => c.inventory.armor,
            InvOffhand => c.inventory.offhand,
            InvEmpty => c.inventory.empty,
            InvHighlight => c.inventory.highlight,
            InvCreative => c.inventory.creative,
            InvCompact => c.inventory.compact_rows,
            ArmorNames => c.armor.names,
            ArmorPercent => c.armor.percent,
            ArmorBars => c.armor.bars,
            ArmorHeld => c.armor.held,
            CoordsChunk => c.coords.chunk,
            CoordsFacing => c.coords.facing,
            CoordsDimension => c.coords.dimension,
            CoordsNether => c.coords.nether,
            CoordsYaw => c.coords.yaw_degrees,
            KeysMouse => c.keys.mouse,
            KeysSpace => c.keys.space,
            KeysSneakSprint => c.keys.sneak_sprint,
            KeysCps => c.keys.cps,
            KeysLabels => c.keys.labels,
            InfoFps => c.info.fps,
            InfoFrameMs => c.info.frame_ms,
            InfoLowFps => c.info.low_fps,
            InfoTime => c.info.time,
            InfoRealTime => c.info.real_time,
            InfoSpeed => c.info.speed,
            InfoCompass => c.info.compass,
            InfoClock24 => c.info.clock24,
            HealthAbsorption => c.health.absorption,
            HealthArmor => c.health.armor_value,
            HealthFood => c.health.food,
            HealthAir => c.health.air,
            HealthXp => c.health.xp,
            HealthBars => c.health.bars,
            HeldOffhand => c.held.offhand,
            HeldArrows => c.held.arrows,
            HeldTotems => c.held.totems,
            HeldCounts => c.held.counts,
            TooltipFood => c.tooltips.food,
            TooltipSneak => c.tooltips.sneak_details,
            TextShadow => c.text_shadow,
            Snap => c.snap,
            ShowHelp => c.show_help,
            VanillaHud => c.vanilla.enabled,
        }
    }
    pub fn set(self, c: &mut Config, v: bool) {
        use Flag::*;
        match self {
            Enabled => c.enabled = v,
            AppleSaturation => c.apple_skin.show_saturation = v,
            AppleExhaustion => c.apple_skin.show_exhaustion = v,
            AppleNumbers => c.apple_skin.show_numbers = v,
            AppleRegen => c.apple_skin.show_regen = v,
            ApplePreview => c.apple_skin.show_preview = v,
            AppleBars => c.apple_skin.show_bars = v,
            AppleCompact => c.apple_skin.compact = v,
            EffectsHideVanilla => c.effects.hide_vanilla = v,
            EffectsIcons => c.effects.icons = v,
            EffectsNames => c.effects.names = v,
            EffectsAmplifier => c.effects.amplifier = v,
            EffectsGroup => c.effects.group = v,
            EffectsAmbient => c.effects.ambient = v,
            EffectsExpiring => c.effects.expiring_only = v,
            EffectsHorizontal => c.effects.horizontal = v,
            InvHotbar => c.inventory.hotbar = v,
            InvArmor => c.inventory.armor = v,
            InvOffhand => c.inventory.offhand = v,
            InvEmpty => c.inventory.empty = v,
            InvHighlight => c.inventory.highlight = v,
            InvCreative => c.inventory.creative = v,
            InvCompact => c.inventory.compact_rows = v,
            ArmorNames => c.armor.names = v,
            ArmorPercent => c.armor.percent = v,
            ArmorBars => c.armor.bars = v,
            ArmorHeld => c.armor.held = v,
            CoordsChunk => c.coords.chunk = v,
            CoordsFacing => c.coords.facing = v,
            CoordsDimension => c.coords.dimension = v,
            CoordsNether => c.coords.nether = v,
            CoordsYaw => c.coords.yaw_degrees = v,
            KeysMouse => c.keys.mouse = v,
            KeysSpace => c.keys.space = v,
            KeysSneakSprint => c.keys.sneak_sprint = v,
            KeysCps => c.keys.cps = v,
            KeysLabels => c.keys.labels = v,
            InfoFps => c.info.fps = v,
            InfoFrameMs => c.info.frame_ms = v,
            InfoLowFps => c.info.low_fps = v,
            InfoTime => c.info.time = v,
            InfoRealTime => c.info.real_time = v,
            InfoSpeed => c.info.speed = v,
            InfoCompass => c.info.compass = v,
            InfoClock24 => c.info.clock24 = v,
            HealthAbsorption => c.health.absorption = v,
            HealthArmor => c.health.armor_value = v,
            HealthFood => c.health.food = v,
            HealthAir => c.health.air = v,
            HealthXp => c.health.xp = v,
            HealthBars => c.health.bars = v,
            HeldOffhand => c.held.offhand = v,
            HeldArrows => c.held.arrows = v,
            HeldTotems => c.held.totems = v,
            HeldCounts => c.held.counts = v,
            TooltipFood => c.tooltips.food = v,
            TooltipSneak => c.tooltips.sneak_details = v,
            TextShadow => c.text_shadow = v,
            Snap => c.snap = v,
            ShowHelp => c.show_help = v,
            VanillaHud => c.vanilla.enabled = v,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Num {
    Opacity,
    Grid,
    AppleLowFood,
    EffectsWarn,
    EffectsRows,
    InvColumns,
    InvSlot,
    InvGap,
    ArmorThreshold,
    CoordsPrecision,
    KeysBox,
}

impl Num {
    pub fn get(self, c: &Config) -> f32 {
        use Num::*;
        match self {
            Opacity => f32::from(c.opacity),
            Grid => f32::from(c.grid),
            AppleLowFood => c.apple_skin.low_threshold as f32,
            EffectsWarn => c.effects.warn_seconds as f32,
            EffectsRows => c.effects.max_rows as f32,
            InvColumns => c.inventory.columns as f32,
            InvSlot => c.inventory.slot_size,
            InvGap => c.inventory.gap,
            ArmorThreshold => c.armor.threshold as f32,
            CoordsPrecision => c.coords.precision as f32,
            KeysBox => c.keys.box_size,
        }
    }
    pub fn set(self, c: &mut Config, v: f32) {
        use Num::*;
        let (lo, hi, _) = self.range();
        let v = v.clamp(lo, hi);
        match self {
            Opacity => c.opacity = v.round() as u8,
            Grid => c.grid = v.round().max(1.0) as u8,
            AppleLowFood => c.apple_skin.low_threshold = v.round() as i32,
            EffectsWarn => c.effects.warn_seconds = v.round() as i32,
            EffectsRows => c.effects.max_rows = v.round() as i32,
            InvColumns => c.inventory.columns = v.round() as i32,
            InvSlot => c.inventory.slot_size = v,
            InvGap => c.inventory.gap = v,
            ArmorThreshold => c.armor.threshold = v.round() as i32,
            CoordsPrecision => c.coords.precision = v.round() as i32,
            KeysBox => c.keys.box_size = v,
        }
    }
    pub fn range(self) -> (f32, f32, f32) {
        use Num::*;
        match self {
            Opacity => (0.0, 255.0, 5.0),
            Grid => (1.0, 64.0, 1.0),
            AppleLowFood => (0.0, 20.0, 1.0),
            EffectsWarn => (0.0, 600.0, 5.0),
            EffectsRows => (0.0, 64.0, 1.0),
            InvColumns => (1.0, 9.0, 1.0),
            InvSlot => (8.0, 32.0, 1.0),
            InvGap => (0.0, 6.0, 0.5),
            ArmorThreshold => (0.0, 100.0, 5.0),
            CoordsPrecision => (0.0, 4.0, 1.0),
            KeysBox => (8.0, 32.0, 1.0),
        }
    }
    pub fn suffix(self) -> &'static str {
        use Num::*;
        match self {
            Opacity | Grid | AppleLowFood | EffectsRows | InvColumns | ArmorThreshold
            | CoordsPrecision => "",
            EffectsWarn => "s",
            InvSlot | KeysBox => "px",
            InvGap => "px",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pick {
    Profile,
    Theme,
    View(ModuleId),
    EditorHotkey,
    SettingsHotkey,
    EffectTime,
    EffectSort,
    InvWhen,
    InvDurability,
    HeldDurability,
    InvCounts,
}

impl Pick {
    pub fn label(self, c: &Config) -> String {
        use Pick::*;
        match self {
            Profile => c.profile.label().into(),
            Theme => c.theme.label().into(),
            View(id) => c.view(id).label().into(),
            EditorHotkey => c.editor_hotkey.label().into(),
            SettingsHotkey => c.settings_hotkey.label().into(),
            EffectTime => c.effects.time.label().into(),
            EffectSort => c.effects.sort.label().into(),
            InvWhen => c.inventory.when.label().into(),
            InvDurability => c.inventory.durability.label().into(),
            HeldDurability => c.held.durability.label().into(),
            InvCounts => c.inventory.counts.label().into(),
        }
    }
    pub fn change(self, c: &mut Config, delta: i32) -> bool {
        use Pick::*;
        let up = delta >= 0;
        match self {
            Profile => {
                let p = if up {
                    c.profile.next()
                } else {
                    c.profile.prev()
                };
                c.apply_profile(p);
            }
            Theme => c.theme = if up { c.theme.next() } else { c.theme.prev() },
            View(id) => {
                let v = c.view(id);
                *c.view_mut(id) = if up { v.next() } else { v.prev() };
            }
            EditorHotkey => {
                c.editor_hotkey = if up {
                    c.editor_hotkey.next()
                } else {
                    c.editor_hotkey.prev()
                }
            }
            SettingsHotkey => {
                c.settings_hotkey = if up {
                    c.settings_hotkey.next()
                } else {
                    c.settings_hotkey.prev()
                }
            }
            EffectTime => {
                c.effects.time = if up {
                    c.effects.time.next()
                } else {
                    c.effects.time.prev()
                }
            }
            EffectSort => {
                c.effects.sort = if up {
                    c.effects.sort.next()
                } else {
                    c.effects.sort.prev()
                }
            }
            InvWhen => {
                c.inventory.when = if up {
                    c.inventory.when.next()
                } else {
                    c.inventory.when.prev()
                }
            }
            InvDurability => {
                c.inventory.durability = if up {
                    c.inventory.durability.next()
                } else {
                    c.inventory.durability.prev()
                }
            }
            HeldDurability => {
                c.held.durability = if up {
                    c.held.durability.next()
                } else {
                    c.held.durability.prev()
                }
            }
            InvCounts => {
                c.inventory.counts = if up {
                    c.inventory.counts.next()
                } else {
                    c.inventory.counts.prev()
                }
            }
        }
        true
    }
}

macro_rules! prev_impl {
    ($t:ty) => {
        impl $t {
            pub fn prev(self) -> Self {
                let all = Self::ALL;
                let i = all.iter().position(|v| *v == self).unwrap_or(0);
                all[(i + all.len() - 1) % all.len()]
            }
        }
    };
}
prev_impl!(Anchor);
prev_impl!(Theme);
prev_impl!(Profile);
prev_impl!(Hotkey);
prev_impl!(TimeFormat);
prev_impl!(EffectSort);
prev_impl!(InventoryWhen);
prev_impl!(ItemDurability);
prev_impl!(CountMode);
prev_impl!(View);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Module(ModuleId, ModField),
    Toggle(Flag),
    Amount(Num),
    Choice(Pick),
    ResetLayout,
    ResetAll,
    OpenLayout,
    OpenVanilla,
    CloseScreen,
}

impl Action {
    pub fn value(self, c: &Config) -> String {
        match self {
            Action::Module(id, field) => {
                let m = c.module(id);
                match field {
                    ModField::Enabled | ModField::Background | ModField::Border => {
                        let on = match field {
                            ModField::Enabled => m.enabled,
                            ModField::Background => m.background,
                            _ => m.border,
                        };
                        (if on { "ON" } else { "OFF" }).into()
                    }
                    ModField::Anchor => m.anchor.label().into(),
                    ModField::Scale => format!("{:.2}x", m.scale),
                }
            }
            Action::Toggle(f) => (if f.get(c) { "ON" } else { "OFF" }).into(),
            Action::Amount(n) => {
                let v = n.get(c);
                let (_, _, step) = n.range();
                if step < 1.0 || (v - v.round()).abs() > 0.001 {
                    format!("{v:.1}{}", n.suffix())
                } else {
                    format!("{}{}", v.round() as i32, n.suffix())
                }
            }
            Action::Choice(p) => p.label(c),
            _ => String::new(),
        }
    }
    pub fn change(self, c: &mut Config, delta: i32) -> bool {
        let up = delta >= 0;
        match self {
            Action::Module(id, field) => {
                match field {
                    ModField::Enabled => {
                        let on = !c.module(id).enabled;
                        c.module_mut(id).enabled = on;
                    }
                    ModField::Anchor => {
                        let m = c.module_mut(id);
                        m.anchor = if up { m.anchor.next() } else { m.anchor.prev() };
                    }
                    ModField::Scale => {
                        let m = c.module_mut(id);
                        *m = m.next_scale(up);
                    }
                    ModField::Background => {
                        let m = c.module_mut(id);
                        m.background = !m.background;
                    }
                    ModField::Border => {
                        let m = c.module_mut(id);
                        m.border = !m.border;
                    }
                }
                true
            }
            Action::Toggle(f) => {
                let v = !f.get(c);
                f.set(c, v);
                true
            }
            Action::Amount(n) => {
                let (lo, hi, step) = n.range();
                let v = (n.get(c) + step * (delta.signum() as f32)).clamp(lo, hi);
                n.set(c, v);
                true
            }
            Action::Choice(p) => p.change(c, delta),
            Action::ResetLayout => {
                c.reset_layout();
                true
            }
            Action::ResetAll => {
                *c = Config::default();
                true
            }
            Action::OpenLayout => {
                edit_editor(|e| e.page = EditorPage::Mods);
                set_pending(ScreenMode::Layout);
                false
            }
            Action::OpenVanilla => {
                edit_editor(|e| {
                    e.page = EditorPage::Vanilla;
                    e.selected_vanilla = e.selected_vanilla.or(Some(VanillaId::Hotbar));
                });
                set_pending(ScreenMode::Layout);
                false
            }
            Action::CloseScreen => {
                set_pending(ScreenMode::Closed);
                false
            }
        }
    }
}

fn all_setting_rows() -> Vec<(Action, &'static str, &'static str)> {
    use Action::*;
    use Flag as F;
    use ModField as MF;
    use ModuleId as M;
    use Num as N;
    use Pick as P;
    let mut rows: Vec<(Action, &'static str, &'static str)> = Vec::new();
    let module = |rows: &mut Vec<_>, id: M| {
        let s = id.title();
        rows.push((Module(id, MF::Enabled), "Enabled", s));
        rows.push((Choice(P::View(id)), "View", s));
        rows.push((Module(id, MF::Anchor), "Anchor", s));
        rows.push((Module(id, MF::Scale), "Scale", s));
        rows.push((Module(id, MF::Background), "Background", s));
        rows.push((Module(id, MF::Border), "Border", s));
    };

    rows.push((Toggle(F::Enabled), "HUD enabled", "General"));
    rows.push((Choice(P::Profile), "Profile", "General"));
    rows.push((Choice(P::Theme), "Theme", "General"));
    rows.push((Amount(N::Opacity), "Opacity", "General"));
    rows.push((Toggle(F::TextShadow), "Text shadow", "General"));
    rows.push((Toggle(F::Snap), "Snap to grid", "General"));
    rows.push((Amount(N::Grid), "Grid", "General"));
    rows.push((Choice(P::EditorHotkey), "Layout hotkey", "General"));
    rows.push((Choice(P::SettingsHotkey), "Settings hotkey", "General"));
    rows.push((Toggle(F::ShowHelp), "Show hints", "General"));
    rows.push((ResetLayout, "Reset layout", "General"));
    rows.push((ResetAll, "Reset everything", "General"));
    rows.push((OpenLayout, "Open layout editor", "General"));
    rows.push((OpenVanilla, "Edit vanilla HUD", "Vanilla HUD"));
    rows.push((Toggle(F::VanillaHud), "Move vanilla HUD", "Vanilla HUD"));

    module(&mut rows, M::AppleSkin);
    rows.push((Toggle(F::AppleSaturation), "Saturation", "AppleSkin"));
    rows.push((Toggle(F::AppleExhaustion), "Exhaustion", "AppleSkin"));
    rows.push((Toggle(F::AppleNumbers), "Numbers", "AppleSkin"));
    rows.push((Toggle(F::AppleRegen), "Regeneration timer", "AppleSkin"));
    rows.push((Toggle(F::ApplePreview), "Held food preview", "AppleSkin"));
    rows.push((Toggle(F::AppleBars), "Food icons", "AppleSkin"));
    rows.push((Amount(N::AppleLowFood), "Low food warning", "AppleSkin"));
    rows.push((Toggle(F::AppleCompact), "Compact", "AppleSkin"));

    module(&mut rows, M::Effects);
    rows.push((
        Toggle(F::EffectsHideVanilla),
        "Hide vanilla icons",
        "Effect timers",
    ));
    rows.push((Toggle(F::EffectsIcons), "Icons", "Effect timers"));
    rows.push((Toggle(F::EffectsNames), "Names", "Effect timers"));
    rows.push((Toggle(F::EffectsAmplifier), "Amplifier", "Effect timers"));
    rows.push((Choice(P::EffectTime), "Time format", "Effect timers"));
    rows.push((Choice(P::EffectSort), "Sort", "Effect timers"));
    rows.push((Amount(N::EffectsWarn), "Warning time", "Effect timers"));
    rows.push((Amount(N::EffectsRows), "Max rows", "Effect timers"));
    rows.push((Toggle(F::EffectsGroup), "Group by benefit", "Effect timers"));
    rows.push((Toggle(F::EffectsAmbient), "Ambient marker", "Effect timers"));
    rows.push((Toggle(F::EffectsExpiring), "Only expiring", "Effect timers"));
    rows.push((Toggle(F::EffectsHorizontal), "Horizontal", "Effect timers"));

    module(&mut rows, M::Inventory);
    rows.push((Amount(N::InvColumns), "Columns", "Inventory"));
    rows.push((Toggle(F::InvHotbar), "Hotbar", "Inventory"));
    rows.push((Toggle(F::InvArmor), "Armour", "Inventory"));
    rows.push((Toggle(F::InvOffhand), "Offhand", "Inventory"));
    rows.push((Toggle(F::InvEmpty), "Empty slots", "Inventory"));
    rows.push((Choice(P::InvCounts), "Counts", "Inventory"));
    rows.push((Choice(P::InvDurability), "Durability", "Inventory"));
    rows.push((Toggle(F::InvHighlight), "Selected slot", "Inventory"));
    rows.push((Choice(P::InvWhen), "When", "Inventory"));
    rows.push((Toggle(F::InvCreative), "In creative", "Inventory"));
    rows.push((Amount(N::InvSlot), "Slot size", "Inventory"));
    rows.push((Amount(N::InvGap), "Gap", "Inventory"));
    rows.push((Toggle(F::InvCompact), "Compact rows", "Inventory"));

    module(&mut rows, M::Armor);
    rows.push((Toggle(F::ArmorNames), "Piece names", "Armour & durability"));
    rows.push((Toggle(F::ArmorPercent), "Percent", "Armour & durability"));
    rows.push((Toggle(F::ArmorBars), "Bars", "Armour & durability"));
    rows.push((
        Toggle(F::ArmorHeld),
        "Include held items",
        "Armour & durability",
    ));
    rows.push((
        Amount(N::ArmorThreshold),
        "Low warning",
        "Armour & durability",
    ));

    module(&mut rows, M::Coords);
    rows.push((Toggle(F::CoordsChunk), "Chunk", "Coordinates"));
    rows.push((Toggle(F::CoordsFacing), "Facing", "Coordinates"));
    rows.push((Toggle(F::CoordsDimension), "Dimension", "Coordinates"));
    rows.push((Toggle(F::CoordsNether), "Other dimension", "Coordinates"));
    rows.push((Amount(N::CoordsPrecision), "Decimals", "Coordinates"));
    rows.push((Toggle(F::CoordsYaw), "Yaw in degrees", "Coordinates"));

    module(&mut rows, M::Keys);
    rows.push((Toggle(F::KeysMouse), "Mouse buttons", "Keystrokes"));
    rows.push((Toggle(F::KeysSpace), "Space bar", "Keystrokes"));
    rows.push((Toggle(F::KeysSneakSprint), "Sneak and sprint", "Keystrokes"));
    rows.push((Toggle(F::KeysCps), "CPS", "Keystrokes"));
    rows.push((Toggle(F::KeysLabels), "Key labels", "Keystrokes"));
    rows.push((Amount(N::KeysBox), "Key size", "Keystrokes"));

    module(&mut rows, M::Info);
    rows.push((Toggle(F::InfoFps), "FPS", "Performance & time"));
    rows.push((Toggle(F::InfoFrameMs), "Frame time", "Performance & time"));
    rows.push((Toggle(F::InfoLowFps), "1% low", "Performance & time"));
    rows.push((Toggle(F::InfoTime), "World time", "Performance & time"));
    rows.push((Toggle(F::InfoRealTime), "Real time", "Performance & time"));
    rows.push((
        Toggle(F::InfoClock24),
        "24-hour clock",
        "Performance & time",
    ));
    rows.push((Toggle(F::InfoSpeed), "Speed", "Performance & time"));
    rows.push((Toggle(F::InfoCompass), "Compass", "Performance & time"));

    module(&mut rows, M::Health);
    rows.push((Toggle(F::HealthAbsorption), "Absorption", "Player vitals"));
    rows.push((Toggle(F::HealthArmor), "Armour", "Player vitals"));
    rows.push((Toggle(F::HealthFood), "Food", "Player vitals"));
    rows.push((Toggle(F::HealthAir), "Air", "Player vitals"));
    rows.push((Toggle(F::HealthXp), "Experience", "Player vitals"));
    rows.push((Toggle(F::HealthBars), "Bars", "Player vitals"));

    module(&mut rows, M::Held);
    rows.push((Choice(P::HeldDurability), "Durability", "Held items"));
    rows.push((Toggle(F::HeldOffhand), "Offhand", "Held items"));
    rows.push((Toggle(F::HeldArrows), "Arrow count", "Held items"));
    rows.push((Toggle(F::HeldTotems), "Totem count", "Held items"));
    rows.push((Toggle(F::HeldCounts), "Stack counts", "Held items"));

    rows.push((Toggle(F::TooltipFood), "Food values", "Tooltips"));
    rows.push((Toggle(F::TooltipSneak), "Sneak details", "Tooltips"));
    rows.push((CloseScreen, "Close settings", "Tooltips"));
    rows
}

#[allow(dead_code)] // Kept for the tests and for callers that want the full list.
pub fn setting_rows() -> Vec<(Action, &'static str, &'static str)> {
    all_setting_rows()
}

#[derive(Clone, Debug, PartialEq)]
pub struct SettingLine {
    pub header: bool,
    pub text: String,
    pub action: Option<Action>,
    pub value: String,
}

pub fn setting_lines(cfg: &Config) -> Vec<SettingLine> {
    let mut out = Vec::new();
    let mut section = "";
    for (action, label, sec) in setting_rows() {
        if sec != section {
            out.push(SettingLine {
                header: true,
                text: sec.into(),
                action: None,
                value: String::new(),
            });
            section = sec;
        }
        out.push(SettingLine {
            header: false,
            text: label.into(),
            action: Some(action),
            value: action.value(cfg),
        });
    }
    out
}

pub struct SettingsView {
    pub panel: Rect,
    pub list: Rect,
    pub row_h: f32,
    pub first: usize,
    pub visible: usize,
    pub lines: usize,
}

/// Settings rows are deliberately tighter than the layout editor's list: the
/// whole configuration lives in one panel, and the world behind it stays
/// visible.
const SETTINGS_ROW_H: f32 = 12.0;

pub fn settings_view(screen: (f32, f32), lines: usize, scroll: i32) -> SettingsView {
    let (sw, sh) = screen;
    // A compact panel on the right: no full-screen veil, so the game stays
    // readable while a value is changed.
    let w = (sw * 0.46).clamp(200.0, 330.0);
    let h = (sh - 12.0).max(96.0);
    let panel = Rect::new(sw - w - 6.0, 6.0, w, h);
    let title_h = 18.0;
    let footer_h = 18.0;
    let list = Rect::new(
        panel.x + 6.0,
        panel.y + title_h,
        panel.w - 12.0,
        (panel.h - title_h - footer_h).max(SETTINGS_ROW_H),
    );
    let visible = (list.h / SETTINGS_ROW_H).floor().max(1.0) as usize;
    let max_first = lines.saturating_sub(visible);
    let first = (scroll.max(0) as usize).min(max_first);
    SettingsView {
        panel,
        list,
        row_h: SETTINGS_ROW_H,
        first,
        visible,
        lines,
    }
}

pub fn draw_settings(p: &mut dyn Painter, cfg: &Config, scroll: i32, mouse: [f32; 2]) {
    let col = cfg.theme.colours(cfg.opacity);
    let (sw, sh) = p.screen();
    let lines = setting_lines(cfg);
    let view = settings_view((sw, sh), lines.len(), scroll);
    // Only the panel is painted: the world keeps its colours and brightness.
    rounded_fill(p, view.panel, with_alpha(col.header, 228));
    rounded_border(p, view.panel, with_alpha(col.border, 150));
    p.fill(
        Rect::new(
            view.panel.x + 2.0,
            view.panel.y + 1.0,
            view.panel.w - 4.0,
            2.0,
        ),
        with_alpha(col.accent, 190),
    );
    txt(
        p,
        cfg,
        "HUD+ 4 · settings",
        view.panel.x + 7.0,
        view.panel.y + 5.0,
        col.accent,
        1.0,
    );
    let profile = format!("{} · {} on", cfg.profile.label(), enabled_count(cfg));
    let pw = p.width(&profile) * 0.78;
    txt(
        p,
        cfg,
        &profile,
        view.panel.right() - 7.0 - pw,
        view.panel.y + 6.0,
        col.muted,
        0.78,
    );
    let mut y = view.list.y;
    for line in lines.iter().skip(view.first).take(view.visible) {
        let row = Rect::new(view.list.x, y, view.list.w, view.row_h);
        if line.header {
            p.fill(row, with_alpha(col.border, 26));
            txt(
                p,
                cfg,
                &line.text,
                view.list.x + 2.0,
                y + 2.0,
                col.muted,
                0.75,
            );
            let lw = p.width(&line.text) * 0.75 + 8.0;
            let rest = (view.list.w - lw - 4.0).max(0.0);
            p.fill(
                Rect::new(view.list.x + lw, y + view.row_h / 2.0, rest, 0.5),
                with_alpha(col.border, 34),
            );
        } else {
            if row.contains(mouse) {
                rounded_fill(p, row, with_alpha(col.accent, 38));
            }
            txt(
                p,
                cfg,
                &line.text,
                view.list.x + 2.0,
                y + 1.5,
                col.text,
                0.86,
            );
            if !line.value.is_empty() {
                let vw = p.width(&line.value) * 0.86;
                txt(
                    p,
                    cfg,
                    &line.value,
                    view.list.right() - 4.0 - vw,
                    y + 1.5,
                    col.accent,
                    0.86,
                );
            }
        }
        y += view.row_h;
    }
    if view.lines > view.visible {
        let track = Rect::new(view.panel.right() - 4.0, view.list.y, 2.0, view.list.h);
        p.fill(track, with_alpha(col.border, 40));
        let frac = view.visible as f32 / view.lines as f32;
        let thumb = (track.h * frac).max(12.0);
        let t = view.first as f32 / (view.lines - view.visible).max(1) as f32;
        p.fill(
            Rect::new(track.x, track.y + (track.h - thumb) * t, 2.0, thumb),
            with_alpha(col.accent, 210),
        );
    }
    let more = format!(
        "{} / {}",
        (view.first + view.visible).min(view.lines),
        view.lines
    );
    let mw = p.width(&more) * 0.75;
    txt(
        p,
        cfg,
        &more,
        view.panel.right() - 8.0 - mw,
        view.panel.bottom() - 12.0,
        col.muted,
        0.75,
    );
    if cfg.show_help {
        let hint = if view.lines > view.visible {
            "Click: change · Wheel: scroll · Esc: close"
        } else {
            "Click: change · Esc: close"
        };
        txt(
            p,
            cfg,
            hint,
            view.panel.x + 7.0,
            view.panel.bottom() - 12.0,
            col.muted,
            0.72,
        );
    }
}

pub fn enabled_count(cfg: &Config) -> usize {
    ModuleId::ALL
        .iter()
        .filter(|id| cfg.module(**id).enabled)
        .count()
}

/// The line index under `at`, with the same geometry the list is drawn with.
fn settings_row_at(view: &SettingsView, at: [f32; 2]) -> Option<usize> {
    if !view.list.contains(at) {
        return None;
    }
    let row = ((at[1] - view.list.y) / view.row_h).floor();
    if row < 0.0 {
        return None;
    }
    let i = view.first + row as usize;
    (i < view.lines).then_some(i)
}

/// Handles a click on the settings list. False when it hit nothing.
pub fn settings_click(
    screen: (f32, f32),
    scroll: i32,
    cfg: &Config,
    at: [f32; 2],
    button: i32,
) -> bool {
    let lines = setting_lines(cfg);
    let view = settings_view(screen, lines.len(), scroll);
    let Some(i) = settings_row_at(&view, at) else {
        return false;
    };
    let Some(action) = lines[i].action else {
        return false;
    };
    let delta = if button == 1 { -1 } else { 1 };
    // A profile or a full reset sets the profile itself; everything else
    // makes the configuration custom.
    let changed = match action {
        Action::Choice(Pick::Profile) | Action::ResetAll => {
            let mut next = *cfg;
            let changed = action.change(&mut next, delta);
            if changed {
                set_config(next);
            }
            changed
        }
        _ => edit_config(|c| action.change(c, delta)),
    };
    let _ = changed;
    true
}

/// Scrolls the settings list by `steps`, kept within the list.
pub fn settings_scroll(screen: (f32, f32), scroll: i32, steps: f32, cfg: &Config) -> i32 {
    let lines = setting_lines(cfg).len();
    let view = settings_view(screen, lines, 0);
    let max_first = lines.saturating_sub(view.visible) as i32;
    (scroll - steps.round() as i32).clamp(0, max_first)
}

// ---------------------------------------------------------------------------
// Layout editor input
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// The editor's pages and the vanilla widgets
// ---------------------------------------------------------------------------

pub fn editor_tabs(screen: (f32, f32)) -> [(EditorPage, Rect); 2] {
    let w = 74.0;
    let h = 14.0;
    let x = (screen.0 - (w * 2.0 + 4.0)) / 2.0;
    [
        (EditorPage::Mods, Rect::new(x, 4.0, w, h)),
        (EditorPage::Vanilla, Rect::new(x + w + 4.0, 4.0, w, h)),
    ]
}

pub fn editor_tab_at(screen: (f32, f32), at: [f32; 2]) -> Option<EditorPage> {
    editor_tabs(screen)
        .into_iter()
        .find(|(_, r)| r.contains(at))
        .map(|(page, _)| page)
}

pub fn vanilla_rects(cfg: &Config, screen: (f32, f32)) -> Vec<(VanillaId, Rect)> {
    VanillaId::ALL
        .iter()
        .copied()
        .map(|id| (id, vanilla_rect(cfg, id, screen)))
        .collect()
}

/// The scale the health and hunger layer takes on the Vanilla page, which the
/// overlay takes with it.
pub fn apple_skin_scale(cfg: &Config) -> f32 {
    if !cfg.vanilla.enabled || !cfg.vanilla.get(VanillaId::PlayerHealth).on {
        return 1.0;
    }
    cfg.vanilla.get(VanillaId::PlayerHealth).scale
}

/// AppleSkin hangs on the vanilla hunger bar: moving or scaling the health and
/// hunger layer on the Vanilla page takes the overlay with it. Once that layer
/// is transformed the overlay is pinned to it — its top-right corner is the
/// bar's — so the module's own offset stops mattering; while the layer still
/// sits at vanilla's place the module keeps its own box, and dragging works.
pub fn apple_skin_follow(cfg: &Config, r: Rect, screen: (f32, f32)) -> Rect {
    if !cfg.vanilla.enabled {
        return r;
    }
    let el = cfg.vanilla.get(VanillaId::PlayerHealth);
    if !el.on || (el.offset == [0.0, 0.0] && (el.scale - 1.0).abs() < 0.001) {
        return r;
    }
    let t = vanilla_rect(cfg, VanillaId::PlayerHealth, screen);
    let m = &cfg.apple_skin.module;
    let pad = pad(m);
    let icon = 9.0 * m.scale * el.scale;
    Rect::new(t.right() + pad - r.w, t.y - (r.h - pad - icon), r.w, r.h)
}

fn vanilla_at(rects: &[(VanillaId, Rect)], at: [f32; 2]) -> Option<VanillaId> {
    rects
        .iter()
        .rev()
        .find(|(_, r)| r.contains(at))
        .map(|(id, _)| *id)
}

fn vanilla_list_panel(screen: (f32, f32)) -> Rect {
    let h = (screen.1 - 56.0).max(60.0);
    Rect::new(4.0, 22.0, 146.0, h)
}

/// The list's last rows are the selected widget and the keys, inside the
/// panel: nothing covers the hotbar and the bars it sits with.
const VANILLA_FOOTER: f32 = 30.0;
/// The list's title row: rows begin below it, in the drawing and in the hit
/// test alike.
const VANILLA_HEADER: f32 = 14.0;

fn vanilla_list_view(screen: (f32, f32), scroll: i32) -> (Rect, f32, usize, usize) {
    let panel = vanilla_list_panel(screen);
    let row_h = 13.0;
    let body = (panel.h - VANILLA_HEADER - VANILLA_FOOTER).max(row_h);
    let visible = (body / row_h).floor().max(1.0) as usize;
    let first = (scroll.max(0) as usize).min(VanillaId::ALL.len().saturating_sub(visible));
    (panel, row_h, first, visible)
}

fn vanilla_row_at(screen: (f32, f32), scroll: i32, at: [f32; 2]) -> Option<VanillaId> {
    let (panel, row_h, first, visible) = vanilla_list_view(screen, scroll);
    if !panel.contains(at) {
        return None;
    }
    let row = ((at[1] - panel.y - VANILLA_HEADER) / row_h).floor();
    if row < 0.0 || row >= visible as f32 {
        return None;
    }
    VanillaId::ALL.get(first + row as usize).copied()
}

pub fn draw_editor_tabs(p: &mut dyn Painter, cfg: &Config, page: EditorPage, mouse: [f32; 2]) {
    let col = cfg.theme.colours(cfg.opacity);
    let screen = p.screen();
    for (id, r) in editor_tabs(screen) {
        let on = id == page;
        rounded_fill(
            p,
            r,
            if on {
                with_alpha(col.accent, 210)
            } else {
                with_alpha(col.header, 220)
            },
        );
        rounded_border(p, r, with_alpha(col.border, 140));
        if !on && r.contains(mouse) {
            rounded_fill(p, r, with_alpha(col.accent, 36));
        }
        let label = match id {
            EditorPage::Mods => "HUD+",
            EditorPage::Vanilla => "Vanilla",
        };
        let lw = p.width(label);
        txt(
            p,
            cfg,
            label,
            r.x + (r.w - lw) / 2.0,
            r.y + 3.0,
            if on { col.header } else { col.text },
            0.9,
        );
    }
}

/// The vanilla page: real widgets stay visible under a light dim, their boxes
/// and a list let every one be picked, dragged and scaled.
pub fn draw_vanilla_editor(p: &mut dyn Painter, cfg: &Config, scroll: i32, mouse: [f32; 2]) {
    let col = cfg.theme.colours(cfg.opacity);
    let screen = p.screen();
    let e = editor();
    let rects = vanilla_rects(cfg, screen);
    for (id, r) in &rects {
        let el = cfg.vanilla.get(*id);
        let selected = e.selected_vanilla == Some(*id);
        let hovered = r.contains(mouse);
        // The full-screen layers would just frame the screen.
        let full = r.w >= screen.0 - 1.0 && r.h >= screen.1 - 1.0;
        if selected {
            rounded_fill(p, *r, with_alpha(col.accent, 46));
        } else if hovered {
            rounded_fill(p, *r, 0x14FF_FFFF);
        } else if el.on && !full {
            rounded_fill(p, *r, 0x2600_0000);
        }
        if !full || selected || hovered {
            let line = if !el.on {
                with_alpha(col.muted, 80)
            } else if selected {
                col.accent
            } else if hovered {
                with_alpha(col.accent, 190)
            } else {
                with_alpha(col.accent, 105)
            };
            rounded_border(p, *r, line);
        }
        // One label at a time: every widget's name at once collides, and the
        // list on the left already names them all.
        if (selected || hovered) && !full {
            let tag = if el.on {
                format!("{}  {:.2}x", id.title(), el.scale)
            } else {
                format!("{}  · vanilla", id.title())
            };
            let tw = (p.width(&tag) * 0.9 + 6.0).min(screen.0 - 4.0);
            let tx = r.x.min(screen.0 - tw - 2.0).max(2.0);
            let ty = if r.y >= 22.0 {
                r.y - 11.0
            } else {
                (r.bottom() + 1.0).min(screen.1 - 12.0)
            };
            p.fill(Rect::new(tx, ty, tw, 10.0), with_alpha(col.header, 225));
            txt(p, cfg, &tag, tx + 3.0, ty + 1.0, col.accent, 0.9);
        }
    }
    let (panel, row_h, first, visible) = vanilla_list_view(screen, scroll);
    rounded_fill(p, panel, with_alpha(col.header, 238));
    rounded_border(p, panel, with_alpha(col.border, 150));
    txt(
        p,
        cfg,
        "Vanilla HUD",
        panel.x + 5.0,
        panel.y + 3.0,
        col.accent,
        0.95,
    );
    let mut y = panel.y + VANILLA_HEADER;
    for (i, id) in VanillaId::ALL
        .iter()
        .copied()
        .enumerate()
        .skip(first)
        .take(visible)
    {
        let row = Rect::new(panel.x + 2.0, y, panel.w - 4.0, row_h);
        let el = cfg.vanilla.get(id);
        let selected = e.selected_vanilla == Some(id);
        if selected {
            rounded_fill(p, row, with_alpha(col.accent, 54));
        } else if row.contains(mouse) {
            rounded_fill(p, row, with_alpha(col.accent, 26));
        }
        if i % 2 == 0 {
            p.fill(
                Rect::new(row.x, row.bottom() - 0.5, row.w, 0.5),
                with_alpha(col.border, 24),
            );
        }
        txt(
            p,
            cfg,
            id.title(),
            row.x + 3.0,
            row.y + 1.0,
            if el.on { col.text } else { col.muted },
            0.88,
        );
        let v = if el.on {
            format!("{:.2}x", el.scale)
        } else {
            "off".into()
        };
        let vw = p.width(&v) * 0.8;
        txt(
            p,
            cfg,
            &v,
            row.right() - 4.0 - vw,
            row.y + 1.0,
            if el.on { col.accent } else { col.muted },
            0.8,
        );
        y += row_h;
    }
    // The summary and the keys live in the panel's foot: a bar across the
    // bottom would cover the hotbar and the bars being placed.
    let foot_y = panel.bottom() - VANILLA_FOOTER + 3.0;
    p.fill(
        Rect::new(panel.x + 5.0, foot_y - 3.0, panel.w - 10.0, 1.0),
        with_alpha(col.border, 65),
    );
    txt(
        p,
        cfg,
        &editor_summary(cfg),
        panel.x + 6.0,
        foot_y,
        col.text,
        0.8,
    );
    if cfg.show_help {
        txt(
            p,
            cfg,
            "Wheel scales  ·  R resets",
            panel.x + 6.0,
            foot_y + 10.0,
            col.muted,
            0.72,
        );
        txt(
            p,
            cfg,
            "Right-click: vanilla",
            panel.x + 6.0,
            foot_y + 19.0,
            col.muted,
            0.72,
        );
    }
}

fn vanilla_offset_for(
    cfg: &Config,
    id: VanillaId,
    screen: (f32, f32),
    at: [f32; 2],
    grab: [f32; 2],
) -> [f32; 2] {
    let el = cfg.vanilla.get(id);
    let d = id.default_box(screen);
    let (px, py) = (d.x + d.w * id.pivot().0, d.y + d.h * id.pivot().1);
    let s = el.scale;
    let unoffset = Rect::new(px + (d.x - px) * s, py + (d.y - py) * s, d.w * s, d.h * s);
    let mut x = (at[0] - grab[0]).clamp(0.0, (screen.0 - unoffset.w).max(0.0));
    let mut y = (at[1] - grab[1]).clamp(0.0, (screen.1 - unoffset.h).max(0.0));
    if cfg.snap {
        x = snap_to(cfg.grid, x);
        y = snap_to(cfg.grid, y);
    }
    [x - unoffset.x, y - unoffset.y]
}

fn vanilla_press(at: [f32; 2], button: i32) -> bool {
    let screen = screen_size_cache();
    let cfg = config(Config::default);
    let scroll = editor().scroll;
    if let Some(id) = vanilla_row_at(screen, scroll, at) {
        let r = vanilla_rect(&cfg, id, screen);
        edit_editor(|e| {
            e.selected_vanilla = Some(id);
            e.drag_vanilla = (button == 0).then_some(id);
            e.grab = [at[0] - r.x, at[1] - r.y];
        });
        return true;
    }
    let rects = vanilla_rects(&cfg, screen);
    let Some(id) = vanilla_at(&rects, at) else {
        if button == 0 {
            edit_editor(|e| {
                e.selected_vanilla = None;
                e.drag_vanilla = None;
            });
        }
        return false;
    };
    match button {
        0 => {
            let r = vanilla_rect(&cfg, id, screen);
            edit_editor(|e| {
                e.selected_vanilla = Some(id);
                e.drag_vanilla = Some(id);
                e.grab = [at[0] - r.x, at[1] - r.y];
            });
        }
        1 => {
            edit_config(|c| {
                let el = c.vanilla.get_mut(id);
                el.on = !el.on;
                true
            });
        }
        2 => {
            edit_config(|c| {
                *c.vanilla.get_mut(id) = VanillaElement::default();
                true
            });
        }
        _ => return false,
    }
    true
}

fn vanilla_drag(at: [f32; 2]) -> bool {
    let e = editor();
    let Some(id) = e.drag_vanilla else {
        return false;
    };
    let cfg = config(Config::default);
    let screen = screen_size_cache();
    let offset = vanilla_offset_for(&cfg, id, screen, at, e.grab);
    edit_config(|c| {
        let el = c.vanilla.get_mut(id);
        el.offset = offset;
        true
    })
}

fn vanilla_scroll(at: [f32; 2], steps: f32) -> bool {
    let screen = screen_size_cache();
    let cfg = config(Config::default);
    let scroll = editor().scroll;
    let id =
        vanilla_row_at(screen, scroll, at).or_else(|| vanilla_at(&vanilla_rects(&cfg, screen), at));
    let Some(id) = id else {
        return false;
    };
    let up = steps > 0.0;
    edit_config(|c| {
        let el = c.vanilla.get_mut(id);
        let step = 0.05;
        let next = if up { el.scale + step } else { el.scale - step };
        el.scale = (next * 100.0).round().clamp(25.0, 400.0) / 100.0;
        el.on = true;
        true
    });
    edit_editor(|e| e.selected_vanilla = Some(id));
    true
}

fn vanilla_key(key: &str) -> bool {
    let cfg = config(Config::default);
    let e = editor();
    match key {
        "key.keyboard.r" => {
            let _ = edit_config(|c| {
                c.vanilla = VanillaHud::default();
                true
            });
            true
        }
        "key.keyboard.space" | "key.keyboard.delete" => {
            if let Some(id) = e.selected_vanilla {
                let _ = edit_config(|c| {
                    let el = c.vanilla.get_mut(id);
                    el.on = !el.on;
                    true
                });
            }
            true
        }
        "key.keyboard.left" | "key.keyboard.right" | "key.keyboard.up" | "key.keyboard.down" => {
            let Some(id) = e.selected_vanilla else {
                return false;
            };
            let step = if cfg.snap { f32::from(cfg.grid) } else { 1.0 };
            let (dx, dy) = match key {
                "key.keyboard.left" => (-step, 0.0),
                "key.keyboard.right" => (step, 0.0),
                "key.keyboard.up" => (0.0, -step),
                _ => (0.0, step),
            };
            let _ = edit_config(|c| {
                let el = c.vanilla.get_mut(id);
                el.offset = [el.offset[0] + dx, el.offset[1] + dy];
                true
            });
            true
        }
        _ => false,
    }
}

fn module_at(rects: &[(ModuleId, Rect)], at: [f32; 2]) -> Option<ModuleId> {
    rects
        .iter()
        .rev()
        .find(|(_, r)| r.contains(at))
        .map(|(id, _)| *id)
}

/// Starts a drag (left), toggles a module (right) or cycles its anchor
/// (middle). False when the click hit no module.
pub fn editor_press(at: [f32; 2], button: i32) -> bool {
    if let Some(page) = editor_tab_at(screen_size_cache(), at) {
        edit_editor(|e| {
            e.page = page;
            e.drag = None;
            e.drag_vanilla = None;
        });
        return true;
    }
    if editor().page == EditorPage::Vanilla {
        return vanilla_press(at, button);
    }
    let rects = rects();
    let Some(id) = module_at(&rects, at) else {
        if button == 0 {
            edit_editor(|e| {
                e.selected = None;
                e.drag = None;
            });
        }
        return false;
    };
    match button {
        0 => {
            let Some((_, r)) = rects.iter().find(|(m, _)| *m == id) else {
                return false;
            };
            let grab = [at[0] - r.x, at[1] - r.y];
            edit_editor(|e| {
                e.selected = Some(id);
                e.drag = Some(id);
                e.grab = grab;
            });
        }
        1 => {
            let _ = edit_config(|c| {
                let on = !c.module(id).enabled;
                c.module_mut(id).enabled = on;
                true
            });
        }
        2 => {
            let _ = edit_config(|c| {
                let m = c.module_mut(id);
                m.anchor = m.anchor.next();
                true
            });
        }
        _ => return false,
    }
    true
}

/// Moves the dragged module, snapped to the grid. True when it moved.
pub fn editor_drag(at: [f32; 2]) -> bool {
    if editor().page == EditorPage::Vanilla {
        return vanilla_drag(at);
    }
    let e = editor();
    let Some(id) = e.drag else {
        return false;
    };
    let rects = rects();
    let Some((_, r)) = rects.iter().find(|(m, _)| *m == id) else {
        return false;
    };
    let cfg = config(Config::default);
    let (sw, sh) = screen_size_cache();
    let x = at[0] - e.grab[0];
    let y = at[1] - e.grab[1];
    let mut offset = offset_for(&cfg, id, (r.w, r.h), (x, y), (sw, sh));
    if cfg.snap {
        let base = Config::default().module(id).offset;
        offset = snap_offset(cfg.grid, base, offset);
    }
    edit_config(|c| {
        let m = c.module_mut(id);
        m.offset = offset;
        true
    })
}

thread_local! {
    static LAST_SCREEN: std::cell::Cell<(f32, f32)> = const { std::cell::Cell::new((0.0, 0.0)) };
}

pub fn remember_screen(size: (f32, f32)) {
    LAST_SCREEN.with(|s| s.set(size));
}

fn screen_size_cache() -> (f32, f32) {
    LAST_SCREEN.with(std::cell::Cell::get)
}

pub fn editor_release() {
    edit_editor(|e| e.drag = None);
}

/// Wheel over a module changes its scale. True when it did.
pub fn editor_scroll(at: [f32; 2], steps: f32) -> bool {
    if editor().page == EditorPage::Vanilla {
        return vanilla_scroll(at, steps);
    }
    let rects = rects();
    let Some(id) = module_at(&rects, at) else {
        return false;
    };
    let up = steps > 0.0;
    let changed = edit_config(|c| {
        let m = c.module_mut(id);
        *m = m.next_scale(up);
        true
    });
    edit_editor(|e| e.selected = Some(id));
    changed
}

/// The editor's keys. True when the key did something.
pub fn editor_key(key: &str) -> bool {
    if editor().page == EditorPage::Vanilla {
        return vanilla_key(key);
    }
    let cfg = config(Config::default);
    let e = editor();
    match key {
        "key.keyboard.r" => {
            let _ = edit_config(|c| {
                c.reset_layout();
                true
            });
            true
        }
        "key.keyboard.space" | "key.keyboard.delete" => {
            if let Some(id) = e.selected {
                let _ = edit_config(|c| {
                    let on = !c.module(id).enabled;
                    c.module_mut(id).enabled = on;
                    true
                });
            }
            true
        }
        "key.keyboard.left" | "key.keyboard.right" | "key.keyboard.up" | "key.keyboard.down" => {
            let Some(id) = e.selected else {
                return false;
            };
            let step = if cfg.snap { f32::from(cfg.grid) } else { 1.0 };
            let (dx, dy) = match key {
                "key.keyboard.left" => (-step, 0.0),
                "key.keyboard.right" => (step, 0.0),
                "key.keyboard.up" => (0.0, -step),
                _ => (0.0, step),
            };
            let _ = edit_config(|c| {
                let m = c.module_mut(id);
                m.offset = [m.offset[0] + dx, m.offset[1] + dy];
                true
            });
            true
        }
        _ => false,
    }
}

/// The selected module's summary, for the editor footer.
pub fn editor_summary(cfg: &Config) -> String {
    let e = editor();
    if e.page == EditorPage::Vanilla {
        let Some(id) = e.selected_vanilla else {
            return "Select a widget to move it".into();
        };
        let el = cfg.vanilla.get(id);
        return format!(
            "{}  ·  {:.2}x  ·  ({:.0}, {:.0})  ·  {}",
            id.title(),
            el.scale,
            el.offset[0],
            el.offset[1],
            if el.on { "on" } else { "vanilla" }
        );
    }
    let Some(id) = e.selected else {
        return "Select a module to move it".into();
    };
    let m = cfg.module(id);
    format!(
        "{}  ·  {}  ·  {:.2}x  ·  ({:.0}, {:.0})",
        id.title(),
        m.anchor.label(),
        m.scale,
        m.offset[0],
        m.offset[1]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot() -> Snapshot {
        Snapshot {
            in_world: true,
            screen: "playing",
            position: [12.5, 64.0, -3.25],
            yaw: 180.0,
            dimension: "minecraft:overworld".into(),
            health: 18.0,
            max_health: 20.0,
            food: 17,
            saturation: 4.25,
            exhaustion: 1.5,
            regen_ticks: 40,
            air: 300,
            max_air: 300,
            xp_level: 12,
            xp_progress: 0.45,
            fps: 144,
            frame_ms: 6.9,
            day_time: 6000,
            survival: true,
            selected: 2,
            ..Snapshot::default()
        }
    }

    struct MockPainter;

    impl Painter for MockPainter {
        fn screen(&self) -> (f32, f32) {
            (640.0, 360.0)
        }
        fn width(&self, text: &str) -> f32 {
            text.chars().count() as f32 * 6.0
        }
        fn fill(&mut self, _rect: Rect, _color: u32) {}
        fn text(&mut self, _t: &str, _x: f32, _y: f32, _c: u32, _s: bool, _z: f32) -> f32 {
            0.0
        }
        fn sprite(&mut self, _name: &str, _rect: Rect) -> bool {
            true
        }
        fn item(&mut self, _item: &ItemView, _x: f32, _y: f32, _size: f32) -> bool {
            true
        }
    }

    #[test]
    fn appleskin_starts_on_the_vanilla_hunger_bar() {
        let cfg = Config::default();
        let s = snapshot();
        let p = MockPainter;
        let size = size_apple_skin(&p, &s, &cfg);
        let r = module_rect(&cfg, ModuleId::AppleSkin, size, p.screen());
        // `Hud`: the right edge is `w / 2 + 91`, the icons top `h - 39`.
        assert!(
            (r.right() - PADDING - 411.0).abs() < 0.001,
            "right {}",
            r.right()
        );
        assert!(
            (r.bottom() - PADDING - 9.0 - 321.0).abs() < 0.001,
            "bottom {}",
            r.bottom()
        );
        // Nothing is drawn for the numbers until they are turned on.
        assert!(apple_text(&s, &cfg.apple_skin).is_empty());
        let mut numbers = cfg.apple_skin;
        numbers.show_numbers = true;
        assert_eq!(apple_text(&s, &numbers)[0].0, "17 / 20   Sat 4.2");
    }

    #[test]
    fn appleskin_follows_a_moved_hunger_bar() {
        let mut cfg = Config::default();
        let s = snapshot();
        let p = MockPainter;
        let size = size_apple_skin(&p, &s, &cfg);
        let r = module_rect(&cfg, ModuleId::AppleSkin, size, p.screen());
        cfg.vanilla.get_mut(VanillaId::PlayerHealth).offset = [12.0, -8.0];
        let moved = apple_skin_follow(&cfg, r, p.screen());
        assert!((moved.x - r.x - 12.0).abs() < 0.001, "x {}", moved.x);
        assert!((moved.y - r.y + 8.0).abs() < 0.001, "y {}", moved.y);
        // Scaling the layer doubles the bar and the icons with it.
        let el = cfg.vanilla.get_mut(VanillaId::PlayerHealth);
        el.offset = [0.0, 0.0];
        el.scale = 2.0;
        assert_eq!(apple_skin_scale(&cfg), 2.0);
        // Every frame the size is taken again; then the overlay covers the
        // scaled bar's right end.
        let size = size_apple_skin(&p, &s, &cfg);
        let r = module_rect(&cfg, ModuleId::AppleSkin, size, p.screen());
        assert!((r.w - size.0).abs() < 0.001);
        let doubled = apple_skin_follow(&cfg, r, p.screen());
        let pad = pad(&cfg.apple_skin.module);
        let v = vanilla_rect(&cfg, VanillaId::PlayerHealth, p.screen());
        // The box's right edge is the bar's plus its padding; the icons' top
        // is the bar's top.
        assert!((doubled.right() - pad - v.right()).abs() < 0.001);
        assert!((doubled.y + (doubled.h - pad - 9.0 * 2.0) - v.y).abs() < 0.001);
        assert!(
            (r.w - (HUNGER_BAR * 2.0 + 2.0 * pad)).abs() < 0.001,
            "w {}",
            r.w
        );
        // Turning the layer off leaves the overlay where vanilla draws the bar.
        cfg.vanilla.get_mut(VanillaId::PlayerHealth).on = false;
        assert_eq!(apple_skin_scale(&cfg), 1.0);
        let still = apple_skin_follow(&cfg, r, p.screen());
        assert_eq!(still, r);
    }

    #[test]
    fn appleskin_icons_stay_on_the_bar_when_numbers_widen_the_box() {
        let p = MockPainter;
        let s = snapshot();
        let plain = Config::default();
        let mut numbers = Config::default();
        numbers.apple_skin.show_numbers = true;
        numbers.apple_skin.show_saturation = true;
        let small = size_apple_skin(&p, &s, &plain);
        let wide = size_apple_skin(&p, &s, &numbers);
        assert!(wide.0 > small.0);
        let m = numbers.apple_skin.module;
        let scale = numbers.apple_skin.module.scale * apple_skin_scale(&numbers);
        let pad = pad(&m);
        assert_eq!(
            apple_skin_icon_right(&plain.apple_skin.module, scale, pad, p.screen()),
            apple_skin_icon_right(&m, scale, pad, p.screen())
        );
    }

    #[test]
    fn the_flash_pulses_like_appleskins() {
        assert_eq!(flash_alpha(0), 0.0);
        assert!((flash_alpha(4) - 0.5).abs() < 0.001);
        assert_eq!(flash_alpha(8), 1.0);
        assert_eq!(flash_alpha(15), 1.0);
        assert_eq!(flash_alpha(16), 1.0);
        assert!((flash_alpha(20) - 0.5).abs() < 0.001);
        assert_eq!(flash_alpha(24), 0.0);
        assert_eq!(flash_alpha(30), 0.0);
        assert_eq!(flash_alpha(32), 0.0);
    }

    #[test]
    fn the_saturation_outlines_are_four_nine_pixel_tiles() {
        let rgba = apple_icons_rgba();
        assert_eq!(rgba.len(), 36 * 9 * 4);
        assert!(rgba.chunks(4).any(|px| px[3] > 0));
        assert!(rgba.chunks(4).any(|px| px[3] == 0));
    }

    #[test]
    fn config_round_trips_through_json() {
        let c = Config::default();
        let text = c.to_json();
        let back = Config::from_json(&text);
        assert_eq!(c, back);
    }

    #[test]
    fn damaged_config_keeps_defaults_and_clamps() {
        let back = Config::from_json(r#"{"opacity": 9999, "grid": -4, "theme": "nope"}"#);
        assert_eq!(back.opacity, 255);
        assert_eq!(back.grid, 1);
        assert_eq!(back.theme, Theme::Dark);
    }

    #[test]
    fn anchors_place_boxes_at_their_edges() {
        let size = (20.0, 10.0);
        let screen = (100.0, 50.0);
        assert_eq!(Anchor::TopLeft.place(size, [1.0, 2.0], screen), (1.0, 2.0));
        assert_eq!(
            Anchor::TopRight.place(size, [-1.0, 0.0], screen),
            (79.0, 0.0)
        );
        assert_eq!(
            Anchor::BottomCenter.place(size, [0.0, -4.0], screen),
            (40.0, 36.0)
        );
        assert_eq!(
            Anchor::MiddleRight.place(size, [0.0, 0.0], screen),
            (80.0, 20.0)
        );
        assert_eq!(Anchor::nearest((0.0, 0.0), size, screen), Anchor::TopLeft);
        assert_eq!(
            Anchor::nearest((90.0, 40.0), size, screen),
            Anchor::BottomRight
        );
    }

    #[test]
    fn pre_offsets_undo_placement() {
        let cfg = Config::default();
        let size = (30.0, 20.0);
        let screen = (200.0, 300.0);
        for id in ModuleId::ALL {
            let r = module_rect(&cfg, id, size, screen);
            let off = offset_for(&cfg, id, size, (r.x, r.y), screen);
            let m = cfg.module(id);
            assert!((off[0] - m.offset[0]).abs() < 0.001);
            assert!((off[1] - m.offset[1]).abs() < 0.001);
        }
    }

    #[test]
    fn a_module_never_leaves_the_screen() {
        let cfg = Config::default();
        let size = (30.0, 20.0);
        let screen = (100.0, 80.0);
        let r = module_rect(&cfg, ModuleId::Inventory, size, screen);
        assert!(r.x >= 0.0 && r.y >= 0.0);
        assert!(r.right() <= screen.0 && r.bottom() <= screen.1);
        let off = offset_for(&cfg, ModuleId::Inventory, size, (0.0, 0.0), screen);
        assert_eq!(off, [0.0, -60.0]);
    }

    #[test]
    fn vanilla_widgets_start_where_vanilla_draws_them() {
        let cfg = Config::default();
        let screen = (320.0, 240.0);
        for id in VanillaId::ALL {
            let d = id.default_box(screen);
            let r = vanilla_rect(&cfg, id, screen);
            assert!((r.x - d.x).abs() < 0.001, "{:?} x", id);
            assert!((r.y - d.y).abs() < 0.001, "{:?} y", id);
            assert!((r.w - d.w).abs() < 0.001, "{:?} w", id);
            assert!((r.h - d.h).abs() < 0.001, "{:?} h", id);
        }
    }

    #[test]
    fn vanilla_widgets_move_and_scale_around_their_pivot() {
        let mut cfg = Config::default();
        let screen = (320.0, 240.0);
        let d = VanillaId::Hotbar.default_box(screen);
        let el = cfg.vanilla.get_mut(VanillaId::Hotbar);
        el.offset = [10.0, -12.0];
        el.scale = 1.5;
        let r = vanilla_rect(&cfg, VanillaId::Hotbar, screen);
        let (px, py) = (d.x + d.w * 0.5, d.y + d.h * 0.5);
        assert!((r.w - d.w * 1.5).abs() < 0.001);
        assert!((r.h - d.h * 1.5).abs() < 0.001);
        assert!((r.x - (px + (d.x - px) * 1.5 + 10.0)).abs() < 0.001);
        assert!((r.y - (py + (d.y - py) * 1.5 - 12.0)).abs() < 0.001);
    }

    #[test]
    fn vanilla_layout_round_trips_through_json() {
        let mut c = Config::default();
        {
            let el = c.vanilla.get_mut(VanillaId::Chat);
            el.offset = [24.0, -8.0];
            el.scale = 0.8;
            el.on = false;
        }
        let back = Config::from_json(&c.to_json());
        assert_eq!(back.vanilla, c.vanilla);
        assert_eq!(back.vanilla.get(VanillaId::Chat).offset, [24.0, -8.0]);
        assert_eq!(back.vanilla.get(VanillaId::Chat).scale, 0.8);
        assert!(!back.vanilla.get(VanillaId::Chat).on);
    }

    #[test]
    fn vanilla_layers_map_to_their_widgets() {
        for id in VanillaId::ALL {
            assert_eq!(VanillaId::from_layer(id.layer()), Some(id));
        }
        assert_eq!(VanillaId::from_layer("minecraft:nope"), None);
    }

    #[test]
    fn coordinates_read_like_the_game() {
        let s = snapshot();
        let c = Coords::default();
        let lines = coords_lines(&s, &c);
        assert_eq!(lines[0], "XYZ 12.5 64.0 -3.2");
        assert_eq!(lines[1], "Facing North (-Z)");
        assert!(lines[2].starts_with("Nether 1.6 64.0 -0.4"));
        assert_eq!(facing_name(0.0), "South (+Z)");
        assert_eq!(facing_short(-90.0), "E");
    }

    #[test]
    fn the_clock_starts_the_day_at_six() {
        assert_eq!(world_clock(0), (0, 6, 0));
        assert_eq!(world_clock(6000), (0, 12, 0));
        assert_eq!(world_clock(18000), (0, 0, 0));
        assert_eq!(clock_text(6000), "Day 0 12:00");
        assert_eq!(real_clock(13, 5, 9, true), "13:05:09");
        assert_eq!(real_clock(13, 5, 9, false), "1:05 PM");
        assert_eq!(real_clock(0, 30, 0, false), "12:30 AM");
    }

    #[test]
    fn durations_have_every_format() {
        assert_eq!(duration_text(-1, TimeFormat::Clock), "∞");
        assert_eq!(duration_text(200, TimeFormat::Seconds), "10s");
        assert_eq!(duration_text(1250, TimeFormat::Clock), "1:02");
        assert_eq!(duration_text(1250, TimeFormat::Both), "1:02 (62s)");
        assert_eq!(duration_text(200, TimeFormat::Ticks), "200t");
        assert_eq!(roman(1), "I");
        assert_eq!(roman(4), "IV");
        assert_eq!(roman(0), "");
    }

    #[test]
    fn wear_reads_from_full_to_broken() {
        let item = ItemView {
            id: "minecraft:diamond_pickaxe".into(),
            name: "Diamond Pickaxe".into(),
            count: 1,
            damage: 781,
            max_damage: 1561,
        };
        assert_eq!(item.left_percent(), Some(50));
        assert!((item.wear().unwrap() - 0.5).abs() < 0.001);
        let stone = ItemView {
            id: "minecraft:stone".into(),
            name: "Stone".into(),
            count: 64,
            ..ItemView::default()
        };
        assert_eq!(stone.wear(), None);
    }

    #[test]
    fn effects_sort_by_duration_name_and_strength() {
        let mut s = snapshot();
        s.effects = vec![
            EffectView {
                id: "speed".into(),
                name: "Speed".into(),
                sprite: "mob_effect/speed".into(),
                amplifier: 1,
                duration: 400,
                beneficial: true,
                ..EffectView::default()
            },
            EffectView {
                id: "poison".into(),
                name: "Poison".into(),
                sprite: "mob_effect/poison".into(),
                amplifier: 0,
                duration: 100,
                beneficial: false,
                ..EffectView::default()
            },
        ];
        let c = Effects::default();
        let cells = effect_cells(&s, &c);
        assert_eq!(cells[0].name, "Poison");
        assert_eq!(cells[1].name, "Speed");
        assert_eq!(cells[1].amplifier, "II");
        let mut c = Effects {
            sort: EffectSort::Amplifier,
            ..Default::default()
        };
        assert_eq!(effect_cells(&s, &c)[0].name, "Speed");
        c.sort = EffectSort::Name;
        assert_eq!(effect_cells(&s, &c)[0].name, "Poison");
    }

    #[test]
    fn inventory_cells_follow_the_hotbar_and_columns() {
        let mut s = snapshot();
        s.inventory = (0..36)
            .map(|i| {
                Some(ItemView {
                    id: format!("minecraft:item{i}"),
                    name: format!("Item {i}"),
                    count: 1,
                    ..ItemView::default()
                })
            })
            .collect();
        s.selected = 3;
        let mut c = Inventory::default();
        // The default inventory HUD leaves the hotbar row to vanilla and its
        // own hotbar, so it starts with the 27 backpack slots.
        assert_eq!(inventory_cells(&s, &c).len(), 27);
        c.hotbar = true;
        let cells = inventory_cells(&s, &c);
        assert_eq!(cells.len(), 36);
        assert!(cells[3].selected);
    }

    #[test]
    fn settings_rows_all_apply_and_keep_the_value_in_range() {
        for (action, label, _) in setting_rows() {
            let mut cfg = Config::default();
            action.change(&mut cfg, 1);
            action.change(&mut cfg, -1);
            // Actions that open screens are not configuration.
            let _ = label;
            if let Action::Amount(n) = action {
                let (lo, hi, _) = n.range();
                let v = n.get(&cfg);
                assert!(v >= lo && v <= hi, "{label} out of range");
            }
            // The value string never panics for any action.
            let _ = action.value(&cfg);
        }
    }

    #[test]
    fn toggles_and_cycles_move_in_both_directions() {
        let mut cfg = Config::default();
        let before = cfg.theme;
        Action::Choice(Pick::Theme).change(&mut cfg, 1);
        assert_eq!(cfg.theme, before.next());
        Action::Choice(Pick::Theme).change(&mut cfg, -1);
        assert_eq!(cfg.theme, before);
        let before = cfg.coords.facing;
        Action::Toggle(Flag::CoordsFacing).change(&mut cfg, 1);
        assert_eq!(cfg.coords.facing, !before);
        let a = Action::Module(ModuleId::Info, ModField::Enabled);
        assert!(!cfg.module(ModuleId::Info).enabled);
        a.change(&mut cfg, 0);
        assert!(cfg.module(ModuleId::Info).enabled);
        a.change(&mut cfg, 0);
        assert!(!cfg.module(ModuleId::Info).enabled);
    }

    #[test]
    fn profiles_reset_the_layout_and_pick_modules() {
        let mut cfg = Config::default();
        cfg.module_mut(ModuleId::Info).offset = [42.0, 42.0];
        cfg.apply_profile(Profile::Pvp);
        assert_eq!(cfg.profile, Profile::Pvp);
        assert!(cfg.module(ModuleId::Keys).enabled);
        assert!(!cfg.module(ModuleId::Inventory).enabled);
        assert_eq!(
            cfg.module(ModuleId::Info).offset,
            Config::default().module(ModuleId::Info).offset
        );
    }

    #[test]
    fn snapping_keeps_the_default_offset_reachable() {
        // The default offsets are not multiples of the default grid: a drag
        // back to where a module started must land on the exact default, not
        // on the nearest point of the absolute grid.
        let base = [3.0, -41.0];
        assert_eq!(snap_offset(8, base, base), base);
        assert_eq!(snap_offset(8, base, [base[0] + 2.0, base[1] - 3.0]), base);
        assert_eq!(
            snap_offset(8, base, [base[0] + 9.0, base[1] - 7.0]),
            [base[0] + 8.0, base[1] - 8.0]
        );
        assert_eq!(
            snap_offset(8, base, [base[0] + 16.0, base[1]]),
            [19.0, -41.0]
        );
        // Vanilla widgets start at zero, so their absolute grid is unchanged.
        assert_eq!(snap_offset(8, [0.0, 0.0], [3.0, 4.0]), [0.0, 8.0]);
    }

    #[test]
    fn snapping_rounds_to_the_grid() {
        assert_eq!(snap_to(8, 3.0), 0.0);
        assert_eq!(snap_to(8, 4.0), 8.0);
        assert_eq!(snap_to(10, 26.0), 30.0);
        assert_eq!(snap_to(1, 3.7), 4.0);
    }

    #[test]
    fn settings_lines_hold_every_section_on_one_screen() {
        let cfg = Config::default();
        let lines = setting_lines(&cfg);
        assert!(lines[0].header);
        assert_eq!(lines[0].text, "General");
        // Every module's own section is in the same scrolling list, with its
        // six module rows and at least one option of its own, and no row opens
        // a separate page.
        for id in ModuleId::ALL {
            let title = id.title();
            let start = lines
                .iter()
                .position(|l| l.header && l.text == title)
                .unwrap_or_else(|| panic!("no section for {title}"));
            assert!(lines[start + 1..].iter().any(|l| l.text == "Enabled"));
            assert!(lines[start + 1..].iter().any(|l| l.text == "Anchor"));
            assert!(
                lines[start + 1..].iter().take_while(|l| !l.header).count() > 2,
                "{title} has no options of its own"
            );
        }
        assert!(
            lines
                .iter()
                .all(|l| !matches!(l.action, Some(Action::CloseScreen)))
                || lines.iter().any(|l| l.text == "Close settings")
        );
        assert!(lines.iter().any(|l| l.header && l.text == "Vanilla HUD"));
        assert!(lines.iter().any(|l| l.header && l.text == "Tooltips"));
        let view = settings_view((320.0, 200.0), lines.len(), 0);
        assert!(view.visible > 0);
        assert_eq!(view.first, 0);
        assert!(view.panel.right() <= 320.0);
        assert!(view.panel.w < 320.0);
    }

    #[test]
    fn settings_row_hit_test_follows_the_drawn_row() {
        let cfg = Config::default();
        let lines = setting_lines(&cfg);
        let screen = (640.0, 360.0);
        let scroll = 7;
        let view = settings_view(screen, lines.len(), scroll);
        for row in 0..view.visible {
            let at = [
                view.list.x + view.list.w / 2.0,
                view.list.y + (row as f32 + 0.5) * view.row_h,
            ];
            assert_eq!(
                settings_row_at(&view, at),
                Some(view.first + row),
                "row {row} maps below itself"
            );
        }
        // The title and the footer are not rows.
        assert_eq!(
            settings_row_at(&view, [view.list.x + 4.0, view.panel.y + 4.0]),
            None
        );
        assert_eq!(
            settings_row_at(
                &view,
                [
                    view.list.x + 4.0,
                    view.panel.bottom() - 4.0
                ]
            ),
            None
        );
        assert_eq!(
            settings_view(screen, lines.len(), 0).first, 0,
            "first row is visible without scrolling"
        );
    }

    #[test]
    fn vanilla_list_hit_test_picks_the_row_under_the_pointer() {
        let screen = (640.0, 360.0);
        let (panel, row_h, first, visible) = vanilla_list_view(screen, 0);
        let rows = visible.min(VanillaId::ALL.len() - first);
        for row in 0..rows {
            let at = [
                panel.x + 10.0,
                panel.y + VANILLA_HEADER + (row as f32 + 0.5) * row_h,
            ];
            assert_eq!(
                vanilla_row_at(screen, 0, at),
                Some(VanillaId::ALL[first + row]),
                "row {row} selects the wrong widget"
            );
        }
        // The title and the footer are not rows.
        assert_eq!(
            vanilla_row_at(screen, 0, [panel.x + 10.0, panel.y + 5.0]),
            None
        );
        assert_eq!(
            vanilla_row_at(
                screen,
                0,
                [panel.x + 10.0, panel.y + VANILLA_HEADER + visible as f32 * row_h + 2.0]
            ),
            None
        );
        // Scrolled lists hit their own first row, not the first widget.
        let (_, _, first, _) = vanilla_list_view(screen, 2);
        assert_eq!(
            vanilla_row_at(
                screen,
                2,
                [panel.x + 10.0, panel.y + VANILLA_HEADER + row_h / 2.0]
            ),
            Some(VanillaId::ALL[first])
        );
    }

    #[test]
    fn subtitle_box_sits_where_the_captions_are_drawn() {
        let screen = (640.0, 360.0);
        let box_ = VanillaId::Subtitle.default_box(screen);
        // `SubtitleOverlay` right-aligns the captions above the hotbar.
        assert!(box_.right() >= screen.0 - 2.0);
        assert!(box_.y > screen.1 - 60.0, "captions belong at the bottom");
        assert!(box_.h > 0.0 && box_.w > 0.0);
        // A moved subtitle keeps its offset as the distance from vanilla's
        // own place, so the editor's box and the captions travel together.
        let mut cfg = Config::default();
        cfg.vanilla.get_mut(VanillaId::Subtitle).offset = [0.0, -150.0];
        assert_eq!(
            vanilla_rect(&cfg, VanillaId::Subtitle, screen).y,
            box_.y - 150.0
        );
    }

    #[test]
    fn item_names_fall_back_to_a_readable_path() {
        assert_eq!(pretty_id("minecraft:diamond_pickaxe"), "Diamond Pickaxe");
        assert_eq!(pretty_id("mymod:odd/thing"), "Odd/thing");
    }

    #[test]
    fn arrows_and_totems_are_counted() {
        let mut s = snapshot();
        s.inventory = vec![
            Some(ItemView {
                id: "minecraft:arrow".into(),
                count: 32,
                ..ItemView::default()
            }),
            Some(ItemView {
                id: "minecraft:spectral_arrow".into(),
                count: 8,
                ..ItemView::default()
            }),
            Some(ItemView {
                id: "minecraft:totem_of_undying".into(),
                count: 2,
                ..ItemView::default()
            }),
        ];
        assert_eq!(s.arrows_in_inventory(), 40);
        assert_eq!(s.totems_in_inventory(), 2);
        assert!(is_arrow("minecraft:arrow"));
        assert!(!is_arrow("minecraft:arrowhead"));
    }

    #[test]
    fn views_cycle_and_parse() {
        assert_eq!(View::ALL.len(), 4);
        assert_eq!(View::Minimal.next(), View::Compact);
        assert_eq!(View::Compact.next(), View::Standard);
        assert_eq!(View::Standard.next(), View::Full);
        assert_eq!(View::Full.next(), View::Minimal);
        assert_eq!(View::Full.prev(), View::Standard);
        assert_eq!(View::parse("compact"), View::Compact);
        assert_eq!(View::parse("minimal"), View::Minimal);
        assert_eq!(View::parse("full"), View::Full);
        assert_eq!(View::parse("nope"), View::Standard);
    }

    #[test]
    fn views_shrink_and_grow_every_module() {
        let s = snapshot();
        let view = |v: View| v;

        let apple = AppleSkin {
            view: view(View::Minimal),
            ..AppleSkin::default()
        };
        assert!(apple_text(&s, &apple).is_empty());
        assert!(!apple.effective().show_bars);
        let apple = AppleSkin {
            view: View::Full,
            ..AppleSkin::default()
        };
        assert!(apple.effective().show_numbers);
        assert!(apple.effective().show_exhaustion);
        assert!(apple.effective().show_bars);

        let effects = Effects {
            view: View::Minimal,
            ..Effects::default()
        };
        assert_eq!(effects.effective().time, TimeFormat::Off);
        assert!(!effects.effective().names);
        assert_eq!(effects_shown(10, &effects.effective()), 3);
        let effects = Effects {
            view: View::Full,
            ..Effects::default()
        };
        assert_eq!(effects.effective().max_rows, 0);
        assert!(effects.effective().names);
        assert_eq!(effects_shown(10, &effects.effective()), 10);

        let mut s36 = snapshot();
        s36.inventory = vec![None; 36];
        let inv = Inventory {
            view: View::Minimal,
            ..Inventory::default()
        };
        assert_eq!(inventory_cells(&s36, &inv).len(), 9);
        assert!(!inv.effective().grid);
        let inv = Inventory {
            view: View::Full,
            ..Inventory::default()
        };
        assert_eq!(inventory_cells(&s36, &inv).len(), 36);
        assert!(inv.effective().armor);

        let coords = Coords {
            view: View::Minimal,
            ..Coords::default()
        };
        assert_eq!(coords_lines(&s, &coords).len(), 1);
        let coords = Coords {
            view: View::Compact,
            ..Coords::default()
        };
        assert_eq!(coords_lines(&s, &coords).len(), 2);
        let coords = Coords {
            view: View::Full,
            ..Coords::default()
        };
        assert!(coords_lines(&s, &coords).len() >= 4);

        let keys = Keys {
            view: View::Minimal,
            ..Keys::default()
        };
        assert_eq!(key_rows(&keys.effective()), 2);
        let keys = Keys {
            view: View::Compact,
            ..Keys::default()
        };
        assert_eq!(key_rows(&keys.effective()), 3);
        let keys = Keys {
            view: View::Full,
            ..Keys::default()
        };
        assert_eq!(key_rows(&keys.effective()), 5);

        let info = Info {
            view: View::Minimal,
            ..Info::default()
        };
        assert_eq!(info_lines(&s, &info).len(), 1);
        let info = Info {
            view: View::Full,
            ..Info::default()
        };
        assert!(info_lines(&s, &info).len() >= 5);

        let mut full = snapshot();
        full.absorption = 4.0;
        full.air = 120;
        let health = Health {
            view: View::Minimal,
            ..Health::default()
        };
        assert_eq!(health_rows(&full, &health).len(), 1);
        let health = Health {
            view: View::Compact,
            ..Health::default()
        };
        assert_eq!(health_rows(&full, &health).len(), 2);
        let health = Health {
            view: View::Full,
            ..Health::default()
        };
        assert_eq!(health_rows(&full, &health).len(), 6);

        let item = ItemView {
            id: "minecraft:diamond_helmet".into(),
            name: "Diamond Helmet".into(),
            count: 1,
            damage: 200,
            max_damage: 363,
        };
        let mut armored = snapshot();
        armored.armor_items = vec![Some(item.clone())];
        armored.held = Some(item.clone());
        armored.offhand = Some(item);
        let armor = Armor {
            view: View::Minimal,
            ..Armor::default()
        };
        let rows = armor_rows(&armored, &armor);
        assert_eq!(rows.len(), 1);
        assert!(rows[0].name.is_empty());
        assert!(rows[0].detail.is_empty());
        let armor = Armor {
            view: View::Full,
            ..Armor::default()
        };
        assert_eq!(armor_rows(&armored, &armor).len(), 3);
        assert!(armor_rows(&armored, &armor)[0].name.contains("Helmet"));

        let held = Held {
            view: View::Minimal,
            ..Held::default()
        };
        assert_eq!(held_rows(&armored, &held).len(), 1);
        assert!(held_rows(&armored, &held)[0].detail.is_empty());
        let held = Held {
            view: View::Full,
            ..Held::default()
        };
        assert_eq!(held_rows(&armored, &held).len(), 2);
        assert!(held_rows(&armored, &held)[1].name.starts_with("Off: "));
    }
}
