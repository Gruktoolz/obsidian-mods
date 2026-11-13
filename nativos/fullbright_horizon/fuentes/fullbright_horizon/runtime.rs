// Fullbright (Gamma Utils) native mod for Horizon: toggle maximum brightness
// (1000% gamma / full lightmap illumination without shadows in caves and at
// night) using a keybind (G by default), with optional smooth transition and
// HUD status notification (`Fullbright: ON` / `Fullbright: OFF`).
// This file is both the standalone Cargo library (for unit tests) and the
// runtime module injected into `crates/horizon-client/src/game.rs`.
use std::cell::RefCell;

/// The mod's id (its settings folder and its keybind prefix).
pub const ID: &str = "fullbright_horizon";
pub const NAME: &str = "Fullbright";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Toggle maximum brightness (1000% gamma) in caves and at night with G, with optional smooth fade and HUD feedback.";

/// The toggle key's name and its default binding (`G`).
pub const KEY: &str = "toggle";
pub const KEY_LABEL: &str = "Toggle Fullbright";
pub const DEFAULT_KEY: &str = "key.keyboard.g";

/// Maximum gamma value (10.0 = 1000%, matching Java's Gamma Utils / Fullbright).
pub const MAX_GAMMA: f32 = 10.0;
/// Duration in seconds for smooth brightness transitions.
pub const FADE_SECONDS: f32 = 0.25;
/// Duration in seconds to show the `Fullbright: ON/OFF` HUD notification.
pub const HUD_SECONDS: f32 = 1.5;

/// HUD notification colors (`0xAARRGGBB` background, `0xRRGGBB` text colors).
pub const BACKGROUND: u32 = 0x7800_0000;
pub const COLOR_ON: u32 = 0x55_FF55;
pub const COLOR_OFF: u32 = 0xFF_5555;

/// Persistent settings stored under `horizon-mods/.horizon-mod-data/fullbright/`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// Whether Fullbright is currently toggled on.
    pub enabled: bool,
    /// Whether to smoothly interpolate gamma when toggling.
    pub smooth: bool,
    /// Whether to show `Fullbright: ON` / `Fullbright: OFF` on the HUD when toggled.
    pub hud_feedback: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            enabled: false,
            smooth: false,
            hud_feedback: true,
        }
    }
}

impl Settings {
    /// Reads settings from Horizon's per-key storage, falling back to defaults.
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let enabled = get("enabled").is_some_and(|v| v.trim() == "true");
        let smooth = get("smooth").is_some_and(|v| v.trim() == "true");
        let hud_feedback = get("hud_feedback").is_none_or(|v| v.trim() != "false");
        Self {
            enabled,
            smooth,
            hud_feedback,
        }
    }

    /// Button labels for `Options > Mods > Fullbright`.
    pub fn labels(&self) -> Vec<String> {
        let on = |b: bool| if b { "ON" } else { "OFF" };
        vec![
            format!("Fullbright: {}", on(self.enabled)),
            format!("Smooth transition: {}", on(self.smooth)),
            format!("HUD notification: {}", on(self.hud_feedback)),
        ]
    }

    /// Handles a click on button `i` in `Options > Mods > Fullbright`.
    pub fn click(&mut self, i: usize) {
        match i {
            0 => self.enabled = !self.enabled,
            1 => self.smooth = !self.smooth,
            2 => self.hud_feedback = !self.hud_feedback,
            _ => {}
        }
    }

    /// Key-value pairs to persist in `horizon-mods/.horizon-mod-data/fullbright/`.
    pub fn stored(&self) -> [(&'static str, String); 3] {
        let flag = |b: bool| if b { "true" } else { "false" }.to_owned();
        [
            ("enabled", flag(self.enabled)),
            ("smooth", flag(self.smooth)),
            ("hud_feedback", flag(self.hud_feedback)),
        ]
    }
}

#[derive(Default)]
struct State {
    /// Loaded on first frame or key press, then cached in memory.
    settings: Option<Settings>,
    /// Current interpolation factor in `0.0..=1.0` (`0.0` = vanilla, `1.0` = fullbright).
    progress: f32,
    /// Remaining seconds for the HUD notification (`Fullbright: ON/OFF`).
    hud_timer: f32,
    /// Last toggled state shown in the HUD notification.
    hud_last_on: bool,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

/// Returns the current settings, loading them once via `get` if needed.
pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    STATE.with_borrow_mut(|s| {
        if let Some(existing) = s.settings {
            return existing;
        }
        let loaded = Settings::read(get);
        s.progress = if loaded.enabled { 1.0 } else { 0.0 };
        s.settings = Some(loaded);
        loaded
    })
}

/// Updates the in-memory settings (e.g. after a click in `Options > Mods`).
pub fn set_settings(new_settings: Settings) {
    STATE.with_borrow_mut(|s| {
        if !new_settings.smooth {
            s.progress = if new_settings.enabled { 1.0 } else { 0.0 };
        }
        s.settings = Some(new_settings);
    });
}

/// Resets runtime transition state when the mod is turned off in `Options > Mods`.
pub fn disable_mod() {
    STATE.with_borrow_mut(|s| {
        s.progress = 0.0;
        s.hud_timer = 0.0;
    });
}

/// Handles the toggle key press (`G` by default). Returns `Some(Settings)` when
/// the state changed so the caller can persist the new `enabled` state.
pub fn toggle_key(
    playing: bool,
    pressed: bool,
    get: impl Fn(&str) -> Option<String>,
) -> Option<Settings> {
    if !pressed || !playing {
        return None;
    }
    let mut current = settings(get);
    current.enabled = !current.enabled;
    STATE.with_borrow_mut(|s| {
        s.settings = Some(current);
        if !current.smooth {
            s.progress = if current.enabled { 1.0 } else { 0.0 };
        }
        if current.hud_feedback {
            s.hud_timer = HUD_SECONDS;
            s.hud_last_on = current.enabled;
        } else {
            s.hud_timer = 0.0;
        }
    });
    Some(current)
}

/// Advances the smooth transition and HUD timer by `dt` seconds, returning the
/// `(gamma, fullbright_flag)` to apply to the renderer for this frame.
pub fn step(
    dt: f32,
    base_gamma: f32,
    base_fullbright: bool,
    get: impl Fn(&str) -> Option<String>,
) -> (f32, bool) {
    let cfg = settings(get);
    let safe_dt = if dt.is_finite() && dt > 0.0 {
        dt.min(0.25)
    } else {
        0.0
    };
    let clean_base = if base_gamma.is_finite() {
        base_gamma.clamp(0.0, MAX_GAMMA)
    } else {
        1.0
    };

    STATE.with_borrow_mut(|s| {
        if s.hud_timer > 0.0 {
            s.hud_timer = (s.hud_timer - safe_dt).max(0.0);
        }

        let target = if cfg.enabled { 1.0 } else { 0.0 };
        if cfg.smooth && FADE_SECONDS > 0.0 {
            let delta = safe_dt / FADE_SECONDS;
            if s.progress < target {
                s.progress = (s.progress + delta).min(target);
            } else if s.progress > target {
                s.progress = (s.progress - delta).max(target);
            }
        } else {
            s.progress = target;
        }

        if s.progress <= 0.0 {
            (clean_base, base_fullbright)
        } else if s.progress >= 1.0 {
            (MAX_GAMMA, true)
        } else {
            let gamma = clean_base + (MAX_GAMMA - clean_base) * s.progress;
            (gamma, base_fullbright)
        }
    })
}

/// Returns the active HUD notification text and RGB color if the feedback timer is active.
pub fn hud_banner() -> Option<(&'static str, u32)> {
    STATE.with_borrow(|s| {
        let cfg = s.settings?;
        if !cfg.hud_feedback || s.hud_timer <= 0.0 {
            return None;
        }
        if s.hud_last_on {
            Some(("Fullbright: ON", COLOR_ON))
        } else {
            Some(("Fullbright: OFF", COLOR_OFF))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stored<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        |key| pairs.iter().find(|p| p.0 == key).map(|p| p.1.to_owned())
    }

    fn reset_state() {
        STATE.with_borrow_mut(|s| *s = State::default());
    }

    #[test]
    fn reads_and_writes_settings_cleanly() {
        reset_state();
        let default_s = Settings::read(stored(&[]));
        assert_eq!(
            default_s,
            Settings {
                enabled: false,
                smooth: false,
                hud_feedback: true,
            }
        );

        let custom = Settings::read(stored(&[
            ("enabled", "true\n"),
            ("smooth", "true"),
            ("hud_feedback", "false"),
        ]));
        assert_eq!(
            custom,
            Settings {
                enabled: true,
                smooth: true,
                hud_feedback: false,
            }
        );

        let pairs = custom.stored();
        let refs: Vec<(&str, &str)> = pairs.iter().map(|(k, v)| (*k, v.as_str())).collect();
        assert_eq!(Settings::read(stored(&refs)), custom);
    }

    #[test]
    fn toggles_instant_fullbright_and_restores_original_gamma() {
        reset_state();
        let empty = stored(&[]);

        // Not playing (e.g. chat/inventory open): key press is ignored.
        assert_eq!(toggle_key(false, true, &empty), None);
        assert_eq!(step(0.016, 0.5, false, &empty), (0.5, false));

        // Press G while playing -> instant 1000% gamma (10.0) + fullbright lightmap.
        let toggled_on = toggle_key(true, true, &empty);
        assert!(toggled_on.is_some_and(|s| s.enabled));
        assert_eq!(step(0.016, 0.5, false, &empty), (MAX_GAMMA, true));
        assert_eq!(hud_banner(), Some(("Fullbright: ON", COLOR_ON)));

        // Key release does nothing.
        assert_eq!(toggle_key(true, false, &empty), None);

        // Press G again -> restores exact original gamma (0.5) and fullbright (false).
        let toggled_off = toggle_key(true, true, &empty);
        assert!(toggled_off.is_some_and(|s| !s.enabled));
        assert_eq!(step(0.016, 0.5, false, &empty), (0.5, false));
        assert_eq!(hud_banner(), Some(("Fullbright: OFF", COLOR_OFF)));

        // Banner expires after HUD_SECONDS.
        let _ = step(0.25, 0.5, false, &empty);
        let _ = step(0.25, 0.5, false, &empty);
        let _ = step(0.25, 0.5, false, &empty);
        let _ = step(0.25, 0.5, false, &empty);
        let _ = step(0.25, 0.5, false, &empty);
        let _ = step(0.25, 0.5, false, &empty);
        assert_eq!(hud_banner(), None);
    }

    #[test]
    fn smooth_transition_interpolates_gamma_and_returns_to_base() {
        reset_state();
        let smooth_store = stored(&[("smooth", "true")]);

        let _ = toggle_key(true, true, &smooth_store);
        // Halfway through 0.25s fade (0.125s): gamma is halfway between 1.0 and 10.0 (5.5).
        let (mid_gamma, mid_fb) = step(0.125, 1.0, false, &smooth_store);
        assert!((mid_gamma - 5.5).abs() < 1e-4);
        assert!(!mid_fb);

        // Complete the fade (another 0.125s): reaches MAX_GAMMA and fullbright = true.
        let (full_gamma, full_fb) = step(0.125, 1.0, false, &smooth_store);
        assert!((full_gamma - MAX_GAMMA).abs() < 1e-4);
        assert!(full_fb);

        // Toggle off and fade back to original gamma (1.0).
        let _ = toggle_key(true, true, &smooth_store);
        let (mid_back, _) = step(0.125, 1.0, false, &smooth_store);
        assert!((mid_back - 5.5).abs() < 1e-4);
        let (restored, restored_fb) = step(0.125, 1.0, false, &smooth_store);
        assert!((restored - 1.0).abs() < 1e-4);
        assert!(!restored_fb);
    }
}
