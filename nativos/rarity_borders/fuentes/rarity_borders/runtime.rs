// Rarity Borders runtime for Horizon Minecraft.
// Adds crisp, vanilla-styled pixel-perfect colored borders to inventory item slots
// based on Minecraft item rarity.

use std::sync::atomic::{AtomicBool, Ordering};

/// Mod identifier.
pub const ID: &str = "rarity_borders";

/// User-facing display name.
pub const NAME: &str = "Rarity Borders";

/// Semantic version.
pub const VERSION: &str = "0.1.1";

/// Mod description for in-game mod list.
pub const DESCRIPTION: &str =
    "Adds colored borders to inventory items based on their rarity in Horizon Minecraft.";

/// Master ON/OFF toggle (default: true).
static ENABLED: AtomicBool = AtomicBool::new(true);
static ENABLED_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Whether to show borders for Common items (default: false).
static SHOW_COMMON: AtomicBool = AtomicBool::new(false);
static SHOW_COMMON_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Whether to show borders in in-game HUD hotbar (default: false).
static SHOW_HOTBAR: AtomicBool = AtomicBool::new(false);
static SHOW_HOTBAR_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Vanilla-styled rarity colors (RGB).
pub const COLOR_COMMON: u32 = 0x94A3B8; // Slate Gray (neutral, subtle)
pub const COLOR_UNCOMMON: u32 = 0xFFFF55; // Soft Golden Yellow (vanilla uncommon)
pub const COLOR_RARE: u32 = 0x55FFFF; // Soft Sky Cyan / Aqua (vanilla rare)
pub const COLOR_EPIC: u32 = 0xFF55FF; // Soft Magenta / Light Purple (vanilla epic)

/// Official Minecraft item rarity categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}

impl std::str::FromStr for Rarity {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "uncommon" => Self::Uncommon,
            "rare" => Self::Rare,
            "epic" => Self::Epic,
            _ => Self::Common,
        })
    }
}

impl Rarity {
    /// Parses rarity string identifier from Minecraft / Horizon.
    pub fn parse_str(s: &str) -> Self {
        match s {
            "uncommon" => Self::Uncommon,
            "rare" => Self::Rare,
            "epic" => Self::Epic,
            _ => Self::Common,
        }
    }

    /// Converts rarity to its canonical string representation.
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Common => "common",
            Self::Uncommon => "uncommon",
            Self::Rare => "rare",
            Self::Epic => "epic",
        }
    }
}

/// Returns the RGB color value for the given rarity.
pub const fn rarity_color(rarity: Rarity) -> u32 {
    match rarity {
        Rarity::Common => COLOR_COMMON,
        Rarity::Uncommon => COLOR_UNCOMMON,
        Rarity::Rare => COLOR_RARE,
        Rarity::Epic => COLOR_EPIC,
    }
}

/// Resolves the border color from a raw rarity string produced by Horizon's `item_tooltip::rarity()`.
/// If the rarity is Common and `show_common` is false, returns `None`.
/// Otherwise returns `Some(color_rgb)`.
pub fn resolve_border_color_from_rarity_str(rarity_str: &str, show_common: bool) -> Option<u32> {
    let rarity = Rarity::parse_str(rarity_str);
    if !show_common && rarity == Rarity::Common {
        return None;
    }
    Some(rarity_color(rarity))
}

/// Evaluates whether a border should be rendered for an item slot decoration pass.
/// Directly models and encapsulates the real Horizon rendering pipeline used in `decorations.rs`.
///
/// Parameters:
/// - `is_master_enabled`: whether the mod is enabled
/// - `count`: `stack.count`
/// - `item_id`: `&*stack.item` (e.g. "minecraft:air", "minecraft:diamond_sword")
/// - `is_playing_hud`: whether current screen is `Screen::Playing`
/// - `show_hotbar`: whether in-game HUD hotbar borders are enabled
/// - `is_carried_cursor`: whether item is currently being carried/moved by cursor
/// - `rarity_str`: string slice returned by `crate::item_tooltip::rarity(stack, &default_rarity)`
/// - `show_common`: whether common borders are enabled
#[allow(clippy::too_many_arguments)]
pub fn evaluate_slot_border(
    is_master_enabled: bool,
    count: u32,
    item_id: &str,
    is_playing_hud: bool,
    show_hotbar: bool,
    is_carried_cursor: bool,
    rarity_str: &str,
    show_common: bool,
) -> Option<u32> {
    if !is_master_enabled {
        return None;
    }
    if count == 0 || item_id == "minecraft:air" || item_id.is_empty() {
        return None;
    }
    if is_playing_hud && !show_hotbar {
        return None;
    }
    if is_carried_cursor {
        return None;
    }
    resolve_border_color_from_rarity_str(rarity_str, show_common)
}

/// Full decoration render runner for a slot.
/// Evaluates the slot border, and if active, calls `draw(rect, argb)` for each of the 4 border rects.
/// Returns the number of draw calls issued (0 if slot is empty, disabled, or suppressed; 4 if rendered).
#[allow(clippy::too_many_arguments)]
pub fn render_slot_decoration<F>(
    x: f32,
    y: f32,
    is_master_enabled: bool,
    count: u32,
    item_id: &str,
    is_playing_hud: bool,
    show_hotbar: bool,
    is_carried_cursor: bool,
    rarity_str: &str,
    show_common: bool,
    draw: F,
) -> usize
where
    F: FnMut([f32; 4], u32),
{
    if let Some(color) = evaluate_slot_border(
        is_master_enabled,
        count,
        item_id,
        is_playing_hud,
        show_hotbar,
        is_carried_cursor,
        rarity_str,
        show_common,
    ) {
        render_slot_border(x, y, color, draw);
        4
    } else {
        0
    }
}

/// Computes the effective item rarity following official vanilla rules:
/// 1. Ominous banner is always Uncommon.
/// 2. Stacks with custom `minecraft:rarity` component use that rarity.
/// 3. Otherwise the item's default rarity from `items.json` is used.
/// 4. Any enchantments upgrade Common & Uncommon to Rare, Rare to Epic, Epic stays Epic.
#[allow(dead_code)]
pub fn resolve_rarity(
    default_rarity: &str,
    is_ominous_banner: bool,
    custom_rarity: Option<&str>,
    is_enchanted: bool,
) -> Rarity {
    let base = if is_ominous_banner {
        Rarity::Uncommon
    } else if let Some(r) = custom_rarity {
        Rarity::parse_str(r)
    } else {
        Rarity::parse_str(default_rarity)
    };

    if !is_enchanted {
        base
    } else {
        match base {
            Rarity::Common | Rarity::Uncommon => Rarity::Rare,
            Rarity::Rare => Rarity::Epic,
            Rarity::Epic => Rarity::Epic,
        }
    }
}

/// Checks if master toggle is enabled, synchronizing with saved settings if needed.
pub fn is_enabled(fallback: impl FnOnce() -> Option<bool>) -> bool {
    if !ENABLED_INITIALIZED.load(Ordering::Acquire) {
        if let Some(v) = fallback() {
            ENABLED.store(v, Ordering::Release);
        }
        ENABLED_INITIALIZED.store(true, Ordering::Release);
    }
    ENABLED.load(Ordering::Acquire)
}

/// Sets master enabled state.
pub fn set_enabled(value: bool) {
    ENABLED.store(value, Ordering::Release);
    ENABLED_INITIALIZED.store(true, Ordering::Release);
}

/// Toggles master enabled state.
#[allow(dead_code)]
pub fn toggle_enabled() -> bool {
    let next = !ENABLED.load(Ordering::Acquire);
    set_enabled(next);
    next
}

/// Checks if borders for common items are enabled.
pub fn is_show_common(fallback: impl FnOnce() -> Option<bool>) -> bool {
    if !SHOW_COMMON_INITIALIZED.load(Ordering::Acquire) {
        if let Some(v) = fallback() {
            SHOW_COMMON.store(v, Ordering::Release);
        }
        SHOW_COMMON_INITIALIZED.store(true, Ordering::Release);
    }
    SHOW_COMMON.load(Ordering::Acquire)
}

/// Sets whether common items should show borders.
pub fn set_show_common(value: bool) {
    SHOW_COMMON.store(value, Ordering::Release);
    SHOW_COMMON_INITIALIZED.store(true, Ordering::Release);
}

/// Toggles common items display.
#[allow(dead_code)]
pub fn toggle_show_common() -> bool {
    let next = !SHOW_COMMON.load(Ordering::Acquire);
    set_show_common(next);
    next
}

/// Checks if borders in the HUD hotbar are enabled.
pub fn is_show_hotbar(fallback: impl FnOnce() -> Option<bool>) -> bool {
    if !SHOW_HOTBAR_INITIALIZED.load(Ordering::Acquire) {
        if let Some(v) = fallback() {
            SHOW_HOTBAR.store(v, Ordering::Release);
        }
        SHOW_HOTBAR_INITIALIZED.store(true, Ordering::Release);
    }
    SHOW_HOTBAR.load(Ordering::Acquire)
}

/// Sets whether HUD hotbar items should show borders.
pub fn set_show_hotbar(value: bool) {
    SHOW_HOTBAR.store(value, Ordering::Release);
    SHOW_HOTBAR_INITIALIZED.store(true, Ordering::Release);
}

/// Toggles HUD hotbar items display.
#[allow(dead_code)]
pub fn toggle_show_hotbar() -> bool {
    let next = !SHOW_HOTBAR.load(Ordering::Acquire);
    set_show_hotbar(next);
    next
}

/// Helper to determine if a slot border should be rendered and what color it should take.
/// Kept for backwards compatibility and high-level decision tables.
#[allow(dead_code)]
pub fn should_render_border(
    is_master_enabled: bool,
    is_playing_hud: bool,
    show_hotbar: bool,
    is_carried_cursor: bool,
    is_empty_or_air: bool,
    rarity: Rarity,
    show_common: bool,
) -> Option<u32> {
    if !is_master_enabled {
        return None;
    }
    if is_empty_or_air {
        return None;
    }
    if is_playing_hud && !show_hotbar {
        return None;
    }
    if is_carried_cursor {
        return None;
    }
    if rarity == Rarity::Common && !show_common {
        return None;
    }
    Some(rarity_color(rarity))
}

#[cfg(test)]
pub fn reset_settings_for_tests() {
    ENABLED.store(true, Ordering::SeqCst);
    ENABLED_INITIALIZED.store(false, Ordering::SeqCst);

    SHOW_COMMON.store(false, Ordering::SeqCst);
    SHOW_COMMON_INITIALIZED.store(false, Ordering::SeqCst);

    SHOW_HOTBAR.store(false, Ordering::SeqCst);
    SHOW_HOTBAR_INITIALIZED.store(false, Ordering::SeqCst);
}

/// Settings button label for Master toggle with RU/EN localization.
pub fn label_master(enabled: bool, is_ru: bool) -> String {
    if is_ru {
        format!("Рамки редкости: {}", if enabled { "ВКЛ" } else { "ВЫКЛ" })
    } else {
        format!("Rarity Borders: {}", if enabled { "ON" } else { "OFF" })
    }
}

/// Settings button label for Common items toggle with RU/EN localization.
pub fn label_common(show_common: bool, is_ru: bool) -> String {
    if is_ru {
        format!(
            "Рамки обычных предметов: {}",
            if show_common { "ВКЛ" } else { "ВЫКЛ" }
        )
    } else {
        format!(
            "Show Common Borders: {}",
            if show_common { "ON" } else { "OFF" }
        )
    }
}

/// Settings button label for Hotbar toggle with RU/EN localization.
pub fn label_hotbar(show_hotbar: bool, is_ru: bool) -> String {
    if is_ru {
        format!(
            "Рамки в хотбаре: {}",
            if show_hotbar { "ВКЛ" } else { "ВЫКЛ" }
        )
    } else {
        format!(
            "Show Hotbar Borders: {}",
            if show_hotbar { "ON" } else { "OFF" }
        )
    }
}

/// Builds the 4 pixel-perfect boundary lines of the 18x18 slot frame:
/// - Top: 1px at 30% alpha (subtle frame line)
/// - Left: 1px at 40% alpha (subtle frame line)
/// - Right: 1px at 40% alpha (subtle frame line)
/// - Bottom: 1px at 100% solid alpha (prominent vibrant bottom accent)
///
/// Parameters:
/// - `x, y`: Top-left coordinates of the 16x16 item icon inside the 18x18 slot.
/// - `color_rgb`: 24-bit RGB color of the rarity.
pub fn slot_border_rects(x: f32, y: f32, color_rgb: u32) -> [([f32; 4], u32); 4] {
    let rgb = color_rgb & 0x00FF_FFFF;

    // Alpha values:
    // Top subtle line: 0x4D (~30% alpha)
    let argb_top = (0x4D << 24) | rgb;
    // Left & Right subtle lines: 0x66 (~40% alpha)
    let argb_sides = (0x66 << 24) | rgb;
    // Bottom prominent accent line: 0xFF (100% solid alpha)
    let argb_bottom = (0xFF << 24) | rgb;

    [
        // Top 1px line (spanning full 18px slot width)
        ([x - 1.0, y - 1.0, 18.0, 1.0], argb_top),
        // Left 1px line (16px vertical inside height)
        ([x - 1.0, y, 1.0, 16.0], argb_sides),
        // Right 1px line (16px vertical inside height)
        ([x + 16.0, y, 1.0, 16.0], argb_sides),
        // Bottom 1px line (prominent 18px solid accent line)
        ([x - 1.0, y + 16.0, 18.0, 1.0], argb_bottom),
    ]
}

/// Renders the slot border lines through a drawing callback `draw(rect, argb)`.
pub fn render_slot_border<F>(x: f32, y: f32, color_rgb: u32, mut draw: F)
where
    F: FnMut([f32; 4], u32),
{
    let rects = slot_border_rects(x, y, color_rgb);
    for (rect, argb) in rects {
        draw(rect, argb);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn test_01_empty_slot_no_border() {
        let _lock = TEST_LOCK.lock().unwrap();
        reset_settings_for_tests();

        // 1. evaluate_slot_border must return None for count == 0 regardless of item
        assert_eq!(
            evaluate_slot_border(true, 0, "minecraft:air", false, false, false, "rare", false),
            None
        );
        assert_eq!(
            evaluate_slot_border(
                true,
                0,
                "minecraft:diamond_sword",
                false,
                false,
                false,
                "rare",
                false
            ),
            None
        );

        // 2. evaluate_slot_border must return None for minecraft:air even if count > 0
        assert_eq!(
            evaluate_slot_border(true, 1, "minecraft:air", false, false, false, "rare", false),
            None
        );
        assert_eq!(
            evaluate_slot_border(true, 64, "", false, false, false, "rare", false),
            None
        );

        // 3. render_slot_decoration on empty/air slot MUST execute 0 draw calls
        let mut drawn_calls = 0;
        let count = render_slot_decoration(
            10.0,
            10.0,
            true,
            0,
            "minecraft:air",
            false,
            false,
            false,
            "rare",
            false,
            |_, _| drawn_calls += 1,
        );
        assert_eq!(
            count, 0,
            "render_slot_decoration must return 0 calls for empty slot"
        );
        assert_eq!(drawn_calls, 0, "Empty slot must not trigger any draw calls");

        let mut drawn_calls_air = 0;
        let count_air = render_slot_decoration(
            10.0,
            10.0,
            true,
            1,
            "minecraft:air",
            false,
            false,
            false,
            "epic",
            false,
            |_, _| drawn_calls_air += 1,
        );
        assert_eq!(count_air, 0);
        assert_eq!(drawn_calls_air, 0);
    }

    #[test]
    fn test_02_regression_independent_settings_initialization() {
        let _lock = TEST_LOCK.lock().unwrap();
        reset_settings_for_tests();

        let mut enabled_read_count = 0;
        let mut common_read_count = 0;
        let mut hotbar_read_count = 0;

        // Step 1: Call is_enabled first (this previously broke subsequent common/hotbar reads!)
        let enabled = is_enabled(|| {
            enabled_read_count += 1;
            Some(true)
        });
        assert!(enabled);
        assert_eq!(enabled_read_count, 1);

        // Step 2: Call is_show_common - fallback MUST be executed!
        let common = is_show_common(|| {
            common_read_count += 1;
            Some(true) // disk has saved 'true'
        });
        assert!(common, "is_show_common must load saved 'true' from disk");
        assert_eq!(
            common_read_count, 1,
            "Fallback for show_common must be invoked exactly once"
        );

        // Step 3: Call is_show_hotbar - fallback MUST be executed!
        let hotbar = is_show_hotbar(|| {
            hotbar_read_count += 1;
            Some(true) // disk has saved 'true'
        });
        assert!(hotbar, "is_show_hotbar must load saved 'true' from disk");
        assert_eq!(
            hotbar_read_count, 1,
            "Fallback for show_hotbar must be invoked exactly once"
        );

        // Step 4: Subsequent calls must use cached values without re-invoking fallback
        assert!(is_enabled(|| {
            panic!("Should not re-read from disk");
        }));
        assert!(is_show_common(|| {
            panic!("Should not re-read from disk");
        }));
        assert!(is_show_hotbar(|| {
            panic!("Should not re-read from disk");
        }));

        // Step 5: Test restart simulation where disk has disabled=true, common=true, hotbar=false
        reset_settings_for_tests();
        assert!(!is_enabled(|| Some(false)));
        assert!(is_show_common(|| Some(true)));
        assert!(!is_show_hotbar(|| Some(false)));
    }

    #[test]
    fn test_03_item_tooltip_rarity_pipeline_uncommon() {
        // Output from crate::item_tooltip::rarity() is "uncommon"
        let rarity_str = "uncommon";
        assert_eq!(
            resolve_border_color_from_rarity_str(rarity_str, false),
            Some(COLOR_UNCOMMON)
        );

        let mut drawn = Vec::new();
        let calls = render_slot_decoration(
            16.0,
            16.0,
            true,
            1,
            "minecraft:snort_pottery_sherd",
            false,
            false,
            false,
            rarity_str,
            false,
            |rect, argb| drawn.push((rect, argb)),
        );
        assert_eq!(calls, 4);
        assert_eq!(drawn.len(), 4);
        assert_eq!(drawn[3].1, (0xFF << 24) | COLOR_UNCOMMON); // Bottom solid accent
    }

    #[test]
    fn test_04_item_tooltip_rarity_pipeline_rare() {
        // Output from crate::item_tooltip::rarity() is "rare"
        let rarity_str = "rare";
        assert_eq!(
            resolve_border_color_from_rarity_str(rarity_str, false),
            Some(COLOR_RARE)
        );

        let mut drawn = Vec::new();
        let calls = render_slot_decoration(
            16.0,
            16.0,
            true,
            1,
            "minecraft:trident",
            false,
            false,
            false,
            rarity_str,
            false,
            |rect, argb| drawn.push((rect, argb)),
        );
        assert_eq!(calls, 4);
        assert_eq!(drawn.len(), 4);
        assert_eq!(drawn[3].1, (0xFF << 24) | COLOR_RARE); // Bottom solid accent
    }

    #[test]
    fn test_05_item_tooltip_rarity_pipeline_epic() {
        // Output from crate::item_tooltip::rarity() is "epic"
        let rarity_str = "epic";
        assert_eq!(
            resolve_border_color_from_rarity_str(rarity_str, false),
            Some(COLOR_EPIC)
        );

        let mut drawn = Vec::new();
        let calls = render_slot_decoration(
            16.0,
            16.0,
            true,
            1,
            "minecraft:dragon_egg",
            false,
            false,
            false,
            rarity_str,
            false,
            |rect, argb| drawn.push((rect, argb)),
        );
        assert_eq!(calls, 4);
        assert_eq!(drawn.len(), 4);
        assert_eq!(drawn[3].1, (0xFF << 24) | COLOR_EPIC); // Bottom solid accent
    }

    #[test]
    fn test_06_item_tooltip_rarity_pipeline_common() {
        // Output from crate::item_tooltip::rarity() is "common"
        let rarity_str = "common";

        // When show_common = false -> None and 0 draw calls
        assert_eq!(
            resolve_border_color_from_rarity_str(rarity_str, false),
            None
        );
        let mut drawn = Vec::new();
        let calls = render_slot_decoration(
            16.0,
            16.0,
            true,
            64,
            "minecraft:stone",
            false,
            false,
            false,
            rarity_str,
            false,
            |rect, argb| drawn.push((rect, argb)),
        );
        assert_eq!(calls, 0);
        assert_eq!(drawn.len(), 0);

        // When show_common = true -> Some(COLOR_COMMON) and 4 draw calls
        assert_eq!(
            resolve_border_color_from_rarity_str(rarity_str, true),
            Some(COLOR_COMMON)
        );
        let mut drawn_on = Vec::new();
        let calls_on = render_slot_decoration(
            16.0,
            16.0,
            true,
            64,
            "minecraft:stone",
            false,
            false,
            false,
            rarity_str,
            true,
            |rect, argb| drawn_on.push((rect, argb)),
        );
        assert_eq!(calls_on, 4);
        assert_eq!(drawn_on.len(), 4);
        assert_eq!(drawn_on[3].1, (0xFF << 24) | COLOR_COMMON);
    }

    #[test]
    fn test_07_item_tooltip_enchantment_upgrade_flow() {
        // Simulate item_tooltip::rarity logic when stack is enchanted:
        // Common & Uncommon become Rare; Rare becomes Epic; Epic stays Epic.
        let upgrade = |base_rarity: &str, is_enchanted: bool| -> &'static str {
            if !is_enchanted {
                match base_rarity {
                    "uncommon" => "uncommon",
                    "rare" => "rare",
                    "epic" => "epic",
                    _ => "common",
                }
            } else {
                match base_rarity {
                    "common" | "uncommon" => "rare",
                    "rare" => "epic",
                    _ => "epic",
                }
            }
        };

        // Enchanted Common sword becomes Rare ("rare" -> COLOR_RARE)
        let res = upgrade("common", true);
        assert_eq!(res, "rare");
        assert_eq!(
            resolve_border_color_from_rarity_str(res, false),
            Some(COLOR_RARE)
        );

        // Enchanted Uncommon bow becomes Rare ("rare" -> COLOR_RARE)
        let res_uncommon = upgrade("uncommon", true);
        assert_eq!(res_uncommon, "rare");
        assert_eq!(
            resolve_border_color_from_rarity_str(res_uncommon, false),
            Some(COLOR_RARE)
        );

        // Enchanted Rare trident becomes Epic ("epic" -> COLOR_EPIC)
        let res_rare = upgrade("rare", true);
        assert_eq!(res_rare, "epic");
        assert_eq!(
            resolve_border_color_from_rarity_str(res_rare, false),
            Some(COLOR_EPIC)
        );

        // Enchanted Epic item remains Epic ("epic" -> COLOR_EPIC)
        let res_epic = upgrade("epic", true);
        assert_eq!(res_epic, "epic");
        assert_eq!(
            resolve_border_color_from_rarity_str(res_epic, false),
            Some(COLOR_EPIC)
        );
    }

    #[test]
    fn test_08_damaged_item_no_durability_overlap() {
        let x = 100.0;
        let y = 100.0;
        let rects = slot_border_rects(x, y, COLOR_UNCOMMON);

        let bar_y_top = y + 13.0;
        let bar_y_bottom = bar_y_top + 2.0;
        let bottom_line_y = rects[3].0[1];

        assert!(
            bottom_line_y >= bar_y_bottom,
            "Bottom border line ({}) overlaps durability bar (bottom {})",
            bottom_line_y,
            bar_y_bottom
        );
    }

    #[test]
    fn test_09_stacked_items_no_text_overlap() {
        let x = 50.0;
        let y = 50.0;
        let rects = slot_border_rects(x, y, COLOR_RARE);

        for (rect, _) in rects {
            let [rx, ry, rw, rh] = rect;
            let inside_item_x = rx > x && (rx + rw) < (x + 16.0);
            let inside_item_y = ry > y && (ry + rh) < (y + 16.0);
            assert!(
                !(inside_item_x && inside_item_y),
                "Border rect {:?} intrudes into item interior",
                rect
            );
        }
    }

    #[test]
    fn test_10_gui_scale_proportionality() {
        for scale in [1.0, 2.0, 3.0, 4.0] {
            let x = 32.0 * scale;
            let y = 32.0 * scale;
            let rects = slot_border_rects(x, y, COLOR_EPIC);
            assert_eq!(rects[0].0[2], 18.0);
            assert_eq!(rects[0].0[3], 1.0);
            assert_eq!(rects[3].0[2], 18.0);
            assert_eq!(rects[3].0[3], 1.0);
        }
    }

    #[test]
    fn test_11_mod_disabled_toggle() {
        let _lock = TEST_LOCK.lock().unwrap();
        reset_settings_for_tests();

        set_enabled(true);
        assert!(is_enabled(|| None));
        let toggled = toggle_enabled();
        assert!(!toggled);
        assert!(!is_enabled(|| None));

        // When disabled, render_slot_decoration produces 0 draw calls
        let mut drawn = 0;
        let calls = render_slot_decoration(
            10.0,
            10.0,
            false,
            1,
            "minecraft:trident",
            false,
            false,
            false,
            "rare",
            false,
            |_, _| drawn += 1,
        );
        assert_eq!(calls, 0);
        assert_eq!(drawn, 0);

        set_enabled(true);
    }

    #[test]
    fn test_12_show_common_toggle() {
        let _lock = TEST_LOCK.lock().unwrap();
        reset_settings_for_tests();

        set_show_common(false);
        assert!(!is_show_common(|| None));
        let toggled = toggle_show_common();
        assert!(toggled);
        assert!(is_show_common(|| None));

        let rarity = resolve_rarity("common", false, None, false);
        assert_eq!(rarity, Rarity::Common);
        let color = rarity_color(rarity);
        assert_eq!(color, COLOR_COMMON);
        assert_eq!(color, 0x94A3B8);

        set_show_common(false);
    }

    #[test]
    fn test_13_slot_border_geometry_bounds() {
        let x = 20.0;
        let y = 30.0;
        let rects = slot_border_rects(x, y, COLOR_RARE);

        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for (rect, _) in rects {
            let [rx, ry, rw, rh] = rect;
            min_x = min_x.min(rx);
            min_y = min_y.min(ry);
            max_x = max_x.max(rx + rw);
            max_y = max_y.max(ry + rh);
        }

        assert_eq!(min_x, x - 1.0);
        assert_eq!(min_y, y - 1.0);
        assert_eq!(max_x, x + 17.0);
        assert_eq!(max_y, y + 17.0);
        assert_eq!(max_x - min_x, 18.0);
        assert_eq!(max_y - min_y, 18.0);
    }

    #[test]
    fn test_14_no_slot_highlight_conflict() {
        let x = 60.0;
        let y = 60.0;
        let rects = slot_border_rects(x, y, COLOR_EPIC);

        let slot_min_x = x - 1.0;
        let slot_max_x = x + 17.0;
        let slot_min_y = y - 1.0;
        let slot_max_y = y + 17.0;

        for (rect, _) in rects {
            let [rx, ry, rw, rh] = rect;
            assert!(rx >= slot_min_x && (rx + rw) <= slot_max_x);
            assert!(ry >= slot_min_y && (ry + rh) <= slot_max_y);
        }
    }

    #[test]
    fn test_15_carried_cursor_suppression() {
        // Carried cursor item must NEVER show a slot border and must yield 0 draw calls
        let mut drawn = 0;
        let calls = render_slot_decoration(
            10.0,
            10.0,
            true,
            1,
            "minecraft:trident",
            false,
            false,
            true, // is_carried_cursor
            "rare",
            false,
            |_, _| drawn += 1,
        );
        assert_eq!(calls, 0);
        assert_eq!(drawn, 0);
    }

    #[test]
    fn test_16_hotbar_rules() {
        // In-game playing HUD:
        // When show_hotbar is false -> 0 calls
        let mut drawn = 0;
        let calls = render_slot_decoration(
            10.0,
            10.0,
            true,
            1,
            "minecraft:dragon_egg",
            true,  // is_playing_hud
            false, // show_hotbar = false
            false,
            "epic",
            false,
            |_, _| drawn += 1,
        );
        assert_eq!(calls, 0);
        assert_eq!(drawn, 0);

        // When show_hotbar is true -> 4 calls
        let calls_on = render_slot_decoration(
            10.0,
            10.0,
            true,
            1,
            "minecraft:dragon_egg",
            true, // is_playing_hud
            true, // show_hotbar = true
            false,
            "epic",
            false,
            |_, _| drawn += 1,
        );
        assert_eq!(calls_on, 4);
        assert_eq!(drawn, 4);
    }

    #[test]
    fn test_17_exact_draw_calls_alpha_distribution() {
        let mut drawn = Vec::new();
        render_slot_border(10.0, 20.0, COLOR_RARE, |rect, argb| {
            drawn.push((rect, argb));
        });

        assert_eq!(drawn.len(), 4);
        // Top line: 0x4D alpha
        assert_eq!(drawn[0].0, [9.0, 19.0, 18.0, 1.0]);
        assert_eq!(drawn[0].1, (0x4D << 24) | COLOR_RARE);

        // Left line: 0x66 alpha
        assert_eq!(drawn[1].0, [9.0, 20.0, 1.0, 16.0]);
        assert_eq!(drawn[1].1, (0x66 << 24) | COLOR_RARE);

        // Right line: 0x66 alpha
        assert_eq!(drawn[2].0, [26.0, 20.0, 1.0, 16.0]);
        assert_eq!(drawn[2].1, (0x66 << 24) | COLOR_RARE);

        // Bottom line: 0xFF solid alpha (prominent vibrant accent)
        assert_eq!(drawn[3].0, [9.0, 36.0, 18.0, 1.0]);
        assert_eq!(drawn[3].1, (0xFF << 24) | COLOR_RARE);
    }

    #[test]
    fn test_18_custom_rarity_component_override() {
        // A stick with custom `minecraft:rarity = "epic"` must resolve to Epic
        assert_eq!(
            resolve_rarity("common", false, Some("epic"), false),
            Rarity::Epic
        );
        assert_eq!(
            resolve_rarity("common", false, Some("epic"), true),
            Rarity::Epic
        );
    }

    #[test]
    fn test_19_ominous_banner_override() {
        // An ominous banner is always Uncommon base rarity
        assert_eq!(
            resolve_rarity("common", true, None, false),
            Rarity::Uncommon
        );
        // When enchanted, it upgrades to Rare
        assert_eq!(resolve_rarity("common", true, None, true), Rarity::Rare);
    }
}
