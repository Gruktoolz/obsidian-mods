// ArmorStatusHUD native mod for Horizon: displays equipped armor pieces and
// held items with their true 1:1 Java durability color coding (Green > 50%,
// Yellow 25-50%, Orange 10-25%, Red < 10%) and configurable positioning.
// This file is both the standalone Cargo library (for unit tests) and the
// runtime module injected into `crates/horizon-client/src/game.rs`.
use std::cell::RefCell;

pub const ID: &str = "armor_status";
pub const NAME: &str = "Armor Status";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Displays equipped armor and held items with precise durability and 1:1 Java original color coding (Green/Yellow/Orange/Red).";

/// Original Java ArmorStatusHUD color constants.
pub const COLOR_GREEN: u32 = 0x55_FF55;   // > 50% durability
pub const COLOR_YELLOW: u32 = 0xFF_FF55;  // 25% - 50% durability
pub const COLOR_ORANGE: u32 = 0xFF_AA00;  // 10% - 25% durability
pub const COLOR_RED: u32 = 0xFF_5555;     // < 10% durability
pub const COLOR_COUNT: u32 = 0xFF_FFFF;   // Non-damageable item stack count
pub const COLOR_DIM: u32 = 0xAA_AAAA;     // Empty / placeholder
pub const BACKGROUND: u32 = 0x7800_0000;  // Translucent dark box

pub const POSITIONS: [&str; 6] = [
    "Hotbar left",
    "Hotbar right",
    "Top left",
    "Bottom left",
    "Top right",
    "Bottom right",
];

pub const FORMATS: [&str; 3] = [
    "Remaining (1520)",
    "Percent (97%)",
    "Fraction (1520/1561)",
];

pub const HANDS: [&str; 4] = [
    "Main & off hand",
    "Main hand only",
    "Off hand only",
    "Off (Armor only)",
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// Index in `POSITIONS`. Default: 0 (Hotbar left).
    pub position: usize,
    /// Index in `FORMATS`. Default: 0 (Remaining value).
    pub format: usize,
    /// Index in `HANDS`. Default: 0 (Both hands).
    pub hands: usize,
    /// Draw a translucent dark background box behind items.
    pub background: bool,
    /// Show stack count for items without durability (e.g. 64 torches).
    pub show_count: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            position: 5, // Default: Bottom right (unten rechts)
            format: 0,
            hands: 0,
            background: true,
            show_count: true,
        }
    }
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let position = get("position")
            .and_then(|v| v.trim().parse::<usize>().ok())
            .map_or(5, |p| p % POSITIONS.len());
        let format = get("format")
            .and_then(|v| v.trim().parse::<usize>().ok())
            .map_or(0, |f| f % FORMATS.len());
        let hands = get("hands")
            .and_then(|v| v.trim().parse::<usize>().ok())
            .map_or(0, |h| h % HANDS.len());
        let flag = |key: &str, def: bool| {
            get(key).map_or(def, |v| v.trim() != "false")
        };
        Self {
            position,
            format,
            hands,
            background: flag("background", true),
            show_count: flag("show_count", true),
        }
    }

    pub fn labels(&self) -> Vec<String> {
        let on = |b: bool| if b { "ON" } else { "OFF" };
        vec![
            format!("Position: {}", POSITIONS[self.position]),
            format!("Durability: {}", FORMATS[self.format]),
            format!("Hand items: {}", HANDS[self.hands]),
            format!("Box background: {}", on(self.background)),
            format!("Show stack count: {}", on(self.show_count)),
        ]
    }

    pub fn click(&mut self, i: usize) {
        match i {
            0 => self.position = (self.position + 1) % POSITIONS.len(),
            1 => self.format = (self.format + 1) % FORMATS.len(),
            2 => self.hands = (self.hands + 1) % HANDS.len(),
            3 => self.background = !self.background,
            4 => self.show_count = !self.show_count,
            _ => {}
        }
    }

    pub fn stored(&self) -> [(&'static str, String); 5] {
        let flag = |b: bool| if b { "true" } else { "false" }.to_owned();
        [
            ("position", self.position.to_string()),
            ("format", self.format.to_string()),
            ("hands", self.hands.to_string()),
            ("background", flag(self.background)),
            ("show_count", flag(self.show_count)),
        ]
    }
}

thread_local! {
    static SETTINGS: RefCell<Option<Settings>> = const { RefCell::new(None) };
}

pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    SETTINGS.with_borrow_mut(|s| *s.get_or_insert_with(|| Settings::read(get)))
}

pub fn set_settings(new_settings: Settings) {
    SETTINGS.with_borrow_mut(|s| *s = Some(new_settings));
}

/// Calculates the exact 1:1 Java ArmorStatusHUD color for a given durability ratio.
pub fn durability_color(ratio: f32) -> u32 {
    if !ratio.is_finite() || ratio < 0.10 {
        COLOR_RED
    } else if ratio < 0.25 {
        COLOR_ORANGE
    } else if ratio <= 0.50 {
        COLOR_YELLOW
    } else {
        COLOR_GREEN
    }
}

/// Formats the durability text according to the selected format index.
pub fn format_durability(remaining: u32, max_damage: u32, format: usize) -> String {
    match format {
        1 => {
            let pct = if max_damage > 0 {
                ((remaining as f32 / max_damage as f32) * 100.0).round() as u32
            } else {
                100
            };
            format!("{pct}%")
        }
        2 => format!("{remaining}/{max_damage}"),
        _ => remaining.to_string(),
    }
}

/// Evaluates an item stack's display text and color for ArmorStatusHUD.
/// Returns `Some((text, color))` if text should be displayed beside the icon.
pub fn format_item(
    count: u32,
    damage: u32,
    max_damage: u32,
    format: usize,
    show_count: bool,
) -> Option<(String, u32)> {
    if max_damage > 0 {
        let remaining = max_damage.saturating_sub(damage);
        let ratio = remaining as f32 / max_damage as f32;
        let color = durability_color(ratio);
        let text = format_durability(remaining, max_damage, format);
        Some((text, color))
    } else if show_count && count > 1 {
        Some((count.to_string(), COLOR_COUNT))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stored<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        |key| pairs.iter().find(|p| p.0 == key).map(|p| p.1.to_owned())
    }

    #[test]
    fn reads_and_writes_settings_correctly() {
        let defaults = Settings::read(stored(&[]));
        assert_eq!(defaults, Settings::default());
        assert_eq!(defaults.position, 5); // Bottom right (unten rechts)

        let custom = Settings::read(stored(&[
            ("position", "1"),
            ("format", "2"),
            ("hands", "3"),
            ("background", "false"),
            ("show_count", "false"),
        ]));
        assert_eq!(
            custom,
            Settings {
                position: 1,
                format: 2,
                hands: 3,
                background: false,
                show_count: false,
            }
        );

        let pairs = custom.stored();
        let refs: Vec<(&str, &str)> = pairs.iter().map(|(k, v)| (*k, v.as_str())).collect();
        assert_eq!(Settings::read(stored(&refs)), custom);
    }

    #[test]
    fn button_clicks_cycle_options() {
        let mut s = Settings::default();
        assert_eq!(s.position, 5); // Default is Bottom right
        s.click(0); // position cycles -> Hotbar left (0)
        assert_eq!(s.position, 0);
        s.click(1); // format -> Percent
        assert_eq!(s.format, 1);
        s.click(2); // hands -> Main hand only
        assert_eq!(s.hands, 1);
        s.click(3); // background -> false
        assert!(!s.background);
        s.click(4); // show_count -> false
        assert!(!s.show_count);
    }

    #[test]
    fn original_java_color_thresholds_match_parity() {
        // > 50%: Green
        assert_eq!(durability_color(1.0), COLOR_GREEN);
        assert_eq!(durability_color(0.51), COLOR_GREEN);

        // 25% - 50%: Yellow
        assert_eq!(durability_color(0.50), COLOR_YELLOW);
        assert_eq!(durability_color(0.25), COLOR_YELLOW);

        // 10% - 25%: Orange
        assert_eq!(durability_color(0.24), COLOR_ORANGE);
        assert_eq!(durability_color(0.10), COLOR_ORANGE);

        // < 10%: Red
        assert_eq!(durability_color(0.09), COLOR_RED);
        assert_eq!(durability_color(0.01), COLOR_RED);
        assert_eq!(durability_color(0.00), COLOR_RED);
    }

    #[test]
    fn formats_durability_styles() {
        assert_eq!(format_durability(1500, 1561, 0), "1500");
        assert_eq!(format_durability(1500, 1561, 1), "96%");
        assert_eq!(format_durability(1500, 1561, 2), "1500/1561");
    }

    #[test]
    fn item_formatting_handles_durability_and_stacks() {
        // Diamond sword at 1500/1561 damage
        let sword = format_item(1, 61, 1561, 0, true);
        assert_eq!(sword, Some(("1500".into(), COLOR_GREEN)));

        // Critical diamond sword at 50/1561 damage
        let low_sword = format_item(1, 1511, 1561, 0, true);
        assert_eq!(low_sword, Some(("50".into(), COLOR_RED)));

        // Non-damageable stack (64 golden apples)
        let stack = format_item(64, 0, 0, 0, true);
        assert_eq!(stack, Some(("64".into(), COLOR_COUNT)));

        // Single item without damage -> no count displayed
        let single = format_item(1, 0, 0, 0, true);
        assert_eq!(single, None);
    }
}
