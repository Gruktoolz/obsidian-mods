// Toggle Sprint runtime state & pure movement HUD label/boost calculations.
// Compiles both standalone (`cargo test`) and inside `horizon-client` via `horizon-mixins`.

use std::sync::Mutex;

pub const ID: &str = "toggle_sprint";
pub const NAME: &str = "Toggle Sprint";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "1:1 Better Sprinting & Lunar ToggleSprint HUD: toggle sprint and toggle sneak with [Sprinting (Toggled)] / [Key Held] / [Flying] status HUD and configurable Creative Fly Speed Boost (1x-4x).";

pub const KEY_SPRINT: &str = "toggle_sprint";
pub const KEY_SPRINT_LABEL: &str = "Toggle Sprint";
pub const KEY_SPRINT_DEFAULT: &str = "key.keyboard.v";

pub const KEY_SNEAK: &str = "toggle_sneak";
pub const KEY_SNEAK_LABEL: &str = "Toggle Sneak";
pub const KEY_SNEAK_DEFAULT: &str = "key.keyboard.c";

pub const BACKGROUND: u32 = 0x7800_0000;
pub const DEFAULT_FLY_SPEED: f32 = 0.05;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    pub enabled: bool,
    pub sprint_toggled: bool,
    pub sneak_toggled: bool,
    /// Also allow tapping the vanilla Sprint key (e.g. Left Ctrl) to flip Toggle Sprint
    pub tap_sprint_key_toggles: bool,
    /// 0 = 1.0x (Off), 1 = 1.5x, 2 = 2.0x, 3 = 3.0x, 4 = 4.0x
    pub fly_boost: u8,
    pub show_hud: bool,
    /// 0 = Bottom-Left, 1 = Top-Left, 2 = Top-Right, 3 = Bottom-Right
    pub position: u8,
    pub background: bool,
    /// 0 = White, 1 = Gold, 2 = Aqua, 3 = Green
    pub color_preset: u8,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            enabled: true,
            sprint_toggled: true,
            sneak_toggled: false,
            tap_sprint_key_toggles: true,
            fly_boost: 2, // 2.0x default in Better Sprinting / Lunar
            show_hud: true,
            position: 0,
            background: true,
            color_preset: 0,
        }
    }
}

impl Settings {
    pub fn load<'a>(mut get: impl FnMut(&str) -> Option<String> + 'a) -> Self {
        let mut s = Self::default();
        if let Some(v) = get("enabled") {
            s.enabled = v != "false" && v != "0";
        }
        if let Some(v) = get("sprint_toggled") {
            s.sprint_toggled = v != "false" && v != "0";
        }
        if let Some(v) = get("sneak_toggled") {
            s.sneak_toggled = v == "true" || v == "1";
        }
        if let Some(v) = get("tap_sprint_key_toggles") {
            s.tap_sprint_key_toggles = v != "false" && v != "0";
        }
        if let Some(v) = get("fly_boost").and_then(|v| v.parse::<u8>().ok()) {
            s.fly_boost = v % 5;
        }
        if let Some(v) = get("show_hud") {
            s.show_hud = v != "false" && v != "0";
        }
        if let Some(v) = get("position").and_then(|v| v.parse::<u8>().ok()) {
            s.position = v % 4;
        }
        if let Some(v) = get("background") {
            s.background = v != "false" && v != "0";
        }
        if let Some(v) = get("color_preset").and_then(|v| v.parse::<u8>().ok()) {
            s.color_preset = v % 4;
        }
        s
    }

    pub fn stored(&self) -> [(&'static str, String); 9] {
        let b = |v: bool| if v { "true" } else { "false" }.to_owned();
        [
            ("enabled", b(self.enabled)),
            ("sprint_toggled", b(self.sprint_toggled)),
            ("sneak_toggled", b(self.sneak_toggled)),
            ("tap_sprint_key_toggles", b(self.tap_sprint_key_toggles)),
            ("fly_boost", self.fly_boost.to_string()),
            ("show_hud", b(self.show_hud)),
            ("position", self.position.to_string()),
            ("background", b(self.background)),
            ("color_preset", self.color_preset.to_string()),
        ]
    }

    pub fn fly_boost_multiplier(&self) -> f32 {
        match self.fly_boost % 5 {
            0 => 1.0,
            1 => 1.5,
            2 => 2.0,
            3 => 3.0,
            _ => 4.0,
        }
    }

    pub fn fly_boost_name(idx: u8) -> &'static str {
        match idx % 5 {
            0 => "1.0x (Vanilla)",
            1 => "1.5x",
            2 => "2.0x",
            3 => "3.0x",
            _ => "4.0x",
        }
    }

    pub fn position_name(pos: u8) -> &'static str {
        match pos % 4 {
            0 => "Bottom-Left",
            1 => "Top-Left",
            2 => "Top-Right",
            _ => "Bottom-Right",
        }
    }

    pub fn color_name(preset: u8) -> &'static str {
        match preset % 4 {
            0 => "White",
            1 => "Gold",
            2 => "Aqua",
            _ => "Green",
        }
    }

    pub fn text_color(&self) -> u32 {
        match self.color_preset % 4 {
            0 => 0x00FF_FFFF,
            1 => 0x00FF_AA00,
            2 => 0x0055_FFFF,
            _ => 0x0055_FF55,
        }
    }

    pub fn labels(&self) -> Vec<String> {
        let on_off = |v: bool| if v { "ON" } else { "OFF" };
        vec![
            format!("Toggle Sprint Mod: {}", on_off(self.enabled)),
            format!("Sprint State: {}", if self.sprint_toggled { "Toggled ON" } else { "OFF" }),
            format!("Sneak State: {}", if self.sneak_toggled { "Toggled ON" } else { "OFF" }),
            format!("Vanilla Sprint Key Toggles: {}", on_off(self.tap_sprint_key_toggles)),
            format!("Creative Fly Speed Boost: {}", Self::fly_boost_name(self.fly_boost)),
            format!("Movement Status HUD: {}", on_off(self.show_hud)),
            format!("HUD Position: {}", Self::position_name(self.position)),
            format!("Background Box: {}", on_off(self.background)),
            format!("Text Color: {}", Self::color_name(self.color_preset)),
        ]
    }

    pub fn click(&mut self, button: usize) {
        match button {
            0 => self.enabled = !self.enabled,
            1 => self.sprint_toggled = !self.sprint_toggled,
            2 => self.sneak_toggled = !self.sneak_toggled,
            3 => self.tap_sprint_key_toggles = !self.tap_sprint_key_toggles,
            4 => self.fly_boost = (self.fly_boost + 1) % 5,
            5 => self.show_hud = !self.show_hud,
            6 => self.position = (self.position + 1) % 4,
            7 => self.background = !self.background,
            8 => self.color_preset = (self.color_preset + 1) % 4,
            _ => {}
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MovementSnapshot {
    pub flying: bool,
    pub fall_flying: bool,
    pub riding: bool,
    pub swimming: bool,
    pub sprinting: bool,
    pub crouching: bool,
    pub sprint_key_held: bool,
    pub sneak_key_held: bool,
}

#[derive(Clone, Debug, Default)]
pub struct RuntimeState {
    pub loaded: bool,
    pub settings: Settings,
}

static STATE: Mutex<RuntimeState> = Mutex::new(RuntimeState {
    loaded: false,
    settings: Settings {
        enabled: true,
        sprint_toggled: true,
        sneak_toggled: false,
        tap_sprint_key_toggles: true,
        fly_boost: 2,
        show_hud: true,
        position: 0,
        background: true,
        color_preset: 0,
    },
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

/// Computes the 1:1 Lunar / Badlion / Better Sprinting HUD status text for the current movement state.
pub fn format_status_text(s: &Settings, mov: &MovementSnapshot) -> Option<String> {
    if !s.enabled || !s.show_hud {
        return None;
    }
    if mov.flying {
        let mult = s.fly_boost_multiplier();
        if mult > 1.0 && (s.sprint_toggled || mov.sprint_key_held || mov.sprinting) {
            return Some(format!("[Flying ({mult:.1}x Boost)]"));
        }
        return Some("[Flying]".to_owned());
    }
    if mov.fall_flying {
        return Some("[Gliding (Elytra)]".to_owned());
    }
    if mov.riding {
        return Some("[Riding]".to_owned());
    }
    if mov.swimming {
        return Some("[Swimming]".to_owned());
    }
    if s.sneak_toggled {
        return Some("[Sneaking (Toggled)]".to_owned());
    }
    if mov.sneak_key_held || mov.crouching {
        return Some("[Sneaking (Key Held)]".to_owned());
    }
    if s.sprint_toggled {
        return Some("[Sprinting (Toggled)]".to_owned());
    }
    if mov.sprint_key_held || mov.sprinting {
        return Some("[Sprinting (Key Held)]".to_owned());
    }
    None
}

/// Computes the effective Creative `flying_speed` (`0.05 * multiplier` when boost is active).
pub fn compute_flying_speed(s: &Settings, sprint_active: bool) -> f32 {
    if !s.enabled || !sprint_active {
        return DEFAULT_FLY_SPEED;
    }
    DEFAULT_FLY_SPEED * s.fly_boost_multiplier()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_labels_follow_lunar_and_better_sprinting_priority() {
        let mut s = Settings::default();
        let mut mov = MovementSnapshot::default();

        // Default has sprint_toggled = true -> [Sprinting (Toggled)]
        assert_eq!(
            format_status_text(&s, &mov).as_deref(),
            Some("[Sprinting (Toggled)]")
        );

        // Turn off sprint_toggled, hold sprint key -> [Sprinting (Key Held)]
        s.sprint_toggled = false;
        mov.sprint_key_held = true;
        assert_eq!(
            format_status_text(&s, &mov).as_deref(),
            Some("[Sprinting (Key Held)]")
        );

        // Sneaking overrides sprinting
        s.sneak_toggled = true;
        assert_eq!(
            format_status_text(&s, &mov).as_deref(),
            Some("[Sneaking (Toggled)]")
        );

        // Flying with 2.0x boost overrides sneaking
        mov.flying = true;
        assert_eq!(
            format_status_text(&s, &mov).as_deref(),
            Some("[Flying (2.0x Boost)]")
        );
    }

    #[test]
    fn creative_fly_speed_boost_scales_correctly() {
        let mut s = Settings::default();
        s.fly_boost = 2; // 2.0x
        assert!((compute_flying_speed(&s, true) - 0.10).abs() < 1e-6);
        assert!((compute_flying_speed(&s, false) - 0.05).abs() < 1e-6);
        s.fly_boost = 4; // 4.0x
        assert!((compute_flying_speed(&s, true) - 0.20).abs() < 1e-6);
    }

    #[test]
    fn settings_persistence_roundtrip() {
        let mut s = Settings::default();
        s.click(2); // toggle sneak
        s.click(4); // fly boost -> 3 (3.0x)
        s.click(8); // color -> 1 (Gold)
        assert!(s.sneak_toggled);
        assert_eq!(s.fly_boost, 3);
        assert_eq!(s.color_preset, 1);

        let map: std::collections::HashMap<_, _> = s.stored().into_iter().collect();
        let loaded = Settings::load(|k| map.get(k).cloned());
        assert_eq!(s, loaded);
    }
}
