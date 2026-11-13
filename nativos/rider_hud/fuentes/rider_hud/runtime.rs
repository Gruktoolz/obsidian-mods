// Rider HUD - Native Horizon Mod
// Displays mount health, player hunger, and smart jump/exp bars simultaneously while riding.

pub const ID: &str = "rider_hud";
pub const NAME: &str = "Rider HUD";
pub const VERSION: &str = "0.1.0";
pub const DESCRIPTION: &str =
    "Shows mount health, player hunger, and smart jump/exp bars simultaneously while riding.";

/// Vertical pixel offset applied to vehicle health rendering.
///
/// Vanilla `vehicle_health(w, h, hearts, health)` starts drawing at `y = h - 39`.
/// By subtracting 10 pixels from `h`, the base row begins at `(h - 10) - 39 = h - 49`,
/// leaving `y = h - 39` free for the player hunger bar.
pub const VEHICLE_HEALTH_Y_OFFSET: i32 = 10;

/// Standard base Y coordinate for the first HUD row above the hotbar (e.g. food).
#[allow(dead_code)]
pub const HUD_BASE_ROW_Y_OFFSET: i32 = 39;

/// Vertical height in pixels of one HUD icon row (hearts, food, air bubbles).
pub const HUD_ROW_STEP: i32 = 10;

/// Computes the number of vertical rows required for vehicle hearts.
///
/// In Minecraft, 1 row holds up to 10 hearts (20 health points).
/// - 0 hearts -> 0 rows.
/// - 1..=10 hearts -> 1 row.
/// - 11..=20 hearts -> 2 rows.
/// - 21..=30 hearts -> 3 rows.
#[inline]
pub fn vehicle_heart_rows(hearts: i32) -> i32 {
    if hearts <= 0 { 0 } else { (hearts + 9) / 10 }
}

/// Computes the vertical pixel shift for air bubbles so they sit neatly above vehicle hearts.
///
/// When riding a vehicle, mount health starts at `h - 49` (one row above player food at `h - 39`).
/// If the vehicle has `N` rows of hearts, air bubbles must be shifted up by `N * 10` pixels
/// to avoid colliding with mount hearts.
#[inline]
pub fn air_bubbles_shift_y(vehicle_hearts: i32) -> i32 {
    vehicle_heart_rows(vehicle_hearts) * HUD_ROW_STEP
}

/// Returns the expected Y coordinate of the player food row for screen height `h`.
#[allow(dead_code)]
#[inline]
pub fn food_row_y(h: i32) -> i32 {
    h - HUD_BASE_ROW_Y_OFFSET
}

/// Returns the expected Y coordinate of vehicle heart row `row_index` (0-based) for screen height `h`.
#[allow(dead_code)]
#[inline]
pub fn vehicle_health_row_y(h: i32, row_index: i32) -> i32 {
    (h - VEHICLE_HEALTH_Y_OFFSET) - HUD_BASE_ROW_Y_OFFSET - (row_index * HUD_ROW_STEP)
}

/// Returns the expected Y coordinate of the air bubbles row for screen height `h` and given `vehicle_hearts`.
#[allow(dead_code)]
#[inline]
pub fn air_bubbles_row_y(h: i32, vehicle_hearts: i32) -> i32 {
    let base_air_y = h - HUD_BASE_ROW_Y_OFFSET - HUD_ROW_STEP; // h - 49
    base_air_y - air_bubbles_shift_y(vehicle_hearts)
}

/// Checks whether the contextual info bar should display the experience bar instead of the jump bar.
///
/// Returns `true` when riding a jumpable mount, but the player is NOT actively charging jump,
/// creative mode is off, and no locator waypoints are active.
#[inline]
pub fn should_show_experience_instead_of_jump(
    is_riding_jumpable: bool,
    is_charging_jump: bool,
    has_waypoints: bool,
    is_creative: bool,
) -> bool {
    !is_creative && is_riding_jumpable && !is_charging_jump && !has_waypoints
}

/// Parses a boolean configuration value from string storage.
pub fn read_bool_setting(stored: Option<String>, default_val: bool) -> bool {
    match stored {
        Some(v) => {
            let s = v.trim();
            if s.eq_ignore_ascii_case("true") || s == "1" || s.eq_ignore_ascii_case("on") {
                true
            } else if s.eq_ignore_ascii_case("false") || s == "0" || s.eq_ignore_ascii_case("off") {
                false
            } else {
                default_val
            }
        }
        None => default_val,
    }
}

pub fn enabled_label(enabled: bool) -> String {
    format!("Rider HUD: {}", if enabled { "ON" } else { "OFF" })
}

pub fn hunger_label(show: bool) -> String {
    format!("Hunger While Riding: {}", if show { "ON" } else { "OFF" })
}

pub fn exp_label(show: bool) -> String {
    format!(
        "Experience While Riding: {}",
        if show { "ON" } else { "OFF" }
    )
}

thread_local! {
    static ENABLED: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
    static SHOW_HUNGER: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
    static SHOW_EXP: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
}

pub fn is_enabled(get: impl FnOnce() -> Option<String>) -> bool {
    ENABLED.with(|e| {
        let val = e.get().unwrap_or_else(|| read_bool_setting(get(), true));
        e.set(Some(val));
        val
    })
}

pub fn set_enabled(enabled: bool) {
    ENABLED.with(|e| e.set(Some(enabled)));
}

pub fn is_show_hunger(get: impl FnOnce() -> Option<String>) -> bool {
    SHOW_HUNGER.with(|e| {
        let val = e.get().unwrap_or_else(|| read_bool_setting(get(), true));
        e.set(Some(val));
        val
    })
}

pub fn set_show_hunger(show: bool) {
    SHOW_HUNGER.with(|e| e.set(Some(show)));
}

pub fn is_show_exp(get: impl FnOnce() -> Option<String>) -> bool {
    SHOW_EXP.with(|e| {
        let val = e.get().unwrap_or_else(|| read_bool_setting(get(), true));
        e.set(Some(val));
        val
    })
}

pub fn set_show_exp(show: bool) {
    SHOW_EXP.with(|e| e.set(Some(show)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vehicle_heart_rows() {
        assert_eq!(vehicle_heart_rows(-5), 0);
        assert_eq!(vehicle_heart_rows(0), 0);
        assert_eq!(vehicle_heart_rows(1), 1);
        assert_eq!(vehicle_heart_rows(5), 1);
        assert_eq!(vehicle_heart_rows(10), 1);
        assert_eq!(vehicle_heart_rows(11), 2);
        assert_eq!(vehicle_heart_rows(15), 2);
        assert_eq!(vehicle_heart_rows(20), 2);
        assert_eq!(vehicle_heart_rows(21), 3);
        assert_eq!(vehicle_heart_rows(30), 3);
    }

    #[test]
    fn test_air_bubbles_shift() {
        assert_eq!(air_bubbles_shift_y(0), 0);
        assert_eq!(air_bubbles_shift_y(6), 10);
        assert_eq!(air_bubbles_shift_y(10), 10);
        assert_eq!(air_bubbles_shift_y(14), 20);
        assert_eq!(air_bubbles_shift_y(20), 20);
        assert_eq!(air_bubbles_shift_y(26), 30);
    }

    #[test]
    fn test_hud_row_coordinates() {
        let h = 240;
        // Food row is at h - 39 = 201
        assert_eq!(food_row_y(h), 201);

        // Mount health row 0 is at h - 49 = 191
        assert_eq!(vehicle_health_row_y(h, 0), 191);
        // Mount health row 1 is at h - 59 = 181
        assert_eq!(vehicle_health_row_y(h, 1), 181);

        // Air bubbles when on foot (0 mount hearts): at h - 49 = 191
        assert_eq!(air_bubbles_row_y(h, 0), 191);

        // Air bubbles when riding a 10-heart mount: at h - 59 = 181
        assert_eq!(air_bubbles_row_y(h, 10), 181);

        // Air bubbles when riding a 20-heart mount: at h - 69 = 171
        assert_eq!(air_bubbles_row_y(h, 20), 171);

        // Verify strictly non-overlapping hierarchy
        assert!(food_row_y(h) > vehicle_health_row_y(h, 0));
        assert!(vehicle_health_row_y(h, 0) > air_bubbles_row_y(h, 10));
    }

    #[test]
    fn test_contextual_bar_selection() {
        // Normal riding on jumpable mount: should show experience
        assert!(should_show_experience_instead_of_jump(
            true, false, false, false
        ));

        // Actively charging jump: should show jump bar (not experience)
        assert!(!should_show_experience_instead_of_jump(
            true, true, false, false
        ));

        // Walking on foot: not riding jumpable, vanilla handles it
        assert!(!should_show_experience_instead_of_jump(
            false, false, false, false
        ));

        // Waypoints active: should keep locator bar
        assert!(!should_show_experience_instead_of_jump(
            true, false, true, false
        ));

        // Creative mode: creative does not show experience bar
        assert!(!should_show_experience_instead_of_jump(
            true, false, false, true
        ));
    }

    #[test]
    fn test_settings_labels() {
        assert_eq!(enabled_label(true), "Rider HUD: ON");
        assert_eq!(enabled_label(false), "Rider HUD: OFF");
        assert_eq!(hunger_label(true), "Hunger While Riding: ON");
        assert_eq!(hunger_label(false), "Hunger While Riding: OFF");
        assert_eq!(exp_label(true), "Experience While Riding: ON");
        assert_eq!(exp_label(false), "Experience While Riding: OFF");
    }

    #[test]
    fn test_read_bool_setting() {
        assert!(read_bool_setting(Some("true".into()), false));
        assert!(read_bool_setting(Some("TRUE".into()), false));
        assert!(read_bool_setting(Some("1".into()), false));
        assert!(read_bool_setting(Some("on".into()), false));

        assert!(!read_bool_setting(Some("false".into()), true));
        assert!(!read_bool_setting(Some("FALSE".into()), true));
        assert!(!read_bool_setting(Some("0".into()), true));
        assert!(!read_bool_setting(Some("off".into()), true));

        // Fallbacks
        assert!(read_bool_setting(None, true));
        assert!(!read_bool_setting(None, false));
        assert!(read_bool_setting(Some("invalid".into()), true));
        assert!(!read_bool_setting(Some("invalid".into()), false));
    }

    #[test]
    fn test_settings_toggling() {
        set_enabled(true);
        assert!(is_enabled(|| None));
        set_enabled(false);
        assert!(!is_enabled(|| None));
        set_enabled(true);

        set_show_hunger(true);
        assert!(is_show_hunger(|| None));
        set_show_hunger(false);
        assert!(!is_show_hunger(|| None));
        set_show_hunger(true);

        set_show_exp(true);
        assert!(is_show_exp(|| None));
        set_show_exp(false);
        assert!(!is_show_exp(|| None));
        set_show_exp(true);
    }
}
