pub const ID: &str = "movement_status";
pub const NAME: &str = "Movement Status";
pub const VERSION: &str = "0.1.1";
pub const DESCRIPTION: &str = "Sprint / Sneak / Flying status lines. Reads current player state and physical key holds. Optional background box.";
pub const BACKGROUND_COLOR: u32 = 0x7800_0000;
pub const INK: u32 = 0xFF_FFFF;

pub fn read_background(stored: Option<String>) -> bool {
    stored.is_none_or(|v| v.trim() != "false")
}
pub fn background_label(on: bool) -> String {
    format!("Background: {}", if on { "ON" } else { "OFF" })
}

thread_local! { static BG_SHOWN: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) }; }

pub fn background(get: impl FnOnce() -> Option<String>) -> bool {
    BG_SHOWN.with(|c| {
        let v = c.get().unwrap_or_else(|| read_background(get()));
        c.set(Some(v));
        v
    })
}
pub fn set_background(on: bool) {
    BG_SHOWN.with(|c| c.set(Some(on)));
}

pub fn status_lines(
    sprinting: bool,
    crouching: bool,
    flying: bool,
    fall_flying: bool,
    sprint_held: bool,
    sneak_held: bool,
    toggle_sprint: bool,
    toggle_sneak: bool,
) -> Vec<String> {
    let mut lines = Vec::new();

    if sprinting {
        let mode = if sprint_held {
            "[Held]"
        } else if toggle_sprint {
            "[Toggled]"
        } else {
            "[Held]"
        };
        lines.push(format!("Sprint {mode}"));
    }
    if crouching {
        let mode = if sneak_held {
            "[Held]"
        } else if toggle_sneak {
            "[Toggled]"
        } else {
            "[Held]"
        };
        lines.push(format!("Sneak {mode}"));
    }
    if fall_flying {
        lines.push("Flying [Elytra]".to_string());
    } else if flying {
        lines.push("Flying [Creative]".to_string());
    }

    lines
}
