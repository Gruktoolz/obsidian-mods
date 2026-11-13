// Fly Speed: how fast you fly in creative and spectator-free flight, set from
// chat with /flyspeed or from Options > Mods > Fly Speed > Settings. This file
// is both the mod's Cargo library (its tests run with `cargo test`) and the
// module the bundle adds to the game as crate::game::fly_speed.

/// The mod's id (its settings folder), name, version and line.
pub const ID: &str = "fly_speed";
pub const NAME: &str = "Fly Speed";
pub const VERSION: &str = "0.2.0";
pub const DESCRIPTION: &str = "Fly 0.1x to 20x your normal speed: /flyspeed 2, /flyspeed +1, /flyspeed reset for 1x.";

/// The game's own flying speed: `Abilities.flyingSpeed`, what the game sets
/// when it gives you flight. 1x is this.
pub const BASE: f32 = 0.05;
/// The slowest and fastest multipliers a player can ask for.
pub const MIN: f32 = 0.1;
pub const MAX: f32 = 20.0;
/// What the settings page's one button steps through.
pub const PRESETS: [f32; 6] = [0.5, 1.0, 2.0, 4.0, 8.0, 16.0];

thread_local! {
    /// The multiplier, read from its file once, then kept.
    static MULTIPLIER: std::cell::Cell<Option<f32>> = const { std::cell::Cell::new(None) };
}

/// `multiplier` kept within what the mod allows; anything unusable is 1x.
pub fn clamp(multiplier: f32) -> f32 {
    if multiplier.is_finite() {
        multiplier.clamp(MIN, MAX)
    } else {
        1.0
    }
}

/// The multiplier: the stored one the first time (`saved` reads it), then the
/// running one.
pub fn multiplier(saved: impl FnOnce() -> Option<String>) -> f32 {
    MULTIPLIER.with(|m| {
        let found =
            m.get().unwrap_or_else(|| clamp(saved().and_then(|s| s.trim().parse().ok()).unwrap_or(1.0)));
        m.set(Some(found));
        found
    })
}

/// Replaces the multiplier (a command or the settings page changed it).
pub fn set_multiplier(multiplier: f32) {
    MULTIPLIER.with(|m| m.set(Some(clamp(multiplier))));
}

/// The multiplier as its setting file keeps it.
pub fn stored(multiplier: f32) -> String {
    format!("{}", clamp(multiplier))
}

/// The game's flying speed for a multiplier: what `PlayerMotion::flying_speed`
/// is set to each tick.
pub fn flying_speed(multiplier: f32) -> f32 {
    BASE * clamp(multiplier)
}

/// `1x`, `2.5x`, `0.5x`: how the mod says a multiplier.
pub fn label(multiplier: f32) -> String {
    format!("{}x", short(clamp(multiplier)))
}

/// A whole number without a `.0`; a fraction as it is.
fn short(multiplier: f32) -> String {
    if (multiplier - multiplier.round()).abs() < 1e-4 {
        format!("{}", multiplier.round() as i64)
    } else {
        format!("{multiplier}")
    }
}

/// The next preset after `multiplier`, wrapping to the slowest.
pub fn next_preset(multiplier: f32) -> f32 {
    PRESETS
        .iter()
        .copied()
        .find(|p| *p > clamp(multiplier) + 1e-4)
        .unwrap_or(PRESETS[0])
}

/// `/flyspeed`'s argument: a multiplier (`2`, `1.5`), or a step from the
/// current one (`+1`, `-0.5`). The result is kept within `MIN`..=`MAX`.
pub fn parse(input: &str, current: f32) -> Result<f32, String> {
    let arg = input.trim();
    let (digits, sign, relative) = match arg.as_bytes().first() {
        Some(b'+') => (&arg[1..], 1.0, true),
        Some(b'-') => (&arg[1..], -1.0, true),
        _ => (arg, 1.0, false),
    };
    let Some(value) = digits.trim().parse::<f32>().ok().filter(|v| v.is_finite() && *v >= 0.0)
    else {
        return Err(format!("Unknown fly speed {arg}: use /flyspeed <number> or /flyspeed reset"));
    };
    Ok(clamp(if relative {
        current + sign * value
    } else {
        value
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_multipliers_where_it_says() {
        assert_eq!(clamp(2.0), 2.0);
        assert_eq!(clamp(0.01), MIN);
        assert_eq!(clamp(50.0), MAX);
        assert_eq!(clamp(f32::NAN), 1.0);
        assert_eq!(clamp(f32::INFINITY), 1.0);
    }

    #[test]
    fn turns_a_multiplier_into_the_games_flying_speed() {
        assert_eq!(flying_speed(1.0), BASE);
        assert!((flying_speed(2.0) - 0.1).abs() < 1e-7);
        assert!((flying_speed(0.5) - 0.025).abs() < 1e-7);
        // A multiplier out of range is kept in it.
        assert!((flying_speed(1000.0) - BASE * MAX).abs() < 1e-7);
    }

    #[test]
    fn says_multipliers_plainly() {
        assert_eq!(label(1.0), "1x");
        assert_eq!(label(2.0), "2x");
        assert_eq!(label(2.5), "2.5x");
        assert_eq!(label(0.5), "0.5x");
        assert_eq!(stored(2.0), "2");
    }

    #[test]
    fn steps_through_the_presets() {
        assert_eq!(next_preset(1.0), 2.0);
        assert_eq!(next_preset(0.5), 1.0);
        assert_eq!(next_preset(8.0), 16.0);
        assert_eq!(next_preset(16.0), 0.5);
        assert_eq!(next_preset(1.3), 2.0);
    }

    #[test]
    fn reads_its_argument() {
        assert_eq!(parse("2", 1.0), Ok(2.0));
        assert_eq!(parse(" 1.5 ", 1.0), Ok(1.5));
        assert_eq!(parse("+1", 2.0), Ok(3.0));
        assert_eq!(parse("-0.5", 1.0), Ok(0.5));
        assert_eq!(parse("0", 1.0), Ok(MIN));
        assert_eq!(parse("1000", 1.0), Ok(MAX));
        assert_eq!(parse("-1000", 4.0), Ok(MIN));
        assert!(parse("fast", 1.0).is_err());
        assert!(parse("", 1.0).is_err());
        assert!(parse("2x", 1.0).is_err());
    }

    #[test]
    fn reads_and_keeps_the_stored_multiplier() {
        set_multiplier(4.0);
        assert_eq!(multiplier(|| Some("8".into())), 4.0);
        set_multiplier(1.0);
        assert_eq!(multiplier(|| Some("garbage".into())), 1.0);
    }
}
