// Fullbright: set the brightness (lightmap gamma) past the video settings
// slider's 100% with `/gamma <value>`, and toggle full brightness with
// `/fullbright` or a key. Its settings are files in
// `horizon-mods/.horizon-mod-data/fullbright/` (`enabled`, `gamma`). This file
// is both the mod's Cargo library (its tests) and the module the bundle adds
// to the game.
use std::cell::RefCell;

/// The mod's id: its settings folder and its key's line.
pub const ID: &str = "fullbright";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Brightness past 100%: /gamma <value> (1 = Bright, 15 = fullbright), /gamma reset, /fullbright.";
/// The toggle key's name, and its default (none until the player picks one).
pub const KEY: &str = "toggle";
pub const DEFAULT_KEY: &str = "key.keyboard.unknown";
/// The gamma `/fullbright` and the toggle key turn on.
pub const FULLBRIGHT: f32 = 15.0;
/// The highest gamma `/gamma` takes (10000%).
pub const MAX: f32 = 100.0;
/// The gammas the settings page's button cycles through.
pub const PRESETS: [f32; 5] = [1.0, 2.0, 5.0, 10.0, 15.0];

/// The settings.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// The custom gamma is in effect (else the slider's).
    pub enabled: bool,
    /// The custom gamma: 1 is vanilla's "Bright" (100%).
    pub gamma: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            enabled: false,
            gamma: FULLBRIGHT,
        }
    }
}

impl Settings {
    /// From the stored values (`get` reads one key's file); the defaults
    /// (off, 15) for the ones missing or unreadable.
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let d = Self::default();
        let enabled = get("enabled").map_or(d.enabled, |v| v.trim() == "true");
        let gamma = get("gamma")
            .and_then(|v| v.trim().parse::<f32>().ok())
            .filter(|v| v.is_finite())
            .map_or(d.gamma, |v| v.clamp(0.0, MAX));
        Self { enabled, gamma }
    }
    /// The gamma in effect, none when the slider's is.
    pub fn active(&self) -> Option<f32> {
        self.enabled.then_some(self.gamma)
    }
    /// The settings page's buttons.
    pub fn labels(&self) -> Vec<String> {
        let on = if self.enabled { "ON" } else { "OFF" };
        vec![
            format!("Custom gamma: {on}"),
            format!("Gamma: {}", percent(self.gamma)),
        ]
    }
    /// A click on button `i` of `labels`.
    pub fn click(&mut self, i: usize) {
        match i {
            0 => self.enabled = !self.enabled,
            1 => {
                // The next preset above the current gamma, else the first.
                self.gamma = PRESETS
                    .iter()
                    .copied()
                    .find(|&p| p > self.gamma + 1e-3)
                    .unwrap_or(PRESETS[0]);
                self.enabled = true;
            }
            _ => {}
        }
    }
    /// What to store, key by key.
    pub fn stored(&self) -> [(&'static str, String); 2] {
        [
            ("enabled", self.enabled.to_string()),
            ("gamma", self.gamma.to_string()),
        ]
    }
}

/// `1.5` as `150%`.
pub fn percent(gamma: f32) -> String {
    format!("{}%", (gamma * 100.0).round() as i64)
}

/// A gamma as the player types it: `15`, `1.5`, or a percentage `1500%`.
fn parse(arg: &str) -> Option<f32> {
    let v = match arg.strip_suffix('%') {
        Some(p) => p.parse::<f32>().ok()? / 100.0,
        None => arg.parse::<f32>().ok()?,
    };
    (v.is_finite() && (0.0..=MAX).contains(&v)).then_some(v)
}

/// Runs `command` (without its slash) when it is one of the mod's: the
/// settings after it and the line to show in chat. None for other commands.
/// `slider` is the video settings' gamma, for `/gamma` to report.
pub fn run(before: Settings, command: &str, slider: f32) -> Option<(Settings, String)> {
    let mut words = command.split_whitespace();
    let name = words.next()?.to_ascii_lowercase();
    let arg = words.next().map(str::to_ascii_lowercase);
    let mut s = before;
    let message = match (name.as_str(), arg.as_deref()) {
        ("fullbright", _) => {
            if s.enabled && s.gamma >= FULLBRIGHT {
                s.enabled = false;
                format!("[Fullbright] OFF, brightness back to {}", percent(slider))
            } else {
                s.gamma = s.gamma.max(FULLBRIGHT);
                s.enabled = true;
                format!("[Fullbright] ON, gamma {}", percent(s.gamma))
            }
        }
        ("gamma", None) => match s.active() {
            Some(g) => format!(
                "[Fullbright] Gamma is {} (custom). /gamma reset gives the slider's {} back.",
                percent(g),
                percent(slider)
            ),
            None => format!(
                "[Fullbright] Gamma is {} (video settings). /gamma <value> sets your own, e.g. /gamma 15.",
                percent(slider)
            ),
        },
        ("gamma", Some("reset" | "off")) => {
            s.enabled = false;
            format!("[Fullbright] Gamma reset to the video settings' {}", percent(slider))
        }
        ("gamma", Some("on")) => {
            s.enabled = true;
            format!("[Fullbright] Gamma {}", percent(s.gamma))
        }
        ("gamma", Some(value)) => match parse(value) {
            Some(g) => {
                s.gamma = g;
                s.enabled = true;
                format!("[Fullbright] Gamma set to {}", percent(g))
            }
            None => format!(
                "[Fullbright] Not a gamma: {value}. Use a number from 0 to {} (1 = Bright, 15 = fullbright) or a percentage like 1500%.",
                MAX
            ),
        },
        _ => return None,
    };
    Some((s, message))
}

thread_local! {
    /// Read on first use, then kept.
    static STATE: RefCell<Option<Settings>> = const { RefCell::new(None) };
}

/// The settings, read once through `get`.
pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    STATE.with_borrow_mut(|s| *s.get_or_insert_with(|| Settings::read(get)))
}

/// Replaces the settings (a command, the key or the settings page changed them).
pub fn set_settings(settings: Settings) {
    STATE.with_borrow_mut(|s| *s = Some(settings));
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
        let s = Settings::read(stored(&[("enabled", "true\n"), ("gamma", "7.5")]));
        assert_eq!(s, Settings { enabled: true, gamma: 7.5 });
        assert_eq!(Settings::read(stored(&[("gamma", "999")])).gamma, MAX);
        assert_eq!(Settings::read(stored(&[("gamma", "NaN")])).gamma, FULLBRIGHT);
        let round: Vec<(&str, String)> = s.stored().into();
        let back: Vec<(&str, &str)> = round.iter().map(|(k, v)| (*k, v.as_str())).collect();
        assert_eq!(Settings::read(stored(&back)), s);
    }

    #[test]
    fn gamma_command() {
        let off = Settings::default();
        let (s, m) = run(off, "gamma 2.5", 0.5).unwrap();
        assert_eq!(s.active(), Some(2.5));
        assert!(m.contains("250%"), "{m}");
        let (s, _) = run(off, "gamma 1500%", 0.5).unwrap();
        assert_eq!(s.active(), Some(15.0));
        let (s, m) = run(s, "gamma reset", 0.5).unwrap();
        assert_eq!(s.active(), None);
        assert!(m.contains("50%"), "{m}");
        let (same, m) = run(off, "gamma lots", 0.5).unwrap();
        assert_eq!(same, off);
        assert!(m.starts_with("[Fullbright] Not a gamma"), "{m}");
        assert_eq!(run(off, "gamma -1", 0.5).unwrap().0, off);
        assert_eq!(run(off, "gamma", 0.5).unwrap().0, off);
        assert_eq!(run(off, "GAMMA 3", 0.5).unwrap().0.active(), Some(3.0));
    }

    #[test]
    fn fullbright_toggles() {
        let (s, _) = run(Settings::default(), "fullbright", 1.0).unwrap();
        assert_eq!(s.active(), Some(FULLBRIGHT));
        let (s, _) = run(s, "fullbright", 1.0).unwrap();
        assert_eq!(s.active(), None);
        // A custom gamma below fullbright turns into fullbright, not off.
        let low = Settings { enabled: true, gamma: 2.0 };
        assert_eq!(run(low, "fullbright", 1.0).unwrap().0.active(), Some(FULLBRIGHT));
    }

    #[test]
    fn leaves_other_commands() {
        assert!(run(Settings::default(), "gamemode creative", 1.0).is_none());
        assert!(run(Settings::default(), "", 1.0).is_none());
        assert!(run(Settings::default(), "gammas 2", 1.0).is_none());
    }

    #[test]
    fn settings_page_buttons() {
        let mut s = Settings { enabled: false, gamma: 1.0 };
        s.click(1);
        assert_eq!(s, Settings { enabled: true, gamma: 2.0 });
        s.gamma = 15.0;
        s.click(1);
        assert_eq!(s.gamma, 1.0);
        s.click(0);
        assert_eq!(s.labels(), ["Custom gamma: OFF", "Gamma: 100%"]);
    }
}
