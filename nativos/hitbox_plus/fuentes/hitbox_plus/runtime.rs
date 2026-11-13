// HitBox+ runtime: state, configuration, and styling for custom hitbox rendering.
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering};

pub const ID: &str = "hitbox_plus";
pub const NAME: &str = "HitBox+";
pub const VERSION: &str = "1.3.0";
pub const DESCRIPTION: &str =
    "Advanced hitbox rendering: green when crosshair is on an entity in attack reach, custom colors/thickness, eye rays and toggle sprint display.";
pub const KEY: &str = "toggle";
pub const DEFAULT_KEY: &str = "key.keyboard.h";

pub static ENABLED: AtomicBool = AtomicBool::new(true);
pub static SHOW_EYE: AtomicBool = AtomicBool::new(true);
pub static STYLE_MODE: AtomicU8 = AtomicU8::new(0);
pub static THICKNESS_MODE: AtomicU8 = AtomicU8::new(1);

// Toggle Sprint Display
pub static SPRINT_DISPLAY: AtomicBool = AtomicBool::new(true);
pub static SPRINT_CORNER: AtomicU8 = AtomicU8::new(0); // 0: TopLeft, 1: TopRight, 2: BottomLeft, 3: BottomRight
pub const SPRINT_CORNER_NAMES: [&str; 4] = ["Top-Left", "Top-Right", "Bottom-Left", "Bottom-Right"];

static PLAYER_EYE_X: AtomicU64 = AtomicU64::new(0);
static PLAYER_EYE_Y: AtomicU64 = AtomicU64::new(0);
static PLAYER_EYE_Z: AtomicU64 = AtomicU64::new(0);

static PLAYER_DIR_X: AtomicU64 = AtomicU64::new(0);
static PLAYER_DIR_Y: AtomicU64 = AtomicU64::new(0);
static PLAYER_DIR_Z: AtomicU64 = AtomicU64::new(0);

static ATTACK_REACH: AtomicU64 = AtomicU64::new(0);

pub const IN_REACH_COLOR: u32 = 0xFF00_E676; // Bright green when crosshair targets entity in attack reach

pub const STYLES: [&str; 4] = ["Neon Cyan", "Gold Yellow", "Crimson Red", "White"];
pub const COLORS: [u32; 4] = [
    0xFF00_E5FF, 0xFFFF_D600, 0xFFFF_1744, 0xFFFF_FFFF,
];
pub const THICKNESSES: [f32; 3] = [1.5, 2.5, 4.0];
pub const THICKNESS_LABELS: [&str; 3] = ["Thin (1.5px)", "Normal (2.5px)", "Thick (4.0px)"];

pub fn is_enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

pub fn current_color() -> u32 {
    COLORS[(STYLE_MODE.load(Ordering::Relaxed) as usize) % COLORS.len()]
}

pub fn current_thickness() -> f32 {
    THICKNESSES[(THICKNESS_MODE.load(Ordering::Relaxed) as usize) % THICKNESSES.len()]
}

pub fn show_eye_ray() -> bool {
    SHOW_EYE.load(Ordering::Relaxed)
}

pub fn toggle() -> bool {
    let next = !is_enabled();
    ENABLED.store(next, Ordering::Relaxed);
    next
}

pub fn cycle_style() -> &'static str {
    let next = (STYLE_MODE.load(Ordering::Relaxed) + 1) % (STYLES.len() as u8);
    STYLE_MODE.store(next, Ordering::Relaxed);
    STYLES[next as usize]
}

pub fn cycle_thickness() -> &'static str {
    let next = (THICKNESS_MODE.load(Ordering::Relaxed) + 1) % (THICKNESSES.len() as u8);
    THICKNESS_MODE.store(next, Ordering::Relaxed);
    THICKNESS_LABELS[next as usize]
}

pub fn toggle_eye_ray() -> bool {
    let next = !show_eye_ray();
    SHOW_EYE.store(next, Ordering::Relaxed);
    next
}

// Toggle Sprint Display helpers
pub fn show_sprint_display() -> bool {
    SPRINT_DISPLAY.load(Ordering::Relaxed)
}

pub fn toggle_sprint_display() -> bool {
    let next = !show_sprint_display();
    SPRINT_DISPLAY.store(next, Ordering::Relaxed);
    next
}

pub fn cycle_sprint_corner() -> &'static str {
    let next = (SPRINT_CORNER.load(Ordering::Relaxed) + 1) % 4;
    SPRINT_CORNER.store(next, Ordering::Relaxed);
    SPRINT_CORNER_NAMES[next as usize]
}

pub fn sprint_corner() -> u8 {
    SPRINT_CORNER.load(Ordering::Relaxed) % 4
}

pub fn update_player_aim(x: f64, y: f64, z: f64, yaw: f32, pitch: f32, reach: f64) {
    PLAYER_EYE_X.store(x.to_bits(), Ordering::Relaxed);
    PLAYER_EYE_Y.store(y.to_bits(), Ordering::Relaxed);
    PLAYER_EYE_Z.store(z.to_bits(), Ordering::Relaxed);

    let (yr, pr) = ((yaw as f64).to_radians(), (pitch as f64).to_radians());
    let dx = -yr.sin() * pr.cos();
    let dy = -pr.sin();
    let dz = yr.cos() * pr.cos();

    PLAYER_DIR_X.store(dx.to_bits(), Ordering::Relaxed);
    PLAYER_DIR_Y.store(dy.to_bits(), Ordering::Relaxed);
    PLAYER_DIR_Z.store(dz.to_bits(), Ordering::Relaxed);

    ATTACK_REACH.store(reach.to_bits(), Ordering::Relaxed);
}

fn player_eye() -> [f64; 3] {
    [
        f64::from_bits(PLAYER_EYE_X.load(Ordering::Relaxed)),
        f64::from_bits(PLAYER_EYE_Y.load(Ordering::Relaxed)),
        f64::from_bits(PLAYER_EYE_Z.load(Ordering::Relaxed)),
    ]
}

fn player_dir() -> [f64; 3] {
    [
        f64::from_bits(PLAYER_DIR_X.load(Ordering::Relaxed)),
        f64::from_bits(PLAYER_DIR_Y.load(Ordering::Relaxed)),
        f64::from_bits(PLAYER_DIR_Z.load(Ordering::Relaxed)),
    ]
}

fn current_reach() -> f64 {
    f64::from_bits(ATTACK_REACH.load(Ordering::Relaxed))
}

/// Raycast test: returns true if the crosshair ray intersects [min, max] within current attack reach.
pub fn is_crosshair_on_target(min: [f64; 3], max: [f64; 3]) -> bool {
    let eye = player_eye();
    let dir = player_dir();
    let reach = current_reach();

    if reach <= 0.0 {
        return false;
    }

    let mut tmin = 0.0f64;
    let mut tmax = reach;

    for a in 0..3 {
        if dir[a].abs() < 1e-9 {
            if eye[a] < min[a] || eye[a] > max[a] {
                return false;
            }
        } else {
            let mut t0 = (min[a] - eye[a]) / dir[a];
            let mut t1 = (max[a] - eye[a]) / dir[a];
            if t0 > t1 {
                core::mem::swap(&mut t0, &mut t1);
            }
            tmin = tmin.max(t0);
            tmax = tmax.min(t1);
            if tmax < tmin {
                return false;
            }
        }
    }

    tmax >= 0.0 && tmin <= reach
}
