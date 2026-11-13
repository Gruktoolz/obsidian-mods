// The Basic HUD: the effects on the player with their time left, the potions,
// golden apples and totems carried, the durability of armour, elytra and
// shield, and how many totems each player has used. This file is both the
// mod's Cargo library (its tests) and the module the bundle adds to the game,
// so it knows nothing of the game's types.
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};

/// The mod's id (its settings folder), name and version.
pub const ID: &str = "basic_hud";
pub const NAME: &str = "The Basic HUD";
pub const VERSION: &str = "0.5.0";
pub const GUIDE_SEEN_KEY: &str = "guide_seen";
pub const DESCRIPTION: &str = "Visual client HUD and in-game editor: effect timers, consumable counts, totem pops, armour durability, coordinates, telemetry and player health.";
#[allow(dead_code)] // Retained for older helpers using the runtime's public interface.
pub const BACKGROUND: u32 = 0x7800_0000;
pub const INK: u32 = 0xFF_FFFF;
pub const GOOD: u32 = 0x7C_E0_7C;
pub const BAD: u32 = 0xFF_70_70;
pub const WARNING: u32 = 0xFF_C8_40;
pub const TOTEM: u32 = 0xFF_D8_50;
/// Effects this close to their end show in `WARNING`, in ticks.
pub const ENDING: i32 = 200;
pub const GOLDEN_APPLE: &str = "minecraft:golden_apple";
pub const ENCHANTED_APPLE: &str = "minecraft:enchanted_golden_apple";
pub const TOTEM_ITEM: &str = "minecraft:totem_of_undying";
/// Beside a count of totems used.
pub const POPPED: &str = "Totems used";

/// The 14 independent widgets supported by the Basic HUD.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Widget {
    Effects,
    Items,
    Durability,
    Totems,
    Health,
    Cooldowns,
    Coordinates,
    Fps,
    Ping,
    Speed,
    Cps,
    Clock,
    ArmorMini,
    Pickups,
}

pub const WIDGETS: [Widget; 14] = [
    Widget::Effects,
    Widget::Items,
    Widget::Durability,
    Widget::Totems,
    Widget::Health,
    Widget::Cooldowns,
    Widget::Coordinates,
    Widget::Fps,
    Widget::Ping,
    Widget::Speed,
    Widget::Cps,
    Widget::Clock,
    Widget::ArmorMini,
    Widget::Pickups,
];

impl Widget {
    pub const fn index(self) -> usize {
        match self {
            Widget::Effects => 0,
            Widget::Items => 1,
            Widget::Durability => 2,
            Widget::Totems => 3,
            Widget::Health => 4,
            Widget::Cooldowns => 5,
            Widget::Coordinates => 6,
            Widget::Fps => 7,
            Widget::Ping => 8,
            Widget::Speed => 9,
            Widget::Cps => 10,
            Widget::Clock => 11,
            Widget::ArmorMini => 12,
            Widget::Pickups => 13,
        }
    }

    #[cfg(test)]
    pub const fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Widget::Effects),
            1 => Some(Widget::Items),
            2 => Some(Widget::Durability),
            3 => Some(Widget::Totems),
            4 => Some(Widget::Health),
            5 => Some(Widget::Cooldowns),
            6 => Some(Widget::Coordinates),
            7 => Some(Widget::Fps),
            8 => Some(Widget::Ping),
            9 => Some(Widget::Speed),
            10 => Some(Widget::Cps),
            11 => Some(Widget::Clock),
            12 => Some(Widget::ArmorMini),
            13 => Some(Widget::Pickups),
            _ => None,
        }
    }

    pub const fn key(self) -> &'static str {
        match self {
            Widget::Effects => "effects",
            Widget::Items => "items",
            Widget::Durability => "durability",
            Widget::Totems => "totems",
            Widget::Health => "health",
            Widget::Cooldowns => "cooldowns",
            Widget::Coordinates => "coordinates",
            Widget::Fps => "fps",
            Widget::Ping => "ping",
            Widget::Speed => "speed",
            Widget::Cps => "cps",
            Widget::Clock => "clock",
            Widget::ArmorMini => "armor_mini",
            Widget::Pickups => "pickups",
        }
    }

    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "effects" => Some(Widget::Effects),
            "items" => Some(Widget::Items),
            "durability" => Some(Widget::Durability),
            "totems" => Some(Widget::Totems),
            "health" => Some(Widget::Health),
            "cooldowns" => Some(Widget::Cooldowns),
            "coordinates" => Some(Widget::Coordinates),
            "fps" => Some(Widget::Fps),
            "ping" => Some(Widget::Ping),
            "speed" => Some(Widget::Speed),
            "cps" => Some(Widget::Cps),
            "clock" => Some(Widget::Clock),
            "armor_mini" | "armormini" => Some(Widget::ArmorMini),
            "pickups" => Some(Widget::Pickups),
            _ => None,
        }
    }

    pub fn title(self, ru: bool) -> &'static str {
        match (self, ru) {
            (Widget::Effects, true) => "Эффекты",
            (Widget::Effects, false) => "Effects",
            (Widget::Items, true) => "Расходники",
            (Widget::Items, false) => "Consumables",
            (Widget::Durability, true) => "Прочность",
            (Widget::Durability, false) => "Durability",
            (Widget::Totems, true) => "Счётчик тотемов",
            (Widget::Totems, false) => "Totems",
            (Widget::Health, true) => "Здоровье",
            (Widget::Health, false) => "Health",
            (Widget::Cooldowns, true) => "Кулдауны",
            (Widget::Cooldowns, false) => "Cooldowns",
            (Widget::Coordinates, true) => "Координаты",
            (Widget::Coordinates, false) => "Coordinates",
            (Widget::Fps, true) => "FPS",
            (Widget::Fps, false) => "FPS",
            (Widget::Ping, true) => "Пинг",
            (Widget::Ping, false) => "Ping",
            (Widget::Speed, true) => "Скорость",
            (Widget::Speed, false) => "Speed",
            (Widget::Cps, true) => "CPS",
            (Widget::Cps, false) => "CPS",
            (Widget::Clock, true) => "Часы",
            (Widget::Clock, false) => "Clock",
            (Widget::ArmorMini, true) => "Мини-броня",
            (Widget::ArmorMini, false) => "Mini Armor",
            (Widget::Pickups, true) => "Недавние подборы",
            (Widget::Pickups, false) => "Recent pickups",
        }
    }

    pub const fn supports_icons_only(self) -> bool {
        match self {
            Widget::Effects
            | Widget::Items
            | Widget::Durability
            | Widget::Totems
            | Widget::Cooldowns
            | Widget::ArmorMini
            | Widget::Pickups => true,
            _ => false,
        }
    }
}

/// Visual transform and styling options for a single widget.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WidgetConfig {
    pub enabled: bool,
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub text: u32,
    pub accent: u32,
    pub background: u32,
    pub opacity: u8,
    pub shadow: bool,
    pub padding: f32,
    pub compact: bool,
    pub icons: bool,
    pub icons_only: bool,
}

impl WidgetConfig {
    /// Automatic status colors; an explicitly chosen FFFFFF stays plain white.
    pub const DEFAULT_TEXT: u32 = 0xFFFF_FFFF;
    pub const DEFAULT_ACCENT: u32 = 0x70_C9_FF;
    pub const DEFAULT_BG: u32 = 0x10_15_1D;
    pub const DEFAULT_OPACITY: u8 = 60;
    pub const DEFAULT_SCALE: f32 = 1.0;
    pub const DEFAULT_PADDING: f32 = 3.0;

    pub const fn new(enabled: bool, x: f32, y: f32) -> Self {
        Self {
            enabled,
            x,
            y,
            scale: Self::DEFAULT_SCALE,
            text: Self::DEFAULT_TEXT,
            accent: Self::DEFAULT_ACCENT,
            background: Self::DEFAULT_BG,
            opacity: Self::DEFAULT_OPACITY,
            shadow: true,
            padding: Self::DEFAULT_PADDING,
            compact: false,
            icons: true,
            icons_only: false,
        }
    }

    pub fn line(&self) -> String {
        let text = if self.text == Self::DEFAULT_TEXT { "auto".to_owned() } else { format!("{:06X}", self.text & 0xFF_FFFF) };
        format!(
            "{},{},{},{},{},{:06X},{:06X},{},{},{},{},{},{}",
            u8::from(self.enabled),
            self.x,
            self.y,
            self.scale,
            text,
            self.accent & 0xFF_FFFF,
            self.background & 0xFF_FFFF,
            self.opacity,
            u8::from(self.shadow),
            self.padding,
            u8::from(self.compact),
            u8::from(self.icons),
            u8::from(self.icons_only),
        )
    }

    pub fn parse(s: &str, default: &WidgetConfig) -> WidgetConfig {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 12 && parts.len() != 13 {
            return *default;
        }

        let enabled = parts[0] != "0";
        let x = parts[1].parse::<f32>().ok().filter(|v| v.is_finite()).map(|v| v.clamp(0.0, 1.0)).unwrap_or(default.x);
        let y = parts[2].parse::<f32>().ok().filter(|v| v.is_finite()).map(|v| v.clamp(0.0, 1.0)).unwrap_or(default.y);
        let scale = parts[3].parse::<f32>().ok().filter(|v| v.is_finite()).map(|v| v.clamp(0.5, 3.0)).unwrap_or(default.scale);
        let text = if parts[4].eq_ignore_ascii_case("auto") { Self::DEFAULT_TEXT } else { u32::from_str_radix(parts[4], 16).map(|v| v & 0xFF_FFFF).unwrap_or(default.text) };
        let accent = u32::from_str_radix(parts[5], 16).map(|v| v & 0xFF_FFFF).unwrap_or(default.accent);
        let background = u32::from_str_radix(parts[6], 16).map(|v| v & 0xFF_FFFF).unwrap_or(default.background);
        let opacity = parts[7].parse::<u8>().map(|v| v.min(100)).unwrap_or(default.opacity);
        let shadow = parts[8] != "0";
        let padding = parts[9].parse::<f32>().ok().filter(|v| v.is_finite()).map(|v| v.clamp(0.0, 12.0)).unwrap_or(default.padding);
        let compact = parts[10] != "0";
        let icons = parts[11] != "0";
        let icons_only = if parts.len() >= 13 { parts[12] != "0" } else { false };

        WidgetConfig {
            enabled,
            x,
            y,
            scale,
            text,
            accent,
            background,
            opacity,
            shadow,
            padding,
            compact,
            icons,
            icons_only,
        }
    }
}

impl Default for WidgetConfig {
    fn default() -> Self {
        Self::new(true, 0.0, 0.0)
    }
}

/// A 2D rectangular hitbox in viewport/GUI coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn contains(&self, point: [f32; 2]) -> bool {
        let [px, py] = point;
        px >= self.x && px <= self.x + self.w && py >= self.y && py <= self.y + self.h
    }
}

/// Computes the exact on-screen render bounds / hitbox for a widget given its base size and viewport.
pub fn place(widget: &WidgetConfig, base_size: [f32; 2], viewport: [f32; 2]) -> Rect {
    let scale = if widget.scale.is_finite() {
        widget.scale.clamp(0.5, 3.0)
    } else {
        1.0
    };
    let w = (base_size[0] * scale).max(0.0);
    let h = (base_size[1] * scale).max(0.0);
    let free_x = (viewport[0] - w).max(0.0);
    let free_y = (viewport[1] - h).max(0.0);
    let norm_x = if widget.x.is_finite() {
        widget.x.clamp(0.0, 1.0)
    } else {
        0.0
    };
    let norm_y = if widget.y.is_finite() {
        widget.y.clamp(0.0, 1.0)
    } else {
        0.0
    };
    let x = norm_x * free_x;
    let y = norm_y * free_y;
    Rect { x, y, w, h }
}

/// Moves a widget to an absolute top-left coordinate, optionally snapping to 8 GUI px grid and edges/center.
pub fn move_to(
    widget: &mut WidgetConfig,
    at: [f32; 2],
    base_size: [f32; 2],
    viewport: [f32; 2],
    snap: bool,
) {
    let scale = if widget.scale.is_finite() {
        widget.scale.clamp(0.5, 3.0)
    } else {
        1.0
    };
    let w = (base_size[0] * scale).max(0.0);
    let h = (base_size[1] * scale).max(0.0);
    let free_x = (viewport[0] - w).max(0.0);
    let free_y = (viewport[1] - h).max(0.0);

    let raw_x = if at[0].is_finite() { at[0] } else { 0.0 };
    let raw_y = if at[1].is_finite() { at[1] } else { 0.0 };

    let snap_axis = |raw: f32, free: f32| -> f32 {
        if free <= 0.0 {
            return 0.0;
        }
        let anchors = [0.0, free / 2.0, free];
        let mut best_anchor = None;
        let mut min_dist = f32::INFINITY;
        for &a in &anchors {
            let dist = (raw - a).abs();
            if dist <= 8.0 && dist < min_dist {
                min_dist = dist;
                best_anchor = Some(a);
            }
        }
        if let Some(anchor) = best_anchor {
            anchor
        } else {
            ((raw / 8.0).round() * 8.0).clamp(0.0, free)
        }
    };

    let target_x = if snap {
        snap_axis(raw_x, free_x)
    } else {
        raw_x
    };
    let target_y = if snap {
        snap_axis(raw_y, free_y)
    } else {
        raw_y
    };

    let norm_x = if widget.x.is_finite() {
        widget.x.clamp(0.0, 1.0)
    } else {
        0.0
    };
    let norm_y = if widget.y.is_finite() {
        widget.y.clamp(0.0, 1.0)
    } else {
        0.0
    };

    widget.x = if free_x > 0.0 {
        (target_x / free_x).clamp(0.0, 1.0)
    } else {
        norm_x
    };
    widget.y = if free_y > 0.0 {
        (target_y / free_y).clamp(0.0, 1.0)
    } else {
        norm_y
    };
}

/// Returns the render bounds for a widget. For ArmorMini when anchored, returns hotbar-relative bounds.
pub fn widget_rect(config: &Config, widget: Widget, base_size: [f32; 2], viewport: [f32; 2]) -> Rect {
    config.widget_rect(widget, base_size, viewport)
}

/// Legacy equal-cell geometry, also kept for callers of the pure core API.
#[allow(dead_code)]
pub fn icon_grid(count: usize, icon_size: f32, padding: f32, max_width: f32) -> (usize, [f32; 2]) {
    if count == 0 {
        return (0, [0.0, 0.0]);
    }
    let icon_size = if icon_size.is_finite() { icon_size.max(1.0) } else { 10.0 };
    let cell = icon_size + 3.0;
    let pad = if padding.is_finite() { padding.max(0.0) } else { 0.0 };
    let available = if max_width.is_finite() { (max_width - pad * 2.0 + 3.0).max(cell) } else { cell };
    let cols = ((available / cell).floor() as usize).max(1).min(count);
    let rows = count.div_ceil(cols);
    let w = pad * 2.0 + cols as f32 * cell - 3.0;
    let h = pad * 2.0 + (rows as f32 * cell - 3.0).max(0.0);
    (cols, [w, h])
}

/// Measure icon+value cells; widths already include each icon, gap and readout.
pub fn icon_value_layout(widths: &[f32], height: f32, padding: f32, max_width: f32) -> (Vec<[f32; 2]>, [f32; 2]) {
    if widths.is_empty() { return (Vec::new(), [0.0, 0.0]); }
    let height = if height.is_finite() { height.max(1.0) } else { 10.0 };
    let pad = if padding.is_finite() { padding.clamp(0.0, 12.0) } else { 0.0 };
    let available = if max_width.is_finite() { (max_width - pad * 2.0).max(1.0) } else { 120.0 };
    let mut positions = Vec::with_capacity(widths.len());
    let (mut x, mut y, mut widest) = (0.0_f32, 0.0_f32, 0.0_f32);
    let mut occupied = false;
    for width in widths {
        let width = if width.is_finite() { width.max(0.0) } else { 0.0 };
        if occupied && x + 3.0 + width > available {
            x = 0.0;
            y += height + 3.0;
            occupied = false;
        }
        if occupied { x += 3.0; }
        positions.push([pad + x, pad + y]);
        x += width;
        widest = widest.max(x);
        occupied = true;
    }
    (positions, [widest + pad * 2.0, y + height + pad * 2.0])
}

pub fn icon_count(count: u32) -> String { format!("×{count}") }

pub fn should_show_guide(marker: Option<&str>, legacy_seen: bool) -> bool {
    marker != Some("1") && !legacy_seen
}

/// Persist onboarding independently from the editable visual draft.
pub fn mark_guide_seen(get: impl FnOnce() -> Option<String>, set: impl FnOnce(&str)) -> bool {
    set("1");
    get().is_some_and(|stored| stored.trim() == "1")
}

/// Onboarding is persisted separately; only editable preferences need saving.
pub fn should_confirm_exit(draft: &Config, baseline: &Config) -> bool {
    let mut preferences = draft.clone();
    preferences.tutorial_seen = baseline.tutorial_seen;
    &preferences != baseline
}

/// Target scope for resetting HUD configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResetTarget {
    All,
    Widget(Widget),
    Tab,
}

/// Tab list overlay configuration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TabConfig {
    pub enabled: bool,
    pub scale: f32,
    pub opacity: u8,
    pub numeric_ping: bool,
    pub ping_bars: bool,
    pub highlight_self: bool,
    pub show_count: bool,
    pub sort: u8,
}

impl TabConfig {
    pub const DEFAULT_SCALE: f32 = 1.0;
    pub const DEFAULT_OPACITY: u8 = 75;

    pub fn line(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{}",
            u8::from(self.enabled),
            self.scale,
            self.opacity,
            u8::from(self.numeric_ping),
            u8::from(self.ping_bars),
            u8::from(self.highlight_self),
            u8::from(self.show_count),
            self.sort,
        )
    }

    pub fn parse(s: &str, default: &TabConfig) -> TabConfig {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 8 {
            return *default;
        }

        let enabled = parts[0] != "0";
        let scale = parts[1]
            .parse::<f32>()
            .ok()
            .filter(|v| v.is_finite())
            .map(|v| v.clamp(0.5, 1.5))
            .unwrap_or(default.scale);
        let opacity = parts[2]
            .parse::<u8>()
            .map(|v| v.min(100))
            .unwrap_or(default.opacity);
        let numeric_ping = parts[3] != "0";
        let ping_bars = parts[4] != "0";
        let highlight_self = parts[5] != "0";
        let show_count = parts[6] != "0";
        let sort = parts[7]
            .parse::<u8>()
            .map(|v| v.min(2))
            .unwrap_or(default.sort);

        TabConfig {
            enabled,
            scale,
            opacity,
            numeric_ping,
            ping_bars,
            highlight_self,
            show_count,
            sort,
        }
    }
}

impl Default for TabConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            scale: Self::DEFAULT_SCALE,
            opacity: Self::DEFAULT_OPACITY,
            numeric_ping: true,
            ping_bars: true,
            highlight_self: true,
            show_count: true,
            sort: 0,
        }
    }
}

/// An effect the settings can pin.
pub struct Known {
    pub id: &'static str,
    /// Comes from a brewed potion, so it is pinned until the player says not.
    pub potion: bool,
    pub harmful: bool,
}

const fn known(id: &'static str, potion: bool, harmful: bool) -> Known {
    Known { id, potion, harmful }
}

/// Instant health and damage are left out: they have no time to show.
pub const EFFECTS: [Known; 34] = [
    known("speed", true, false),
    known("slowness", true, true),
    known("strength", true, false),
    known("jump_boost", true, false),
    known("regeneration", true, false),
    known("resistance", true, false),
    known("fire_resistance", true, false),
    known("water_breathing", true, false),
    known("invisibility", true, false),
    known("night_vision", true, false),
    known("poison", true, true),
    known("weakness", true, true),
    known("slow_falling", true, false),
    known("luck", true, false),
    known("wind_charged", true, true),
    known("weaving", true, true),
    known("oozing", true, true),
    known("infested", true, true),
    known("haste", false, false),
    known("mining_fatigue", false, true),
    known("nausea", false, true),
    known("blindness", false, true),
    known("hunger", false, true),
    known("saturation", false, false),
    known("wither", false, true),
    known("health_boost", false, false),
    known("absorption", false, false),
    known("glowing", false, false),
    known("levitation", false, true),
    known("unluck", false, true),
    known("conduit_power", false, false),
    known("dolphins_grace", false, false),
    known("darkness", false, true),
    known("bad_omen", false, true),
];

/// What the player chose, kept as one line in the `config` setting.
#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub language: String,
    pub effects: bool,
    pub items: bool,
    pub durability: bool,
    pub totems: bool,
    /// Every effect on the player, not only the pinned ones.
    pub all_effects: bool,
    /// Index into [`CORNERS`].
    pub corner: usize,
    pub pinned: BTreeSet<String>,
    pub widgets: [WidgetConfig; 14],
    pub snap: bool,
    pub health_tags: bool,
    pub fps_lows: bool,
    pub fps_label: bool,
    pub fps_auto: bool,
    pub mini_anchor: bool,
    pub mini_numbers: bool,
    /// Seconds since the last pickup of this item, clamped to 1..=60.
    pub pickup_seconds: u32,
    pub tutorial_seen: bool,
    pub tab: TabConfig,
    pub unknown: Vec<(String, String)>,
}

pub const CORNERS: [&str; 4] = ["top left", "top right", "bottom left", "bottom right"];

fn corner_positions(corner: usize) -> [(f32, f32); 4] {
    match corner % 4 {
        0 => [(0.01, 0.26), (0.01, 0.40), (0.01, 0.63), (0.01, 0.86)],
        1 => [(0.99, 0.02), (0.99, 0.12), (0.99, 0.22), (0.99, 0.32)],
        2 => [(0.01, 0.60), (0.01, 0.70), (0.01, 0.80), (0.01, 0.90)],
        _ => [(0.99, 0.60), (0.99, 0.70), (0.99, 0.80), (0.99, 0.90)],
    }
}

pub fn default_widgets(corner: usize) -> [WidgetConfig; 14] {
    let cp = corner_positions(corner);
    let ping_x = if corner % 2 == 0 { 0.99 } else { 0.01 };
    let ping_y = if corner % 2 == 0 { 0.02 } else { 0.17 };
    let speed_y = if corner % 2 == 0 { 0.07 } else { 0.24 };
    [
        // 0: Effects
        WidgetConfig::new(true, cp[0].0, cp[0].1),
        // 1: Items
        WidgetConfig::new(true, cp[1].0, cp[1].1),
        // 2: Durability
        WidgetConfig::new(true, cp[2].0, cp[2].1),
        // 3: Totems
        WidgetConfig::new(true, cp[3].0, cp[3].1),
        // 4: Health
        WidgetConfig::new(true, 0.50, 0.72),
        // 5: Cooldowns
        WidgetConfig::new(true, 0.50, 0.80),
        // 6: Coordinates (below FPS)
        WidgetConfig::new(true, 0.01, 0.12),
        // 7: Fps (top-left)
        WidgetConfig::new(true, 0.01, 0.02),
        // 8: Ping
        WidgetConfig::new(true, ping_x, ping_y),
        // 9: Speed (off by default)
        WidgetConfig::new(false, ping_x, speed_y),
        // 10: Cps (optional off)
        WidgetConfig::new(false, 0.01, 0.75),
        // 11: Clock (optional off)
        WidgetConfig::new(false, 0.50, 0.02),
        // 12: ArmorMini (anchored by default)
        WidgetConfig::new(true, 0.55, 0.90),
        // 13: Recent pickups (right side, clear of the default ping).
        WidgetConfig::new(true, 0.99, 0.40),
    ]
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: "auto".to_owned(),
            effects: true,
            items: true,
            durability: true,
            totems: true,
            all_effects: false,
            corner: 0,
            pinned: EFFECTS.iter().filter(|e| e.potion).map(|e| e.id.to_owned()).collect(),
            widgets: default_widgets(0),
            snap: true,
            health_tags: true,
            fps_lows: false,
            fps_label: true,
            fps_auto: true,
            mini_anchor: true,
            mini_numbers: false,
            pickup_seconds: 5,
            tutorial_seen: false,
            tab: TabConfig::default(),
            unknown: Vec::new(),
        }
    }
}

impl Config {
    pub fn resolve_locale(code: &str) -> &'static str {
        let lower = code.trim().to_ascii_lowercase().replace('-', "_");
        match lower.as_str() {
            "ru" | "ru_ru" => "ru_ru",
            "uk" | "uk_ua" => "uk_ua",
            "de" | "de_de" => "de_de",
            "fr" | "fr_fr" => "fr_fr",
            "es" | "es_es" => "es_es",
            "pt" | "pt_br" | "pt_pt" => "pt_br",
            "pl" | "pl_pl" => "pl_pl",
            "tr" | "tr_tr" => "tr_tr",
            "it" | "it_it" => "it_it",
            "en" | "en_us" | "en_gb" | "en_ca" | "en_au" => "en_us",
            s if s.starts_with("ru") => "ru_ru",
            s if s.starts_with("uk") => "uk_ua",
            s if s.starts_with("de") => "de_de",
            s if s.starts_with("fr") => "fr_fr",
            s if s.starts_with("es") => "es_es",
            s if s.starts_with("pt") => "pt_br",
            s if s.starts_with("pl") => "pl_pl",
            s if s.starts_with("tr") => "tr_tr",
            s if s.starts_with("it") => "it_it",
            _ => "en_us",
        }
    }

    pub fn language<'a>(&'a self, game: &'a str) -> &'a str {
        if self.language == "auto" {
            game
        } else {
            self.language.as_str()
        }
    }

    pub fn reset_target(&mut self, target: ResetTarget) {
        match target {
            ResetTarget::All => {
                let saved_language = std::mem::take(&mut self.language);
                let saved_unknown = std::mem::take(&mut self.unknown);
                *self = Config::default();
                self.language = saved_language;
                self.unknown = saved_unknown;
            }
            ResetTarget::Widget(widget) => {
                self.reset_widget(widget);
            }
            ResetTarget::Tab => {
                self.tab = TabConfig::default();
            }
        }
    }

    pub fn widget(&self, widget: Widget) -> &WidgetConfig {
        &self.widgets[widget.index()]
    }

    pub fn widget_mut(&mut self, widget: Widget) -> &mut WidgetConfig {
        &mut self.widgets[widget.index()]
    }

    pub fn widget_rect(&self, widget: Widget, base_size: [f32; 2], viewport: [f32; 2]) -> Rect {
        let wcfg = self.widget(widget);
        if widget == Widget::ArmorMini && self.mini_anchor {
            let scale = if wcfg.scale.is_finite() {
                wcfg.scale.clamp(0.5, 3.0)
            } else {
                1.0
            };
            let w = (base_size[0] * scale).max(0.0);
            let h = (base_size[1] * scale).max(0.0);
            let free_x = (viewport[0] - w).max(0.0);
            let free_y = (viewport[1] - h).max(0.0);
            let x = (viewport[0] * 0.5 + 98.0).clamp(0.0, free_x);
            let y = (viewport[1] - h - 4.0).clamp(0.0, free_y);
            Rect { x, y, w, h }
        } else {
            place(wcfg, base_size, viewport)
        }
    }

    pub fn move_widget(
        &mut self,
        widget: Widget,
        at: [f32; 2],
        base_size: [f32; 2],
        viewport: [f32; 2],
        snap: bool,
    ) {
        if widget == Widget::ArmorMini {
            self.mini_anchor = false;
        }
        move_to(&mut self.widgets[widget.index()], at, base_size, viewport, snap);
    }

    pub fn reset_widget(&mut self, widget: Widget) {
        let def = Config::default();
        self.widgets[widget.index()] = def.widgets[widget.index()];
        self.set_enabled(widget, def.enabled(widget));
        if widget == Widget::ArmorMini {
            self.mini_anchor = true;
            self.mini_numbers = false;
        } else if widget == Widget::Pickups {
            self.pickup_seconds = def.pickup_seconds;
        } else if widget == Widget::Fps {
            self.fps_label = def.fps_label;
            self.fps_lows = def.fps_lows;
            self.fps_auto = def.fps_auto;
        }
    }

    pub fn enabled(&self, widget: Widget) -> bool {
        match widget {
            Widget::Effects => self.effects && self.widgets[0].enabled,
            Widget::Items => self.items && self.widgets[1].enabled,
            Widget::Durability => self.durability && self.widgets[2].enabled,
            Widget::Totems => self.totems && self.widgets[3].enabled,
            _ => self.widgets[widget.index()].enabled,
        }
    }

    pub fn set_enabled(&mut self, widget: Widget, on: bool) {
        self.widgets[widget.index()].enabled = on;
        match widget {
            Widget::Effects => self.effects = on,
            Widget::Items => self.items = on,
            Widget::Durability => self.durability = on,
            Widget::Totems => self.totems = on,
            _ => {}
        }
    }

    /// `key=value` pairs by `;`; a missing or broken one keeps its default.
    pub fn parse(stored: Option<&str>) -> Self {
        let mut config = Self::default();
        let Some(stored) = stored else {
            return config;
        };

        let mut explicit_widgets = [false; 14];
        let mut parsed_corner = None;
        let mut version = 1;

        for pair in stored.split(';') {
            let pair = pair.trim();
            if pair.is_empty() {
                continue;
            }
            let Some((key, value)) = pair.split_once('=') else {
                continue;
            };
            let key = key.trim();
            let value = value.trim();
            if key.is_empty() {
                continue;
            }

            match key {
                "v" => {
                    if let Ok(v) = value.parse::<u32>() {
                        version = v;
                    }
                }
                "language" => {
                    let lower = value.trim().to_ascii_lowercase().replace('-', "_");
                    config.language = match lower.as_str() {
                        "auto" => "auto".to_owned(),
                        "en_us" | "ru_ru" | "uk_ua" | "de_de" | "fr_fr" | "es_es" | "pt_br" | "pl_pl" | "tr_tr" | "it_it" => lower,
                        "ru" | "uk" | "de" | "fr" | "es" | "pt" | "pl" | "tr" | "it" | "en" => Self::resolve_locale(&lower).to_owned(),
                        _ => "auto".to_owned(),
                    };
                }
                "effects" => {
                    let on = value != "0";
                    config.effects = on;
                    if !explicit_widgets[0] {
                        config.widgets[0].enabled = on;
                    }
                }
                "items" => {
                    let on = value != "0";
                    config.items = on;
                    if !explicit_widgets[1] {
                        config.widgets[1].enabled = on;
                    }
                }
                "durability" => {
                    let on = value != "0";
                    config.durability = on;
                    if !explicit_widgets[2] {
                        config.widgets[2].enabled = on;
                    }
                }
                "totems" => {
                    let on = value != "0";
                    config.totems = on;
                    if !explicit_widgets[3] {
                        config.widgets[3].enabled = on;
                    }
                }
                "all" => config.all_effects = value != "0",
                "corner" => {
                    if let Ok(c) = value.parse::<usize>() {
                        let c = c % CORNERS.len();
                        config.corner = c;
                        parsed_corner = Some(c);
                    }
                }
                "pinned" => {
                    config.pinned = value.split(',').filter(|i| !i.is_empty()).map(str::to_owned).collect();
                }
                "snap" => config.snap = value != "0",
                "health_tags" => config.health_tags = value != "0",
                "fps_lows" => config.fps_lows = value != "0",
                "fps_label" => config.fps_label = value != "0",
                "fps_auto" => config.fps_auto = value != "0",
                "mini_anchor" => config.mini_anchor = value != "0",
                "mini_numbers" => config.mini_numbers = value != "0",
                "pickup_seconds" => {
                    if let Ok(seconds) = value.parse::<u32>() {
                        config.pickup_seconds = seconds.clamp(1, 60);
                    }
                }
                "tutorial_seen" => config.tutorial_seen = value != "0",
                "tab" => {
                    config.tab = TabConfig::parse(value, &config.tab);
                }
                w_key if w_key.starts_with("w.") => {
                    let name = &w_key[2..];
                    if let Some(widget) = Widget::from_key(name) {
                        let idx = widget.index();
                        let parsed = WidgetConfig::parse(value, &config.widgets[idx]);
                        config.widgets[idx] = parsed;
                        explicit_widgets[idx] = true;
                        match widget {
                            Widget::Effects => config.effects = parsed.enabled,
                            Widget::Items => config.items = parsed.enabled,
                            Widget::Durability => config.durability = parsed.enabled,
                            Widget::Totems => config.totems = parsed.enabled,
                            _ => {}
                        }
                    } else if !value.is_empty() {
                        if let Some(entry) = config.unknown.iter_mut().find(|(k, _)| k == key) {
                            entry.1 = value.to_owned();
                        } else {
                            config.unknown.push((key.to_owned(), value.to_owned()));
                        }
                    }
                }
                _ => {
                    if !value.is_empty() {
                        if let Some(entry) = config.unknown.iter_mut().find(|(k, _)| k == key) {
                            entry.1 = value.to_owned();
                        } else {
                            config.unknown.push((key.to_owned(), value.to_owned()));
                        }
                    }
                }
            }
        }

        // Explicit v2 widget configs take precedence over legacy booleans
        if explicit_widgets[0] {
            config.effects = config.widgets[0].enabled;
        }
        if explicit_widgets[1] {
            config.items = config.widgets[1].enabled;
        }
        if explicit_widgets[2] {
            config.durability = config.widgets[2].enabled;
        }
        if explicit_widgets[3] {
            config.totems = config.widgets[3].enabled;
        }

        // Migrate corner positions for legacy configs without explicit widgets
        if let Some(c) = parsed_corner {
            let defaults = default_widgets(c);
            for i in 0..WIDGETS.len() {
                if !explicit_widgets[i] {
                    config.widgets[i].x = defaults[i].x;
                    config.widgets[i].y = defaults[i].y;
                }
            }
        }

        // Migrate untouched v2 defaults to v3 while preserving customized positions
        if version < 3 {
            let old_telem_x = if config.corner % 2 == 0 { 0.99 } else { 0.01 };
            // Untouched speed default was (old_telem_x, 0.31)
            let w_speed = &mut config.widgets[Widget::Speed.index()];
            if (w_speed.x - old_telem_x).abs() < 1e-3 && (w_speed.y - 0.31).abs() < 1e-3 {
                w_speed.enabled = false;
                w_speed.x = if config.corner % 2 == 0 { 0.99 } else { 0.01 };
                w_speed.y = if config.corner % 2 == 0 { 0.07 } else { 0.24 };
            }

            // Untouched fps default was (old_telem_x, 0.15)
            let w_fps = &mut config.widgets[Widget::Fps.index()];
            if (w_fps.x - old_telem_x).abs() < 1e-3 && (w_fps.y - 0.15).abs() < 1e-3 {
                w_fps.x = 0.01;
                w_fps.y = 0.02;
                config.fps_lows = false;
            }

            // Untouched coordinates default was (old_telem_x, 0.02)
            let w_coord = &mut config.widgets[Widget::Coordinates.index()];
            if (w_coord.x - old_telem_x).abs() < 1e-3 && (w_coord.y - 0.02).abs() < 1e-3 {
                w_coord.x = 0.01;
                w_coord.y = 0.12;
            }

            // Untouched corner 0 effects column: positions were 0.02, 0.12, 0.22, 0.32 at x = 0.01
            if config.corner == 0
                && (config.widgets[0].x - 0.01).abs() < 1e-3
                && (config.widgets[0].y - 0.02).abs() < 1e-3
                && (config.widgets[1].x - 0.01).abs() < 1e-3
                && (config.widgets[1].y - 0.12).abs() < 1e-3
                && (config.widgets[2].x - 0.01).abs() < 1e-3
                && (config.widgets[2].y - 0.22).abs() < 1e-3
                && (config.widgets[3].x - 0.01).abs() < 1e-3
                && (config.widgets[3].y - 0.32).abs() < 1e-3
            {
                config.widgets[0].y = 0.26;
                config.widgets[1].y = 0.40;
                config.widgets[2].y = 0.63;
                config.widgets[3].y = 0.86;
            }
        }

        config
    }

    pub fn line(&self) -> String {
        let pinned: Vec<&str> = self.pinned.iter().map(String::as_str).collect();
        let mut parts = vec![
            "v=4".to_owned(),
            format!("language={}", self.language),
            format!("effects={}", u8::from(self.effects)),
            format!("items={}", u8::from(self.items)),
            format!("durability={}", u8::from(self.durability)),
            format!("totems={}", u8::from(self.totems)),
            format!("all={}", u8::from(self.all_effects)),
            format!("corner={}", self.corner),
            format!("pinned={}", pinned.join(",")),
            format!("snap={}", u8::from(self.snap)),
            format!("health_tags={}", u8::from(self.health_tags)),
            format!("fps_lows={}", u8::from(self.fps_lows)),
            format!("fps_label={}", u8::from(self.fps_label)),
            format!("fps_auto={}", u8::from(self.fps_auto)),
            format!("mini_anchor={}", u8::from(self.mini_anchor)),
            format!("mini_numbers={}", u8::from(self.mini_numbers)),
            format!("pickup_seconds={}", self.pickup_seconds.clamp(1, 60)),
            format!("tutorial_seen={}", u8::from(self.tutorial_seen)),
            format!("tab={}", self.tab.line()),
        ];
        for widget in WIDGETS {
            let mut cfg = self.widgets[widget.index()];
            cfg.enabled = self.enabled(widget);
            parts.push(format!("w.{}={}", widget.key(), cfg.line()));
        }
        for (k, v) in &self.unknown {
            parts.push(format!("{k}={v}"));
        }
        parts.join(";")
    }

    /// Whether an active effect belongs on the HUD. Beacons and conduits
    /// (`ambient`) are not potions, whatever effect they give.
    pub fn shows_effect(&self, id: &str, ambient: bool) -> bool {
        self.all_effects || (!ambient && self.pinned.contains(id))
    }

    /// The click on settings button `button`, in the order of [`labels`].
    pub fn click(&mut self, button: usize) {
        match button {
            0 => self.set_enabled(Widget::Effects, !self.enabled(Widget::Effects)),
            1 => self.all_effects = !self.all_effects,
            2 => self.set_enabled(Widget::Items, !self.enabled(Widget::Items)),
            3 => self.set_enabled(Widget::Durability, !self.enabled(Widget::Durability)),
            4 => self.set_enabled(Widget::Totems, !self.enabled(Widget::Totems)),
            5 => {
                self.corner = (self.corner + 1) % CORNERS.len();
                let cp = corner_positions(self.corner);
                for i in 0..4 {
                    self.widgets[i].x = cp[i].0;
                    self.widgets[i].y = cp[i].1;
                }
            }
            n => {
                if let Some(e) = EFFECTS.get(n - 6) {
                    if !self.pinned.remove(e.id) {
                        self.pinned.insert(e.id.to_owned());
                    }
                }
            }
        }
    }
}

fn switch(on: bool) -> &'static str {
    if on { "ON" } else { "OFF" }
}

/// The settings page: `name` gives an effect's name in the player's language.
pub fn labels(config: &Config, name: impl Fn(&str) -> String) -> Vec<String> {
    let mut labels = vec![
        format!("Effects: {}", switch(config.enabled(Widget::Effects))),
        format!("Unpinned effects too: {}", switch(config.all_effects)),
        format!("Potions, apples, totems: {}", switch(config.enabled(Widget::Items))),
        format!("Durability: {}", switch(config.enabled(Widget::Durability))),
        format!("Totems used: {}", switch(config.enabled(Widget::Totems))),
        format!("Corner: {}", CORNERS[config.corner % CORNERS.len()]),
    ];
    labels.extend(EFFECTS.iter().map(|e| {
        format!("{}: {}", name(e.id), if config.pinned.contains(e.id) { "pinned" } else { "-" })
    }));
    labels
}

/// Time left as the game's own effect list writes it: `m:ss`, `h:mm:ss`,
/// and `**:**` for an endless effect (a duration of -1).
pub fn duration(ticks: i32) -> String {
    if ticks < 0 {
        return "**:**".to_owned();
    }
    let seconds = (ticks as i64 + 19) / 20;
    let (hours, minutes, seconds) = (seconds / 3600, seconds / 60 % 60, seconds % 60);
    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

pub fn harmful(id: &str) -> bool {
    EFFECTS.iter().any(|e| e.id == id && e.harmful)
}

pub fn effect_colour(id: &str, ticks: i32) -> u32 {
    if (0..ENDING).contains(&ticks) {
        WARNING
    } else if harmful(id) {
        BAD
    } else {
        GOOD
    }
}

/// Green while it is fine, then amber, then red under a tenth.
pub fn durability_colour(left: u32, max: u32) -> u32 {
    match left.saturating_mul(100) / max.max(1) {
        0..=10 => BAD,
        11..=35 => WARNING,
        _ => GOOD,
    }
}

pub fn durability_text(left: u32, max: u32) -> String {
    format!("{left}/{max}")
}

/// Counts of the same `key`, in the order of each key's first stack.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Tally<K> {
    pub rows: Vec<(K, u32)>,
}

impl<K: PartialEq> Tally<K> {
    pub fn add(&mut self, key: K, count: u32) {
        match self.rows.iter_mut().find(|(k, _)| *k == key) {
            Some((_, n)) => *n += count,
            None => self.rows.push((key, count)),
        }
    }
}

/// Totems each player has used, by name. A player keeps their count until
/// they die or this player leaves the world.
#[derive(Debug, Default)]
pub struct Pops {
    by_name: HashMap<String, u32>,
}

impl Pops {
    pub fn add(&mut self, name: &str) {
        *self.by_name.entry(name.to_owned()).or_default() += 1;
    }
    pub fn get(&self, name: &str) -> u32 {
        self.by_name.get(name).copied().unwrap_or(0)
    }
    pub fn forget(&mut self, name: &str) {
        self.by_name.remove(name);
    }
    pub fn clear(&mut self) {
        self.by_name.clear();
    }
}

thread_local! {
    static CONFIG: RefCell<Option<Config>> = const { RefCell::new(None) };
    static POPS: RefCell<Pops> = RefCell::new(Pops::default());
}

/// Updates the cached live configuration directly.
pub fn install(config: Config) {
    CONFIG.with(|c| {
        *c.borrow_mut() = Some(config);
    });
}

/// Serializes draft, writes via `set`, reads back via `get`, and verifies exact match.
/// Installs the draft into cached live config only on verified readback success.
pub fn commit_verified(
    draft: &Config,
    get: impl FnOnce() -> Option<String>,
    set: impl FnOnce(&str),
) -> bool {
    let serialized = draft.line();
    set(&serialized);
    if let Some(readback) = get() {
        if readback == serialized {
            install(draft.clone());
            return true;
        }
    }
    false
}

/// The settings, read once through `get` (the `config` setting).
pub fn config<R>(get: impl FnOnce() -> Option<String>, read: impl FnOnce(&Config) -> R) -> R {
    CONFIG.with(|c| {
        let mut c = c.borrow_mut();
        read(c.get_or_insert_with(|| Config::parse(get().as_deref())))
    })
}

/// Changes the settings; the line to keep.
#[allow(dead_code)] // Legacy helper; the visual editor uses commit_verified instead.
pub fn change(get: impl FnOnce() -> Option<String>, write: impl FnOnce(&mut Config)) -> String {
    CONFIG.with(|c| {
        let mut c = c.borrow_mut();
        let config = c.get_or_insert_with(|| Config::parse(get().as_deref()));
        write(config);
        config.line()
    })
}

pub fn pops<R>(read: impl FnOnce(&mut Pops) -> R) -> R {
    POPS.with(|p| read(&mut p.borrow_mut()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn potions_are_pinned_by_default_and_others_are_not() {
        let c = Config::default();
        assert!(c.shows_effect("strength", false));
        assert!(c.shows_effect("fire_resistance", false));
        assert!(!c.shows_effect("haste", false));
        assert!(!c.shows_effect("bad_omen", false));
    }

    #[test]
    fn a_beacon_effect_is_not_a_potion() {
        let mut c = Config::default();
        assert!(!c.shows_effect("speed", true));
        c.all_effects = true;
        assert!(c.shows_effect("speed", true));
        assert!(c.shows_effect("some_mod:effect", false));
    }

    #[test]
    fn durations_read_like_the_game() {
        assert_eq!(duration(-1), "**:**");
        assert_eq!(duration(0), "0:00");
        assert_eq!(duration(1), "0:01");
        assert_eq!(duration(20), "0:01");
        assert_eq!(duration(3600), "3:00");
        assert_eq!(duration(9600), "8:00");
        assert_eq!(duration(72_000 + 1200 + 20), "1:01:01");
        assert_eq!(duration(i32::MAX), "29826:09:43");
    }

    #[test]
    fn the_config_survives_its_line() {
        let mut c = Config::default();
        c.click(1);
        c.click(5);
        c.click(6);
        c.click(6 + 18);
        assert!(!c.pinned.contains("speed"));
        assert!(c.pinned.contains("haste"));
        assert_eq!(Config::parse(Some(&c.line())), c);
    }

    #[test]
    fn an_empty_pin_list_stays_empty() {
        let mut c = Config::default();
        c.pinned.clear();
        assert!(Config::parse(Some(&c.line())).pinned.is_empty());
    }

    #[test]
    fn a_broken_line_keeps_the_defaults() {
        assert_eq!(Config::parse(Some("nonsense;corner=x;=")), Config::default());
        assert_eq!(Config::parse(None), Config::default());
        assert_eq!(Config::parse(Some("corner=7")).corner, 3);
    }

    #[test]
    fn every_label_is_a_button_the_click_understands() {
        let c = Config::default();
        let labels = labels(&c, |id| id.to_owned());
        assert_eq!(labels.len(), 6 + EFFECTS.len());
        assert_eq!(labels[6], "speed: pinned");
        assert_eq!(labels[6 + 18], "haste: -");
        let mut c = c;
        c.click(labels.len());
        assert_eq!(c, Config::default());

        let mut extreme = Config::default();
        extreme.corner = usize::MAX;
        let ext_labels = super::labels(&extreme, str::to_owned);
        assert!(ext_labels[5].ends_with("bottom right"));
    }

    #[test]
    fn effect_ids_are_unique() {
        let ids: BTreeSet<_> = EFFECTS.iter().map(|e| e.id).collect();
        assert_eq!(ids.len(), EFFECTS.len());
    }

    #[test]
    fn colours_warn_near_the_end() {
        assert_eq!(effect_colour("speed", 1000), GOOD);
        assert_eq!(effect_colour("poison", 1000), BAD);
        assert_eq!(effect_colour("speed", 100), WARNING);
        assert_eq!(effect_colour("speed", -1), GOOD);
        assert_eq!(durability_colour(432, 432), GOOD);
        assert_eq!(durability_colour(100, 432), WARNING);
        assert_eq!(durability_colour(10, 432), BAD);
        assert_eq!(durability_colour(0, 0), BAD);
    }

    #[test]
    fn stacks_of_one_kind_add_up() {
        let mut t = Tally::default();
        t.add("a", 1);
        t.add("b", 16);
        t.add("a", 2);
        assert_eq!(t.rows, vec![("a", 3), ("b", 16)]);
    }

    #[test]
    fn pops_count_per_player_until_forgotten() {
        let mut p = Pops::default();
        p.add("Alex");
        p.add("Alex");
        p.add("Steve");
        assert_eq!((p.get("Alex"), p.get("Steve"), p.get("Kai")), (2, 1, 0));
        p.forget("Alex");
        assert_eq!(p.get("Alex"), 0);
        p.clear();
        assert_eq!(p.get("Steve"), 0);
    }

    #[test]
    fn finite_invalid_parsers_restore_bounded_defaults() {
        let def = WidgetConfig::new(true, 0.1, 0.2);
        // Nan / Inf / out-of-range inputs
        let parsed = WidgetConfig::parse("1,NaN,inf,100,INVALID,NOTHEX,WRONG,255,1,999,0,1", &def);
        assert_eq!(parsed.x, def.x);
        assert_eq!(parsed.y, def.y);
        assert_eq!(parsed.scale, 3.0); // clamped to upper bound
        assert_eq!(parsed.text, def.text);
        assert_eq!(parsed.accent, def.accent);
        assert_eq!(parsed.background, def.background);
        assert_eq!(parsed.opacity, 100); // clamped to 100
        assert_eq!(parsed.padding, 12.0); // clamped to 12.0

        // Negative values
        let neg = WidgetConfig::parse("0,-5.0,-1.0,0.1,123456,ABCDEF,000000,50,0,-2.0,1,0", &def);
        assert_eq!(neg.x, 0.0);
        assert_eq!(neg.y, 0.0);
        assert_eq!(neg.scale, 0.5); // clamped to lower bound 0.5
        assert_eq!(neg.padding, 0.0); // clamped to 0.0
        assert_eq!(neg.text, 0x123456);
        assert_eq!(neg.accent, 0xABCDEF);
        assert_eq!(neg.background, 0x000000);

        // Broken tokens / wrong count
        let broken = WidgetConfig::parse("1,2,3", &def);
        assert_eq!(broken, def);

        // Entire config parsing with corrupt widget line
        let conf = Config::parse(Some(";;==;w.fps=junk;w.health=1,2"));
        assert_eq!(conf.widget(Widget::Fps), &Config::default().widgets[Widget::Fps.index()]);
    }

    #[test]
    fn all_four_corners_migration_and_staggering() {
        for corner in 0..4 {
            let conf = Config::parse(Some(&format!("corner={corner}")));
            assert_eq!(conf.corner, corner);
            let w0 = conf.widget(Widget::Effects);
            let w1 = conf.widget(Widget::Items);
            let w2 = conf.widget(Widget::Durability);
            let w3 = conf.widget(Widget::Totems);

            match corner {
                0 => {
                    assert!(w0.x < 0.5 && w0.y < 0.5);
                    assert!(w0.y < w1.y && w1.y < w2.y && w2.y < w3.y);
                }
                1 => {
                    assert!(w0.x > 0.5 && w0.y < 0.5);
                    assert!(w0.y < w1.y && w1.y < w2.y && w2.y < w3.y);
                }
                2 => {
                    assert!(w0.x < 0.5 && w0.y > 0.5);
                    assert!(w0.y < w1.y && w1.y < w2.y && w2.y < w3.y);
                }
                3 => {
                    assert!(w0.x > 0.5 && w0.y > 0.5);
                    assert!(w0.y < w1.y && w1.y < w2.y && w2.y < w3.y);
                }
                _ => unreachable!(),
            }

            let saved = conf.line();
            let parsed_back = Config::parse(Some(&saved));
            assert_eq!(parsed_back, conf);
        }
    }

    #[test]
    fn roundtrip_preserves_all_widget_customizations() {
        let mut conf = Config::default();
        conf.snap = false;
        conf.health_tags = false;
        conf.fps_lows = false;

        let w_coord = conf.widget_mut(Widget::Coordinates);
        w_coord.scale = 1.8;
        w_coord.text = 0xAABBCC;
        w_coord.accent = 0x112233;
        w_coord.background = 0x445566;
        w_coord.opacity = 80;
        w_coord.padding = 5.0;
        w_coord.compact = true;
        w_coord.x = 0.45;
        w_coord.y = 0.55;

        let w_fps = conf.widget_mut(Widget::Fps);
        w_fps.enabled = false;
        w_fps.x = 0.85;
        w_fps.y = 0.15;

        let w_health = conf.widget_mut(Widget::Health);
        w_health.scale = 2.5;
        w_health.shadow = false;
        w_health.icons = false;

        let saved = conf.line();
        let parsed = Config::parse(Some(&saved));
        assert_eq!(parsed, conf);
    }

    #[test]
    fn widget_customization_and_metadata() {
        assert_eq!(WIDGETS.len(), 14);
        for (i, &w) in WIDGETS.iter().enumerate() {
            assert_eq!(w.index(), i);
            assert_eq!(Widget::from_index(i), Some(w));
            assert_eq!(Widget::from_key(w.key()), Some(w));
            assert!(!w.title(false).is_empty());
            assert!(!w.title(true).is_empty());
        }

        assert_eq!(Widget::Coordinates.key(), "coordinates");
        assert_eq!(Widget::Speed.key(), "speed");
        assert_eq!(Widget::Totems.key(), "totems");
        assert_eq!(Widget::ArmorMini.key(), "armor_mini");

        let mut conf = Config::default();
        assert!(conf.enabled(Widget::Coordinates));
        assert!(conf.enabled(Widget::ArmorMini));
        assert!(!conf.enabled(Widget::Speed));
        assert!(!conf.enabled(Widget::Cps));
        assert!(!conf.enabled(Widget::Clock));

        conf.set_enabled(Widget::Coordinates, false);
        assert!(!conf.enabled(Widget::Coordinates));
        assert!(!conf.widget(Widget::Coordinates).enabled);

        conf.set_enabled(Widget::Effects, false);
        assert!(!conf.enabled(Widget::Effects));
        assert!(!conf.effects);
    }

    #[test]
    fn hitbox_and_move_to_inverse() {
        let w = WidgetConfig {
            enabled: true,
            x: 0.25,
            y: 0.75,
            scale: 1.0,
            text: 0xFFFFFF,
            accent: 0x70C9FF,
            background: 0x10151D,
            opacity: 60,
            shadow: true,
            padding: 3.0,
            compact: false,
            icons: true,
            icons_only: false,
        };

        let base_size = [100.0, 20.0];
        let viewport = [800.0, 600.0];
        let rect = place(&w, base_size, viewport);

        // free_x = 800 - 100 = 700; x = 0.25 * 700 = 175
        // free_y = 600 - 20 = 580; y = 0.75 * 580 = 435
        assert!((rect.x - 175.0).abs() < 1e-4);
        assert!((rect.y - 435.0).abs() < 1e-4);
        assert_eq!(rect.w, 100.0);
        assert_eq!(rect.h, 20.0);

        assert!(rect.contains([175.0, 435.0]));
        assert!(rect.contains([275.0, 455.0]));
        assert!(!rect.contains([174.0, 435.0]));
        assert!(!rect.contains([276.0, 455.0]));

        // Inverse check without snap
        let mut moved = w;
        move_to(&mut moved, [rect.x, rect.y], base_size, viewport, false);
        assert!((moved.x - w.x).abs() < 1e-4);
        assert!((moved.y - w.y).abs() < 1e-4);

        // Snap check (grid 8)
        let mut snapped = w;
        move_to(&mut snapped, [13.0, 21.0], base_size, viewport, true);
        let snapped_rect = place(&snapped, base_size, viewport);
        // 13.0 snapped to nearest 8 is 16.0
        // 21.0 snapped to nearest 8 is 24.0
        assert!((snapped_rect.x - 16.0).abs() < 1e-4);
        assert!((snapped_rect.y - 24.0).abs() < 1e-4);

        // Nearest edge / center snapping when widget almost fills screen (e.g. 10px free viewport)
        let mut edge_w = WidgetConfig::default();
        move_to(&mut edge_w, [10.0, 0.0], [100.0, 10.0], [110.0, 30.0], true);
        assert_eq!(place(&edge_w, [100.0, 10.0], [110.0, 30.0]).x, 10.0);

        let mut center_w = WidgetConfig::default();
        move_to(&mut center_w, [5.0, 0.0], [100.0, 10.0], [110.0, 30.0], true);
        assert_eq!(place(&center_w, [100.0, 10.0], [110.0, 30.0]).x, 5.0);

        // Zero / tiny viewport does not panic
        let tiny_rect = place(&w, [100.0, 20.0], [50.0, 10.0]);
        assert_eq!(tiny_rect.x, 0.0);
        assert_eq!(tiny_rect.y, 0.0);
        let mut zero_w = w;
        move_to(&mut zero_w, [50.0, 50.0], [100.0, 20.0], [0.0, 0.0], true);
        assert_eq!(zero_w.x, w.x);
        assert_eq!(zero_w.y, w.y);

        let mut over_w = WidgetConfig::new(true, 0.7, 0.3);
        move_to(&mut over_w, [0.0, 0.0], [1000.0, 1000.0], [320.0, 180.0], false);
        assert_eq!([over_w.x, over_w.y], [0.7, 0.3]);
    }

    #[test]
    fn cancel_cloned_draft_leaves_live_config_intact() {
        let initial = Config::default();
        install(initial.clone());

        let live_read = config(|| None, |c| c.clone());
        assert_eq!(live_read, initial);

        // Clone draft and mutate it
        let mut draft = live_read.clone();
        draft.widget_mut(Widget::Coordinates).x = 0.99;
        draft.effects = false;

        // "Cancel" by discarding draft without commit
        drop(draft);

        // Live config must remain identical to initial
        let after_cancel = config(|| None, |c| c.clone());
        assert_eq!(after_cancel, initial);
    }

    #[test]
    fn save_failure_and_success_transactions() {
        let initial = Config::default();
        install(initial.clone());

        let mut draft = initial.clone();
        draft.widget_mut(Widget::Coordinates).x = 0.99;

        // Failure case 1: readback returned None
        let mut written = String::new();
        let res_none = commit_verified(&draft, || None, |s| written = s.to_owned());
        assert!(!res_none);
        assert_eq!(written, draft.line());
        assert_eq!(config(|| None, |c| c.clone()), initial);

        // Failure case 2: readback returned corrupt/mismatched string
        let res_mismatch = commit_verified(&draft, || Some("corrupt_string".to_owned()), |_| {});
        assert!(!res_mismatch);
        assert_eq!(config(|| None, |c| c.clone()), initial);

        // Success case: readback returns exact string
        let line = draft.line();
        let res_ok = commit_verified(&draft, || Some(line.clone()), |_| {});
        assert!(res_ok);
        assert_eq!(config(|| None, |c| c.clone()), draft);
    }
}
