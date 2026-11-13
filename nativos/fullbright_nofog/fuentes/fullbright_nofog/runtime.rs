// Fullbright & NoFog: toggle full brightness (lightmap gamma) and the world's
// fog, with a key each, `/fullbright` and `/nofog`, or the settings page in
// Options > Mods. Its settings are files in
// `horizon-mods/.horizon-mod-data/fullbright_nofog/` (`fullbright`, `nofog`,
// `fluids`). This file is both the mod's Cargo library (its tests) and the
// module the bundle adds to the game.
use std::cell::RefCell;

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "fullbright_nofog";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Toggle Fullbright (G, /fullbright) and NoFog (N, /nofog). Rebind the keys here.";
/// The toggle keys' names and defaults.
pub const KEY_FULLBRIGHT: &str = "fullbright";
pub const DEFAULT_KEY_FULLBRIGHT: &str = "key.keyboard.g";
pub const KEY_NOFOG: &str = "nofog";
pub const DEFAULT_KEY_NOFOG: &str = "key.keyboard.n";
/// The gamma Fullbright turns on (1 is vanilla's "Bright").
pub const FULLBRIGHT_GAMMA: f32 = 15.0;
/// Where the fog starts and ends while NoFog is on: past anything the game draws.
pub const FAR: f32 = 1.0e6;

/// The settings.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Settings {
    pub fullbright: bool,
    pub nofog: bool,
    /// NoFog clears water, lava and powder snow fog too (else the air's only).
    pub fluids: bool,
}

impl Settings {
    /// From the stored values (`get` reads one key's file); off for the ones
    /// missing or unreadable.
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let flag = |key: &str| get(key).is_some_and(|v| v.trim() == "true");
        Self {
            fullbright: flag("fullbright"),
            nofog: flag("nofog"),
            fluids: flag("fluids"),
        }
    }
    /// The settings page's buttons.
    pub fn labels(&self) -> Vec<String> {
        vec![
            format!("Fullbright: {}", on_off(self.fullbright)),
            format!("NoFog: {}", on_off(self.nofog)),
            format!("NoFog in water & lava: {}", on_off(self.fluids)),
        ]
    }
    /// A click on button `i` of `labels`.
    pub fn click(&mut self, i: usize) {
        match i {
            0 => self.fullbright = !self.fullbright,
            1 => self.nofog = !self.nofog,
            2 => self.fluids = !self.fluids,
            _ => {}
        }
    }
    /// What to store, key by key.
    pub fn stored(&self) -> [(&'static str, String); 3] {
        [
            ("fullbright", self.fullbright.to_string()),
            ("nofog", self.nofog.to_string()),
            ("fluids", self.fluids.to_string()),
        ]
    }
    /// Whether NoFog clears this fog: the air's always, the others only with `fluids`.
    pub fn clears(&self, atmospheric: bool) -> bool {
        self.nofog && (atmospheric || self.fluids)
    }
}

fn on_off(on: bool) -> &'static str {
    if on { "ON" } else { "OFF" }
}

/// What to do with the fog: its distances pushed past the view.
/// Order: environmental start/end, render distance start/end.
pub fn cleared_distances() -> [f32; 4] {
    [FAR, FAR * 2.0, FAR, FAR * 2.0]
}

/// Runs `command` (without its slash) when it is one of the mod's: the
/// settings after it. None for other commands.
pub fn run(before: Settings, command: &str) -> Option<Settings> {
    let mut words = command.split_whitespace();
    let name = words.next()?.to_ascii_lowercase();
    let arg = words.next().map(str::to_ascii_lowercase);
    let mut s = before;
    match (name.as_str(), arg.as_deref()) {
        ("fullbright" | "fb", arg) => s.fullbright = switch(s.fullbright, arg)?,
        ("nofog", Some("fluids")) => s.fluids = !s.fluids,
        ("nofog", arg) => s.nofog = switch(s.nofog, arg)?,
        _ => return None,
    }
    Some(s)
}

/// `on`/`off` set it, nothing toggles it; anything else is not the mod's.
fn switch(now: bool, arg: Option<&str>) -> Option<bool> {
    match arg {
        None | Some("toggle") => Some(!now),
        Some("on") => Some(true),
        Some("off") => Some(false),
        Some(_) => None,
    }
}

thread_local! {
    /// Read on first use, then kept.
    static STATE: RefCell<Option<Settings>> = const { RefCell::new(None) };
    /// Fullbright wrote the renderer's gamma last frame, so turning it off
    /// gives the slider's back once (and leaves other mods' gamma alone after).
    static APPLIED: RefCell<bool> = const { RefCell::new(false) };
}

/// The settings, read once through `get`.
pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    STATE.with_borrow_mut(|s| *s.get_or_insert_with(|| Settings::read(get)))
}

/// Replaces the settings (a command, a key or the settings page changed them).
pub fn set_settings(settings: Settings) {
    STATE.with_borrow_mut(|s| *s = Some(settings));
}

/// The gamma to give the renderer this frame, if any: Fullbright's while it
/// is on, the slider's once after it goes off, else none.
pub fn frame_gamma(on: bool, slider: f32) -> Option<f32> {
    APPLIED.with_borrow_mut(|applied| {
        let was = std::mem::replace(applied, on);
        if on {
            Some(FULLBRIGHT_GAMMA)
        } else if was {
            Some(slider)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stored<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        |key| pairs.iter().find(|p| p.0 == key).map(|p| p.1.to_owned())
    }

    #[test]
    fn reads_and_keeps_settings() {
        assert_eq!(Settings::read(stored(&[])), Settings::default());
        let s = Settings::read(stored(&[("fullbright", "true\n"), ("nofog", "true")]));
        assert_eq!(
            s,
            Settings {
                fullbright: true,
                nofog: true,
                fluids: false
            }
        );
        let round: Vec<(&str, String)> = s.stored().into();
        let back: Vec<(&str, &str)> = round.iter().map(|(k, v)| (*k, v.as_str())).collect();
        assert_eq!(Settings::read(stored(&back)), s);
    }

    #[test]
    fn commands_toggle_and_set() {
        let off = Settings::default();
        let s = run(off, "fullbright").unwrap();
        assert!(s.fullbright);
        assert!(!run(s, "FB").unwrap().fullbright);
        assert!(run(off, "nofog on").unwrap().nofog);
        assert!(!run(off, "nofog off").unwrap().nofog);
        assert!(run(off, "nofog fluids").unwrap().fluids);
        assert!(run(off, "nofog maybe").is_none());
        assert!(run(off, "gamemode creative").is_none());
        assert!(run(off, "").is_none());
    }

    #[test]
    fn fog_kinds() {
        let air = Settings {
            nofog: true,
            ..Settings::default()
        };
        assert!(air.clears(true));
        assert!(!air.clears(false));
        assert!(
            Settings {
                fluids: true,
                ..air
            }
            .clears(false)
        );
        assert!(!Settings::default().clears(true));
        let [es, ee, rs, re] = cleared_distances();
        assert!(ee > es && re > rs && ee.is_finite() && re.is_finite());
    }

    #[test]
    fn gamma_restores_once() {
        assert_eq!(frame_gamma(false, 0.5), None);
        assert_eq!(frame_gamma(true, 0.5), Some(FULLBRIGHT_GAMMA));
        assert_eq!(frame_gamma(true, 0.5), Some(FULLBRIGHT_GAMMA));
        assert_eq!(frame_gamma(false, 0.5), Some(0.5));
        assert_eq!(frame_gamma(false, 0.5), None);
    }

    #[test]
    fn settings_page_buttons() {
        let mut s = Settings::default();
        s.click(0);
        s.click(1);
        assert_eq!(
            s.labels(),
            ["Fullbright: ON", "NoFog: ON", "NoFog in water & lava: OFF"]
        );
        s.click(2);
        s.click(9);
        assert!(s.fluids);
    }
}
