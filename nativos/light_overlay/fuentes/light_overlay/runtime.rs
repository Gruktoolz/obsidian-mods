// Light Overlay runtime state & pure 3D geometry/spawn classification logic.
// Compiles both standalone (`cargo test`) and inside `horizon-client` via `horizon-mixins`.

use std::sync::Mutex;

pub const ID: &str = "light_overlay";
pub const NAME: &str = "Light Overlay";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "1:1 Light Overlay & MiniHUD (F7): visualizes mob-spawnable blocks with 3D Yellow (night spawn) and Red (always/cave spawn) crosses (X) or 3D light level numbers (0-15).";

pub const KEY_TOGGLE: &str = "toggle";
pub const KEY_TOGGLE_LABEL: &str = "Toggle Light Overlay";
pub const KEY_TOGGLE_DEFAULT: &str = "key.keyboard.f7";

pub const BACKGROUND: u32 = 0x7800_0000;
pub const COLOR_RED: u32 = 0xFFFF_3333;
pub const COLOR_YELLOW: u32 = 0xFFFF_DD22;
pub const COLOR_GREEN: u32 = 0xFF33_FF55;
pub const HUD_ACTIVE_COLOR: u32 = 0x00FF_DD22;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpawnSafety {
    /// Block light <= threshold AND Sky light <= threshold: hostile mobs can spawn any time (Red).
    AlwaysSpawnable,
    /// Block light <= threshold AND Sky light > threshold: hostile mobs can spawn at night (Yellow).
    NightSpawnable,
    /// Block light > threshold: safe from normal hostile mob spawns (Green).
    Safe,
}

impl SpawnSafety {
    pub fn argb(self) -> u32 {
        match self {
            Self::AlwaysSpawnable => COLOR_RED,
            Self::NightSpawnable => COLOR_YELLOW,
            Self::Safe => COLOR_GREEN,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    pub active: bool,
    /// 0 = Crosses (X), 1 = Numbers (0-15), 2 = Both (Crosses + Numbers)
    pub mode: u8,
    /// 0 = 8 blocks, 1 = 12 blocks (default), 2 = 16 blocks, 3 = 20 blocks
    pub radius_index: u8,
    /// false = Modern 1.18+/26.3 (Block Light == 0), true = Legacy (Block Light <= 7)
    pub legacy_threshold: bool,
    /// Show safe (Green) blocks when in Numbers mode
    pub show_safe: bool,
    pub show_hud: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            active: false,
            mode: 0,
            radius_index: 1,
            legacy_threshold: false,
            show_safe: false,
            show_hud: true,
        }
    }
}

impl Settings {
    pub fn load<'a>(mut get: impl FnMut(&str) -> Option<String> + 'a) -> Self {
        let mut s = Self::default();
        if let Some(v) = get("active") {
            s.active = v == "true" || v == "1";
        }
        if let Some(v) = get("mode").and_then(|v| v.parse::<u8>().ok()) {
            s.mode = v % 3;
        }
        if let Some(v) = get("radius_index").and_then(|v| v.parse::<u8>().ok()) {
            s.radius_index = v % 4;
        }
        if let Some(v) = get("legacy_threshold") {
            s.legacy_threshold = v == "true" || v == "1";
        }
        if let Some(v) = get("show_safe") {
            s.show_safe = v == "true" || v == "1";
        }
        if let Some(v) = get("show_hud") {
            s.show_hud = v != "false" && v != "0";
        }
        s
    }

    pub fn stored(&self) -> [(&'static str, String); 6] {
        let b = |v: bool| if v { "true" } else { "false" }.to_owned();
        [
            ("active", b(self.active)),
            ("mode", self.mode.to_string()),
            ("radius_index", self.radius_index.to_string()),
            ("legacy_threshold", b(self.legacy_threshold)),
            ("show_safe", b(self.show_safe)),
            ("show_hud", b(self.show_hud)),
        ]
    }

    pub fn mode_name(mode: u8) -> &'static str {
        match mode % 3 {
            0 => "Crosses (X)",
            1 => "Numbers (0-15)",
            _ => "Crosses + Numbers",
        }
    }

    pub fn radius(&self) -> i32 {
        match self.radius_index % 4 {
            0 => 8,
            1 => 12,
            2 => 16,
            _ => 20,
        }
    }

    pub fn threshold(&self) -> u8 {
        if self.legacy_threshold { 7 } else { 0 }
    }

    pub fn labels(&self) -> Vec<String> {
        let on_off = |v: bool| if v { "ON" } else { "OFF" };
        vec![
            format!("Light Overlay (F7): {}", on_off(self.active)),
            format!("Display Style: {}", Self::mode_name(self.mode)),
            format!("Scan Radius: {} Blocks", self.radius()),
            format!(
                "Spawn Threshold: {}",
                if self.legacy_threshold {
                    "Legacy (<= 7)"
                } else {
                    "Modern (== 0)"
                }
            ),
            format!("Show Safe Blocks (Green): {}", on_off(self.show_safe)),
            format!("Show Active HUD Badge: {}", on_off(self.show_hud)),
        ]
    }

    pub fn click(&mut self, button: usize) {
        match button {
            0 => self.active = !self.active,
            1 => self.mode = (self.mode + 1) % 3,
            2 => self.radius_index = (self.radius_index + 1) % 4,
            3 => self.legacy_threshold = !self.legacy_threshold,
            4 => self.show_safe = !self.show_safe,
            5 => self.show_hud = !self.show_hud,
            _ => {}
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct RuntimeState {
    pub loaded: bool,
    pub settings: Settings,
    pub red_count: u32,
    pub yellow_count: u32,
}

static STATE: Mutex<RuntimeState> = Mutex::new(RuntimeState {
    loaded: false,
    settings: Settings {
        active: false,
        mode: 0,
        radius_index: 1,
        legacy_threshold: false,
        show_safe: false,
        show_hud: true,
    },
    red_count: 0,
    yellow_count: 0,
});

pub fn settings<'a>(get: impl FnMut(&str) -> Option<String> + 'a) -> Settings {
    let mut guard = STATE.lock().unwrap_or_else(|e| e.into_inner());
    if !guard.loaded {
        guard.settings = Settings::load(get);
        guard.loaded = true;
    }
    guard.settings
}

pub fn set_settings(s: Settings) {
    let mut guard = STATE.lock().unwrap_or_else(|e| e.into_inner());
    guard.settings = s;
    guard.loaded = true;
}

pub fn set_counts(red: u32, yellow: u32) {
    let mut guard = STATE.lock().unwrap_or_else(|e| e.into_inner());
    guard.red_count = red;
    guard.yellow_count = yellow;
}

pub fn counts() -> (u32, u32) {
    let guard = STATE.lock().unwrap_or_else(|e| e.into_inner());
    (guard.red_count, guard.yellow_count)
}

/// Classifies a block top's mob-spawning safety given `block_light`, `sky_light`, and `threshold`.
pub fn classify_spawn(block_light: u8, sky_light: u8, threshold: u8) -> SpawnSafety {
    if block_light <= threshold {
        if sky_light <= threshold {
            SpawnSafety::AlwaysSpawnable
        } else {
            SpawnSafety::NightSpawnable
        }
    } else {
        SpawnSafety::Safe
    }
}

/// Generates the two diagonal 3D line segments forming an `X` cross on the top face of `(bx, by, bz)`.
pub fn cross_segments(bx: i32, by: i32, bz: i32) -> [([f64; 3], [f64; 3]); 2] {
    let x0 = f64::from(bx) + 0.18;
    let x1 = f64::from(bx) + 0.82;
    let y = f64::from(by) + 1.02;
    let z0 = f64::from(bz) + 0.18;
    let z1 = f64::from(bz) + 0.82;
    [([x0, y, z0], [x1, y, z1]), ([x0, y, z1], [x1, y, z0])]
}

/// 7-segment bitmask for digits 0..=9:
/// bit 0: top, bit 1: top-left, bit 2: top-right, bit 3: middle,
/// bit 4: bottom-left, bit 5: bottom-right, bit 6: bottom
const DIGIT_MASK: [u8; 10] = [
    0b111_0111, // 0
    0b010_0100, // 1
    0b101_1101, // 2
    0b110_1101, // 3
    0b010_1110, // 4
    0b110_1011, // 5
    0b111_1011, // 6
    0b010_0101, // 7
    0b111_1111, // 8
    0b110_1111, // 9
];

fn push_digit_segments(
    out: &mut Vec<([f64; 3], [f64; 3])>,
    digit: u8,
    cx: f64,
    y: f64,
    cz: f64,
    half_w: f64,
    half_h: f64,
) {
    let mask = DIGIT_MASK[(digit % 10) as usize];
    let x0 = cx - half_w;
    let x1 = cx + half_w;
    let z0 = cz - half_h;
    let zm = cz;
    let z1 = cz + half_h;

    if mask & (1 << 0) != 0 {
        out.push(([x0, y, z0], [x1, y, z0]));
    }
    if mask & (1 << 1) != 0 {
        out.push(([x0, y, z0], [x0, y, zm]));
    }
    if mask & (1 << 2) != 0 {
        out.push(([x1, y, z0], [x1, y, zm]));
    }
    if mask & (1 << 3) != 0 {
        out.push(([x0, y, zm], [x1, y, zm]));
    }
    if mask & (1 << 4) != 0 {
        out.push(([x0, y, zm], [x0, y, z1]));
    }
    if mask & (1 << 5) != 0 {
        out.push(([x1, y, zm], [x1, y, z1]));
    }
    if mask & (1 << 6) != 0 {
        out.push(([x0, y, z1], [x1, y, z1]));
    }
}

/// Generates 3D line segments on the top face of `(bx, by, bz)` for light level `0..=15`.
pub fn number_segments(bx: i32, by: i32, bz: i32, level: u8) -> Vec<([f64; 3], [f64; 3])> {
    let mut out = Vec::with_capacity(12);
    let cx = f64::from(bx) + 0.5;
    let y = f64::from(by) + 1.03;
    let cz = f64::from(bz) + 0.5;
    let lvl = level.min(15);
    if lvl < 10 {
        push_digit_segments(&mut out, lvl, cx, y, cz, 0.12, 0.20);
    } else {
        push_digit_segments(&mut out, 1, cx - 0.14, y, cz, 0.08, 0.20);
        push_digit_segments(&mut out, lvl - 10, cx + 0.12, y, cz, 0.10, 0.20);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_safety_classification_matches_modern_and_legacy_rules() {
        // Modern threshold (0)
        assert_eq!(classify_spawn(0, 0, 0), SpawnSafety::AlwaysSpawnable);
        assert_eq!(classify_spawn(0, 15, 0), SpawnSafety::NightSpawnable);
        assert_eq!(classify_spawn(1, 0, 0), SpawnSafety::Safe);

        // Legacy threshold (7)
        assert_eq!(classify_spawn(7, 5, 7), SpawnSafety::AlwaysSpawnable);
        assert_eq!(classify_spawn(7, 12, 7), SpawnSafety::NightSpawnable);
        assert_eq!(classify_spawn(8, 0, 7), SpawnSafety::Safe);
    }

    #[test]
    fn cross_and_number_geometry_stay_within_block_face() {
        let cross = cross_segments(10, 64, -5);
        assert_eq!(cross.len(), 2);
        for (a, b) in cross {
            assert!((a[1] - 65.02).abs() < 1e-6);
            assert!((b[1] - 65.02).abs() < 1e-6);
        }

        let num0 = number_segments(0, 64, 0, 0);
        assert_eq!(num0.len(), 6); // '0' has 6 segments
        let num15 = number_segments(0, 64, 0, 15);
        assert_eq!(num15.len(), 2 + 5); // '1' (2 segs) + '5' (5 segs)
    }

    #[test]
    fn settings_persistence_roundtrip() {
        let mut s = Settings::default();
        assert!(!s.active);
        s.click(0); // toggle active
        s.click(1); // mode -> 1
        s.click(2); // radius_index -> 2 (16)
        assert!(s.active);
        assert_eq!(s.mode, 1);
        assert_eq!(s.radius(), 16);

        let map: std::collections::HashMap<_, _> = s.stored().into_iter().collect();
        let loaded = Settings::load(|k| map.get(k).cloned());
        assert_eq!(s, loaded);
    }
}
