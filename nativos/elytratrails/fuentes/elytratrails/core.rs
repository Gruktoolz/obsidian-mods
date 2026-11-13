use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, PoisonError};

pub const ID: &str = "elytratrails";
pub const NAME: &str = "ElytraTrails";
pub const VERSION: &str = "0.1.2";
pub const DESCRIPTION: &str = "Lines from the wingtips of gliding elytra that fade out over time, shared with other players.";

pub const KEYS: [&str; 16] = [
    "trails", "own", "others", "color", "duration", "width", "pattern", "pulse", "fix_own", "fix_others",
    "detail", "range", "formation", "flash", "day_night", "first_person",
];
pub const COLORS: [&str; 7] = ["ice", "gold", "rose", "violet", "white", "rainbow", "speed"];
pub const WIDTHS: [&str; 3] = ["thin", "normal", "wide"];
pub const PATTERNS: [&str; 3] = ["solid", "dashed", "wavy"];
pub const RANGES: [&str; 3] = ["near", "normal", "far"];
const RANGE_BLOCKS: [f64; 3] = [24.0, 48.0, 96.0];

pub const RAINBOW: usize = 5;
pub const SPEED: usize = 6;
pub const DASHED: usize = 1;
pub const WAVY: usize = 2;

pub const MIN_DURATION: u64 = 1;
pub const MAX_DURATION: u64 = 12;
pub const DEFAULT_DURATION: u64 = 5;

pub const PAGE_MAIN: usize = 0;
pub const PAGE_OPTIMIZATION: usize = 1;
pub const OPEN_OPTIMIZATION: usize = 10;
pub const DAY_NIGHT_TOGGLE: usize = 8;
pub const FIRST_PERSON_TOGGLE: usize = 9;
pub const OPTIMIZATION_BACK: usize = 6;

const COLOR_RGBA: [[f32; 4]; 7] = [
    [0.55, 0.9, 1.0, 0.9],
    [1.0, 0.8, 0.3, 0.9],
    [1.0, 0.45, 0.6, 0.9],
    [0.7, 0.5, 1.0, 0.9],
    [1.0, 1.0, 1.0, 0.85],
    [1.0, 1.0, 1.0, 0.9],
    [1.0, 1.0, 1.0, 0.9],
];
const WIDTH_PX: [f32; 3] = [1.5, 2.5, 4.0];

pub static CACHE: Mutex<Option<Settings>> = Mutex::new(None);
static PAGE: AtomicUsize = AtomicUsize::new(PAGE_MAIN);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    pub trails: bool,
    pub own: bool,
    pub others: bool,
    pub color: usize,
    pub duration: u64,
    pub width: usize,
    pub pattern: usize,
    pub pulse: bool,
    pub own_fixed: bool,
    pub others_fixed: bool,
    pub half_detail: bool,
    pub range: usize,
    pub formation: bool,
    pub flash: bool,
    pub day_night: bool,
    pub first_person: bool,
}

fn raw(get: &impl Fn(&str) -> Option<String>, key: &str) -> Option<String> {
    get(key).map(|value| value.trim().to_ascii_lowercase())
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let on_unless_off = |key: &str| raw(&get, key).as_deref() != Some("off");
        let off_unless_on = |key: &str| raw(&get, key).as_deref() == Some("on");
        let choice = |key: &str, names: &[&str], fallback: usize| {
            raw(&get, key)
                .and_then(|value| names.iter().position(|name| *name == value))
                .unwrap_or(fallback)
        };
        let duration = raw(&get, KEYS[4])
            .and_then(|value| value.parse::<u64>().ok())
            .filter(|seconds| (MIN_DURATION..=MAX_DURATION).contains(seconds))
            .unwrap_or(DEFAULT_DURATION);
        Self {
            trails: on_unless_off(KEYS[0]),
            own: on_unless_off(KEYS[1]),
            others: on_unless_off(KEYS[2]),
            color: choice(KEYS[3], &COLORS, 0),
            duration,
            width: choice(KEYS[5], &WIDTHS, 1),
            pattern: choice(KEYS[6], &PATTERNS, 0),
            pulse: on_unless_off(KEYS[7]),
            own_fixed: off_unless_on(KEYS[8]),
            others_fixed: off_unless_on(KEYS[9]),
            half_detail: raw(&get, KEYS[10]).as_deref() == Some("half"),
            range: choice(KEYS[11], &RANGES, 1),
            formation: on_unless_off(KEYS[12]),
            flash: on_unless_off(KEYS[13]),
            day_night: off_unless_on(KEYS[14]),
            first_person: on_unless_off(KEYS[15]),
        }
    }

    pub fn life(&self) -> u64 {
        self.duration * 20
    }

    pub fn px(&self) -> f32 {
        WIDTH_PX[self.width]
    }

    pub fn range_blocks(&self) -> f64 {
        RANGE_BLOCKS[self.range]
    }
}

pub fn rgba_of(index: usize) -> [f32; 4] {
    COLOR_RGBA[index]
}

pub fn flag(on: bool) -> &'static str {
    if on { "on" } else { "off" }
}

pub fn cycle(names: &[&'static str], current: usize) -> &'static str {
    names[(current + 1) % names.len()]
}

pub fn next_duration(seconds: u64) -> u64 {
    if seconds >= MAX_DURATION { MIN_DURATION } else { seconds + 1 }
}

pub fn page() -> usize {
    PAGE.load(Ordering::Relaxed)
}

pub fn set_page(page: usize) {
    PAGE.store(page, Ordering::Relaxed);
}

pub fn fixed_label(fixed: bool) -> &'static str {
    if fixed { "fixed" } else { "dynamic" }
}

pub fn labels(settings: Settings) -> Vec<String> {
    vec![
        format!("Trails: {}", flag(settings.trails)),
        format!("Your trail: {}", flag(settings.own)),
        format!("Other players' trails: {}", flag(settings.others)),
        format!("Color: {}", COLORS[settings.color]),
        format!("Duration: {} s", settings.duration),
        format!("Width: {}", WIDTHS[settings.width]),
        format!("Pattern: {}", PATTERNS[settings.pattern]),
        format!("Pulse: {}", flag(settings.pulse)),
        format!("Day/night brightness: {}", flag(settings.day_night)),
        format!("Your trail in first person: {}", flag(settings.first_person)),
        "Optimization >".to_owned(),
    ]
}

pub fn optimization_labels(settings: Settings) -> Vec<String> {
    vec![
        format!("Your wings: {}", fixed_label(settings.own_fixed)),
        format!("Other players' wings: {}", fixed_label(settings.others_fixed)),
        format!("Line detail: {}", if settings.half_detail { "half" } else { "full" }),
        format!("Other players' range: {} blocks", settings.range_blocks()),
        format!("Formation lines: {}", flag(settings.formation)),
        format!("Boost flash: {}", flag(settings.flash)),
        "< Back".to_owned(),
    ]
}

pub fn invalidate() {
    *CACHE.lock().unwrap_or_else(PoisonError::into_inner) = None;
}
