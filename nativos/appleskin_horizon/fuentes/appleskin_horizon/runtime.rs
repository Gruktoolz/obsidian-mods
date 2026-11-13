// Pure runtime logic for AppleSkin mod.
// 1:1 parity with Java AppleSkin (squeek502) saturation HUD overlay.
use std::cell::RefCell;

pub const ID: &str = "appleskin_horizon";
pub const NAME: &str = "AppleSkin";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Hunger bar saturation HUD overlay with 1:1 Java AppleSkin parity.";

/// Exact 1:1 pixel art coordinates and ARGB colors from AppleSkin's `icons.png`.
/// Full saturation shank overlay (9x9 pixels)
pub const FULL_PIXELS: &[(f32, f32, u32)] = &[
    (2.0, 0.0, 0xFFE9D262),
    (3.0, 0.0, 0xFFE9D262),
    (1.0, 1.0, 0xFFDDC241),
    (4.0, 1.0, 0xFFDDC241),
    (0.0, 2.0, 0xFFC2A100),
    (5.0, 2.0, 0xFFC2A100),
    (0.0, 3.0, 0xFFC2A100),
    (6.0, 3.0, 0xFFC2A100),
    (1.0, 4.0, 0xFF9F8609),
    (6.0, 4.0, 0xFFC2A100),
    (2.0, 5.0, 0xFF9F8609),
    (6.0, 5.0, 0xFFC2A100),
    (3.0, 6.0, 0xFF9F8609),
    (4.0, 6.0, 0xFF9F8609),
    (5.0, 6.0, 0xFF9F8609),
    (7.0, 6.0, 0xFFC2A100),
    (8.0, 6.0, 0xFFC2A100),
    (6.0, 7.0, 0xFF9F8609),
    (8.0, 7.0, 0xFFC2A100),
    (6.0, 8.0, 0xFF9F8609),
    (7.0, 8.0, 0xFF9F8609),
];

/// Half saturation shank overlay (9x9 pixels)
pub const HALF_PIXELS: &[(f32, f32, u32)] = &[
    (2.0, 0.0, 0xFFE9D262),
    (3.0, 0.0, 0xFFE9D262),
    (4.0, 1.0, 0xFFDDC241),
    (5.0, 2.0, 0xFFC2A100),
    (6.0, 3.0, 0xFFC2A100),
    (6.0, 4.0, 0xFFC2A100),
    (2.0, 5.0, 0xFF9F8609),
    (6.0, 5.0, 0xFFC2A100),
    (3.0, 6.0, 0xFF9F8609),
    (4.0, 6.0, 0xFF9F8609),
    (5.0, 6.0, 0xFF9F8609),
    (7.0, 6.0, 0xFFC2A100),
    (8.0, 6.0, 0xFFC2A100),
    (6.0, 7.0, 0xFF9F8609),
    (8.0, 7.0, 0xFFC2A100),
    (6.0, 8.0, 0xFF9F8609),
    (7.0, 8.0, 0xFF9F8609),
];

/// Quarter saturation shank overlay (9x9 pixels)
pub const QUARTER_PIXELS: &[(f32, f32, u32)] = &[
    (3.0, 0.0, 0xFFE9D262),
    (4.0, 1.0, 0xFFDDC241),
    (5.0, 2.0, 0xFFC2A100),
    (6.0, 3.0, 0xFFC2A100),
    (6.0, 4.0, 0xFFC2A100),
    (6.0, 5.0, 0xFFC2A100),
    (4.0, 6.0, 0xFF9F8609),
    (5.0, 6.0, 0xFF9F8609),
    (7.0, 6.0, 0xFFC2A100),
    (8.0, 6.0, 0xFFC2A100),
    (6.0, 7.0, 0xFF9F8609),
    (8.0, 7.0, 0xFFC2A100),
    (6.0, 8.0, 0xFF9F8609),
    (7.0, 8.0, 0xFF9F8609),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaturationLevel {
    None,
    Quarter,
    Half,
    Full,
}

impl SaturationLevel {
    pub fn pixels(self) -> &'static [(f32, f32, u32)] {
        match self {
            SaturationLevel::Full => FULL_PIXELS,
            SaturationLevel::Half => HALF_PIXELS,
            SaturationLevel::Quarter => QUARTER_PIXELS,
            SaturationLevel::None => &[],
        }
    }
}

/// Calculate saturation overlay level for drumstick `index` (0..10, 0 = rightmost).
///
/// Matches AppleSkin Java:
///   `effectiveSaturationOfBar = (saturation / 2.0) - index`
///   `>= 1.0  => Full`
///   `>  0.5  => Half`
///   `>  0.25 => Quarter`
pub fn drumstick_saturation_level(index: usize, saturation: f32) -> SaturationLevel {
    if index >= 10 || saturation <= 0.0 {
        return SaturationLevel::None;
    }
    let effective = (saturation / 2.0) - (index as f32);
    if effective >= 1.0 {
        SaturationLevel::Full
    } else if effective > 0.5 {
        SaturationLevel::Half
    } else if effective > 0.25 {
        SaturationLevel::Quarter
    } else {
        SaturationLevel::None
    }
}

/// Calculate drumstick origin (x_right, y_base) matching Horizon's `hud_layout.rs`.
pub fn drumstick_origin(width: f32, height: f32) -> (i32, i32) {
    let x_right = ((width / 2.0).floor() as i32) + 91;
    let y_base = (height as i32) - 39;
    (x_right, y_base)
}

/// Calculate top-left (x, y) for drumstick `n` (0..10, 0 = rightmost).
pub fn drumstick_pos(x_right: i32, y_base: i32, n: usize) -> (f32, f32) {
    let x = (x_right - (n as i32) * 8 - 9) as f32;
    let y = y_base as f32;
    (x, y)
}

/// Calculate saturation restored by food (Java parity: nutrition * modifier * 2.0).
pub fn calculate_saturation_restore(
    nutrition: u32,
    saturation_modifier: f32,
    current_food: i32,
    current_saturation: f32,
) -> f32 {
    let sat_restored = nutrition as f32 * saturation_modifier * 2.0;
    let target_food = (current_food + nutrition as i32).clamp(0, 20);
    (current_saturation + sat_restored).clamp(0.0, target_food as f32)
}

/// Pulsing alpha calculation for preview blinking effect.
pub fn pulse_alpha(millis: u64) -> f32 {
    let cycle = (millis % 800) as f32 / 800.0;
    let wave = ((cycle * std::f32::consts::PI * 2.0).sin() * 0.5) + 0.5;
    0.35 + (0.55 * wave)
}

/// Settings: only the toggle for Sättigungs-Overlay (clean and simple).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Settings {
    pub saturation_overlay: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            saturation_overlay: true,
        }
    }
}

impl Settings {
    pub fn labels(&self) -> Vec<String> {
        vec![format!(
            "Sättigungs-Overlay: {}",
            if self.saturation_overlay {
                "An"
            } else {
                "Aus"
            }
        )]
    }

    pub fn click(&mut self, button: usize) {
        if button == 0 {
            self.saturation_overlay = !self.saturation_overlay;
        }
    }

    pub fn stored(&self) -> Vec<(&'static str, String)> {
        vec![(
            "saturation_overlay",
            (if self.saturation_overlay { "1" } else { "0" }).to_string(),
        )]
    }

    pub fn from_loader<F: Fn(&str) -> Option<String>>(loader: F) -> Self {
        let mut s = Self::default();
        if let Some(v) = loader("saturation_overlay") {
            s.saturation_overlay = v == "1" || v.eq_ignore_ascii_case("true");
        }
        s
    }
}

thread_local! {
    static SETTINGS: RefCell<Option<Settings>> = const { RefCell::new(None) };
}

pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    SETTINGS.with_borrow_mut(|s| *s.get_or_insert_with(|| Settings::from_loader(get)))
}

pub fn set_settings(new_settings: Settings) {
    SETTINGS.with_borrow_mut(|s| *s = Some(new_settings));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drumstick_saturation_level_parity() {
        // Saturation = 18.0 (Image 3 from user):
        // 9 drumsticks full, 1 empty
        for i in 0..9 {
            assert_eq!(
                drumstick_saturation_level(i, 18.0),
                SaturationLevel::Full,
                "drumstick {} should be full",
                i
            );
        }
        assert_eq!(
            drumstick_saturation_level(9, 18.0),
            SaturationLevel::None,
            "drumstick 9 should be empty"
        );

        // Saturation = 20.0: all 10 full
        for i in 0..10 {
            assert_eq!(drumstick_saturation_level(i, 20.0), SaturationLevel::Full);
        }

        // Saturation = 5.0:
        // i=0: 2.5 - 0 = 2.5 => Full
        // i=1: 2.5 - 1 = 1.5 => Full
        // i=2: 2.5 - 2 = 0.5 => Quarter (0.5 is not > 0.5)
        // i=3..10 => None
        assert_eq!(drumstick_saturation_level(0, 5.0), SaturationLevel::Full);
        assert_eq!(drumstick_saturation_level(1, 5.0), SaturationLevel::Full);
        assert_eq!(drumstick_saturation_level(2, 5.0), SaturationLevel::Quarter);
        assert_eq!(drumstick_saturation_level(3, 5.0), SaturationLevel::None);

        // Saturation = 5.5:
        // i=2: 2.75 - 2 = 0.75 => Half (> 0.5)
        assert_eq!(drumstick_saturation_level(2, 5.5), SaturationLevel::Half);
    }

    #[test]
    fn test_drumstick_layout_parity() {
        let (x_right, y_base) = drumstick_origin(320.0, 240.0);
        assert_eq!(x_right, 251);
        assert_eq!(y_base, 201);

        let (x0, y0) = drumstick_pos(x_right, y_base, 0);
        assert_eq!(x0, 242.0);
        assert_eq!(y0, 201.0);

        let (x9, y9) = drumstick_pos(x_right, y_base, 9);
        assert_eq!(x9, 170.0);
        assert_eq!(y9, 201.0);
    }

    #[test]
    fn test_pixel_counts() {
        assert_eq!(FULL_PIXELS.len(), 21);
        assert_eq!(HALF_PIXELS.len(), 17);
        assert_eq!(QUARTER_PIXELS.len(), 14);
    }

    #[test]
    fn test_settings_toggle() {
        let mut s = Settings::default();
        assert!(s.saturation_overlay);
        assert_eq!(s.labels().len(), 1);
        s.click(0);
        assert!(!s.saturation_overlay);
        s.click(0);
        assert!(s.saturation_overlay);
    }
}
