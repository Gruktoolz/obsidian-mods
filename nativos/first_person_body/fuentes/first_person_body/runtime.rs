// First Person Body for Horizon: your own player model in first person, so
// you see your body, legs and feet when you look down. The head and what is
// worn on it are hidden (the camera is inside it), the body is moved back a
// little so the camera doesn't look into the chest, and the arms are either
// the vanilla first-person hand or the model's own. Inspired by tr7zw's
// First-person Model, written from scratch on Horizon (none of its code).
// This file is both the mod's Cargo library (its tests) and the module the
// bundle adds to the game.
//
// Horizon has no hook point for drawing the local player in first person, so
// the bundle hooks the game's own functions: `own_body_in_view` (whether the
// body is drawn), `player_model_geometry` (while it is drawn),
// `player_model_visibility` (its parts), `base_transform` (where it stands)
// and the layers that would sit at the camera.
use std::cell::Cell;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

/// The mod's id (its settings folder), name and version.
pub const ID: &str = "first_person_body";
pub const NAME: &str = "First Person Body";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "See your own body in first person: look down to see your legs and feet.";

/// The key that turns the body on and off (unbound until the player picks one).
pub const TOGGLE_KEY: &str = "toggle";
pub const TOGGLE_KEY_LABEL: &str = "Toggle First Person Body";
pub const TOGGLE_KEY_DEFAULT: &str = "key.keyboard.unknown";

/// The parts hidden in first person: the head, and the hat layer over it.
pub const HEAD_PARTS: [&str; 2] = ["head", "hat"];
/// The model's arms, hidden when the vanilla hand draws instead.
pub const ARM_PARTS: [&str; 4] = ["right_arm", "left_arm", "right_sleeve", "left_sleeve"];

/// One option of the settings page: its file, its label, the values a click
/// cycles through (with their names) and the default.
pub struct Opt {
    pub key: &'static str,
    pub label: &'static str,
    pub choices: &'static [(u8, &'static str)],
    pub default: u8,
}

const ON_OFF: &[(u8, &str)] = &[(0, "OFF"), (1, "ON")];
const ARMS: &[(u8, &str)] = &[(0, "Vanilla hand"), (1, "Model arms")];
/// How far back the body stands, in hundredths of a block.
const OFFSET: &[(u8, &str)] = &[(0, "None"), (15, "Small"), (25, "Normal"), (35, "Large")];

pub const OPTS: [Opt; 5] = [
    Opt { key: "enabled", label: "First Person Body", choices: ON_OFF, default: 1 },
    Opt { key: "arms", label: "Arms", choices: ARMS, default: 0 },
    Opt { key: "offset", label: "Body offset", choices: OFFSET, default: 25 },
    Opt { key: "riding", label: "While riding", choices: ON_OFF, default: 1 },
    Opt { key: "look_down", label: "Lean back looking down", choices: ON_OFF, default: 1 },
];

/// Indexes into `OPTS`.
pub mod o {
    pub const ENABLED: usize = 0;
    pub const ARMS: usize = 1;
    pub const OFFSET: usize = 2;
    pub const RIDING: usize = 3;
    pub const LOOK_DOWN: usize = 4;
}

pub const RESET_LABEL: &str = "Reset to defaults";

/// The settings, one file each in the mod's data folder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    values: [u8; OPTS.len()],
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
            let value = get(opt.key).and_then(|v| v.trim().parse::<u8>().ok());
            if let Some(v) = value
                && opt.choices.iter().any(|c| c.0 == v)
            {
                s.values[i] = v;
            }
        }
        s
    }

    pub fn get(&self, i: usize) -> u8 {
        self.values[i]
    }

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
            return self.stored();
        }
        let Some(opt) = OPTS.get(i) else {
            return Vec::new();
        };
        let at = opt.choices.iter().position(|c| c.0 == self.values[i]).unwrap_or(0);
        self.values[i] = opt.choices[(at + 1) % opt.choices.len()].0;
        vec![(opt.key, self.values[i].to_string())]
    }

    /// The toggle key: what to store.
    pub fn toggle(&mut self) -> (&'static str, String) {
        self.values[o::ENABLED] = u8::from(self.values[o::ENABLED] == 0);
        (OPTS[o::ENABLED].key, self.values[o::ENABLED].to_string())
    }

    pub fn stored(&self) -> Vec<(&'static str, String)> {
        OPTS.iter().zip(self.values).map(|(opt, v)| (opt.key, v.to_string())).collect()
    }
}

pub fn toggled_line(on: bool) -> String {
    format!("{NAME} {}", if on { "ON" } else { "OFF" })
}

/// Where the body stands relative to the player, in blocks: moved back along
/// the way the player faces (yaw in degrees, 0 south), more when looking
/// down (pitch in degrees, positive down) and when crouching.
pub fn body_offset(settings: &Settings, yaw: f32, pitch: f32, crouching: bool) -> [f32; 3] {
    let mut back = f32::from(settings.get(o::OFFSET)) / 100.0;
    if back == 0.0 {
        return [0.0; 3];
    }
    if settings.get(o::LOOK_DOWN) == 1 {
        back += (pitch.max(0.0) / 90.0) * 0.15;
    }
    if crouching {
        back += 0.05;
    }
    // Facing (-sin yaw, 0, cos yaw); back is the opposite.
    let (s, c) = yaw.to_radians().sin_cos();
    [s * back, 0.0, -c * back]
}

// The settings the render hooks read: kept as plain numbers so any thread may read them.
static LOADED: AtomicBool = AtomicBool::new(false);
static VALUES: [AtomicU8; OPTS.len()] = [const { AtomicU8::new(0) }; OPTS.len()];

/// The settings, read once through `get`, then kept.
pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    if !LOADED.load(Ordering::Acquire) {
        set_settings(Settings::read(get));
    }
    current()
}

pub fn set_settings(settings: Settings) {
    for (slot, v) in VALUES.iter().zip(settings.values) {
        slot.store(v, Ordering::Relaxed);
    }
    LOADED.store(true, Ordering::Release);
}

/// The kept settings (the defaults before they are read).
pub fn current() -> Settings {
    if !LOADED.load(Ordering::Acquire) {
        return Settings::default();
    }
    let mut s = Settings::default();
    for (v, slot) in s.values.iter_mut().zip(&VALUES) {
        *v = slot.load(Ordering::Relaxed);
    }
    s
}

thread_local! {
    /// Set while the game draws the player's own model in first person: the
    /// hooks below the model's drawing act only then.
    static ACTIVE: Cell<Option<[f32; 3]>> = const { Cell::new(None) };
}

/// The own model's drawing starts (`offset`: where the body stands) or ends (None).
pub fn set_active(offset: Option<[f32; 3]>) {
    ACTIVE.with(|a| a.set(offset));
}

/// The body's offset while the own first-person model is drawn.
pub fn active() -> Option<[f32; 3]> {
    ACTIVE.with(Cell::get)
}

/// The parts to hide while active: the head always, the arms with the vanilla hand.
pub fn hidden_parts(settings: &Settings) -> Vec<&'static str> {
    let mut parts = HEAD_PARTS.to_vec();
    if settings.get(o::ARMS) == 0 {
        parts.extend(ARM_PARTS);
    }
    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stored<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        |key| pairs.iter().find(|p| p.0 == key).map(|p| p.1.to_owned())
    }

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-5
    }

    #[test]
    fn defaults_are_choices() {
        for opt in &OPTS {
            assert!(opt.choices.iter().any(|c| c.0 == opt.default), "{}", opt.key);
        }
    }

    #[test]
    fn settings_read_click_toggle_reset() {
        assert_eq!(Settings::read(stored(&[])), Settings::default());
        let mut s = Settings::read(stored(&[("arms", "1"), ("offset", "99")]));
        assert_eq!(s.get(o::ARMS), 1);
        assert_eq!(s.get(o::OFFSET), 25, "not a choice: the default");
        assert_eq!(s.click(o::OFFSET), [("offset", "35".to_owned())]);
        assert_eq!(s.click(o::OFFSET), [("offset", "0".to_owned())]);
        assert_eq!(s.labels()[o::OFFSET], "Body offset: None");
        assert_eq!(s.toggle(), ("enabled", "0".to_owned()));
        assert_eq!(toggled_line(false), "First Person Body OFF");
        assert_eq!(s.click(OPTS.len()).len(), OPTS.len());
        assert_eq!(s, Settings::default());
        assert!(s.click(40).is_empty());
    }

    #[test]
    fn body_moves_back_from_the_view() {
        let s = Settings::default();
        // Facing south (+Z): back is -Z.
        let [x, y, z] = body_offset(&s, 0.0, 0.0, false);
        assert!(close(x, 0.0) && close(y, 0.0) && close(z, -0.25));
        // Facing west (-X): back is +X.
        let [x, _, z] = body_offset(&s, 90.0, 0.0, false);
        assert!(close(x, 0.25) && close(z, 0.0), "{x} {z}");
        // Looking straight down and crouching: further back.
        let [_, _, z] = body_offset(&s, 0.0, 90.0, true);
        assert!(close(z, -0.45));
        let mut none = s;
        none.values[o::OFFSET] = 0;
        assert_eq!(body_offset(&none, 30.0, 60.0, true), [0.0; 3]);
    }

    #[test]
    fn hidden_parts_follow_the_arms_option() {
        let mut s = Settings::default();
        assert_eq!(hidden_parts(&s).len(), 6);
        s.values[o::ARMS] = 1;
        assert_eq!(hidden_parts(&s), HEAD_PARTS);
    }

    #[test]
    fn active_is_per_thread_and_cleared() {
        assert_eq!(active(), None);
        set_active(Some([1.0, 0.0, 0.0]));
        assert_eq!(active(), Some([1.0, 0.0, 0.0]));
        std::thread::spawn(|| assert_eq!(active(), None)).join().unwrap();
        set_active(None);
        assert_eq!(active(), None);
    }

    #[test]
    fn settings_load_once_and_update() {
        let s = settings(stored(&[("arms", "1")]));
        assert_eq!(s.get(o::ARMS), 1);
        assert_eq!(settings(stored(&[("arms", "0")])).get(o::ARMS), 1, "read once");
        let mut changed = s;
        changed.toggle();
        set_settings(changed);
        assert_eq!(current().get(o::ENABLED), 0);
    }
}
