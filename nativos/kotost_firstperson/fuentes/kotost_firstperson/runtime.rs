// Kotost First Person: a Horizon native mod (horizon-mixins support module, put into horizon-client-lib's root).
// 1) First person: the local player's body, legs and both arms are drawn as the model's own (head and hat hidden),
//    pushed back as the view pitches down; the shadow pass keeps only the head at the true position, so the
//    shadow is one whole body.
// 2) The model's right arm holds the items in 3D instead of the game's flat hands; while eating or drinking the
//    game's own hands bring the food before the face (`eat_hand`), or the 3D arm does (`eat_hand = 0`).
// It shares nothing with other mods: whatever else draws on the player model (a bust, clothes) shows on the
// first-person body by itself, since the body is the game's own player model drawn once more.
// Numbers can be tuned while playing: `config/kotost-firstperson.txt` in the game's folder, re-read when it changes.

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::time::{Duration, Instant, SystemTime};

/// What the local player's model is being drawn for right now.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    /// Not ours: the game as it is.
    Off,
    /// First person, the shadow pass's copy at the true position: the head (and head items) only.
    HeadOnly,
    /// First person, the visible copy pushed back: body, legs and arms, no head.
    Body,
}

#[derive(Clone, Copy, Debug)]
pub struct Tune {
    pub first_person: bool,
    /// The model's left arm (and sleeve) shows too.
    pub left_arm: bool,
    /// The model's right arm and both held items in 3D, instead of the game's flat hands.
    pub right_arm: bool,
    /// The body pushed back (blocks), looking ahead and the extra looking straight down.
    pub back: f32,
    pub back_down: f32,
    /// Eating or drinking: how far the arm comes up (radians over the head's pitch), across to the middle
    /// (radians), and the bob of each bite.
    pub eat: bool,
    /// Eating shows the game's own hands (the food before the face) instead of the 3D arm.
    pub eat_hand: bool,
    pub eat_up: f32,
    pub eat_across: f32,
    pub eat_bob: f32,
}

impl Default for Tune {
    fn default() -> Self {
        Tune {
            first_person: true,
            left_arm: true,
            right_arm: true,
            back: 0.0,
            back_down: 0.27,
            eat: true,
            eat_hand: true,
            eat_up: 1.35,
            eat_across: 0.6,
            eat_bob: 0.16,
        }
    }
}

// Drawing runs on one thread; the locks are never held across a call into the game.
static MODE: AtomicU8 = AtomicU8::new(0);
static TUNE: Mutex<Option<TuneCache>> = Mutex::new(None);

struct TuneCache {
    tune: Tune,
    stamp: Option<SystemTime>,
    checked: Instant,
}

fn lock<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

pub fn mode() -> Mode {
    match MODE.load(Ordering::Relaxed) {
        1 => Mode::HeadOnly,
        2 => Mode::Body,
        _ => Mode::Off,
    }
}

pub fn set_mode(m: Mode) {
    let v = match m {
        Mode::Off => 0,
        Mode::HeadOnly => 1,
        Mode::Body => 2,
    };
    MODE.store(v, Ordering::Relaxed);
}

/// The settings file: `config/kotost-firstperson.txt` in the game's folder (`--home`, `HORIZON_HOME` or the
/// platform's data folder), where native mods keep theirs.
fn config_path() -> std::path::PathBuf {
    crate::logging::game_home().join("config").join("kotost-firstperson.txt")
}

/// Kotost Body (up to 0.3) kept these settings in its own file: a first run carries them over.
fn old_body_file() -> Option<String> {
    let home = crate::logging::game_home();
    [home.join("kotost-body.txt"), home.join("config").join("kotost-body.txt")]
        .iter()
        .find_map(|p| std::fs::read_to_string(p).ok())
}

const KEYS: [&str; 10] =
    ["first_person", "left_arm", "right_arm", "back", "back_down", "eat", "eat_hand", "eat_up", "eat_across", "eat_bob"];

/// What a missing settings file is made with: every key, its default and what it does.
const DEFAULT_FILE: &str = r#"# Kotost First Person settings. The game re-reads this file by itself (twice a second): no restart needed.
# key = value; anything after # is a comment; delete a line to get its default back.

first_person = 1   # 0: the game's usual first person
left_arm = 1       # the model's left arm shows
right_arm = 1      # the model's right arm and held items in 3D instead of the flat hands
back = 0.0         # body pushed back (blocks) looking straight ahead
back_down = 0.27   # extra push back looking down (so you never see inside your shoulders)
eat = 1            # eating and drinking: the 3D arm brings the item before your face and bobs (0: no)
eat_hand = 1       # 1: while eating, the game's own first-person hands with the food before your face
eat_up = 1.35      # how high the arm comes up (eat_hand = 0)
eat_across = 0.6   # how far across to the middle
eat_bob = 0.16     # the bob of each bite
"#;

fn parse(text: &str) -> Tune {
    let mut t = Tune::default();
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or("");
        let Some((key, value)) = line.split_once('=') else { continue };
        let (key, value) = (key.trim(), value.trim());
        let on = matches!(value, "1" | "on" | "true" | "yes");
        let Ok(n) = value.parse::<f32>() else {
            match key {
                "first_person" => t.first_person = on,
                "left_arm" => t.left_arm = on,
                "right_arm" => t.right_arm = on,
                "eat" => t.eat = on,
                "eat_hand" => t.eat_hand = on,
                _ => {}
            }
            continue;
        };
        if !n.is_finite() {
            continue;
        }
        match key {
            "first_person" => t.first_person = n != 0.0,
            "left_arm" => t.left_arm = n != 0.0,
            "right_arm" => t.right_arm = n != 0.0,
            "back" => t.back = n.clamp(-0.5, 1.0),
            "back_down" => t.back_down = n.clamp(0.0, 1.0),
            "eat" => t.eat = n != 0.0,
            "eat_hand" => t.eat_hand = n != 0.0,
            "eat_up" => t.eat_up = n.clamp(0.0, 3.0),
            "eat_across" => t.eat_across = n.clamp(0.0, 1.5),
            "eat_bob" => t.eat_bob = n.clamp(0.0, 1.0),
            _ => {}
        }
    }
    t
}

/// The first run's file: the defaults, then the first-person lines of an old kotost-body.txt (later lines win).
fn first_file() -> String {
    let mut text = DEFAULT_FILE.to_owned();
    let Some(old) = old_body_file() else {
        return text;
    };
    let carried: Vec<&str> = old
        .lines()
        .filter(|l| {
            let key = l.split('#').next().unwrap_or("").split('=').next().unwrap_or("").trim();
            l.contains('=') && KEYS.contains(&key)
        })
        .collect();
    if !carried.is_empty() {
        text.push_str("\n# carried over from kotost-body.txt\n");
        for l in carried {
            text.push_str(l.trim());
            text.push('\n');
        }
    }
    text
}

/// The settings, read again when the file changed (checked at most twice a second).
pub fn tune() -> Tune {
    let mut cache = lock(&TUNE);
    let now = Instant::now();
    let due = cache
        .as_ref()
        .is_none_or(|c| now.duration_since(c.checked) > Duration::from_millis(500));
    if due {
        let path = config_path();
        let stamp = std::fs::metadata(&path).and_then(|m| m.modified()).ok();
        match cache.as_mut() {
            Some(c) if c.stamp == stamp => c.checked = now,
            _ => {
                let tune = match std::fs::read_to_string(&path) {
                    Ok(text) => parse(&text),
                    Err(_) => {
                        // First run: write the file with its defaults, to be edited while playing.
                        let text = first_file();
                        if !path.exists()
                            && let Some(dir) = path.parent()
                            && std::fs::create_dir_all(dir).is_ok()
                        {
                            let _ = std::fs::write(&path, &text);
                        }
                        parse(&text)
                    }
                };
                *cache = Some(TuneCache { tune, stamp, checked: now });
            }
        }
    }
    cache.as_ref().expect("filled above").tune
}

/// Whether to draw the body.
pub fn first_person() -> bool {
    tune().first_person
}

/// How far back (blocks) the body goes at this pitch (degrees, + looks down).
pub fn back(pitch: f32) -> f64 {
    let t = tune();
    let down = pitch.to_radians().sin().max(0.0);
    f64::from(t.back + t.back_down * down)
}

// --- eating and drinking --------------------------------------------------------------------------

thread_local! {
    /// The own model being posed for the first-person body eats or drinks: with the left arm, ticks in.
    static EATING: std::cell::Cell<Option<(bool, f32)>> = const { std::cell::Cell::new(None) };
}

/// Items used with their own arm pose (bow, shield, spyglass...): not eating.
const NOT_FOOD: [&str; 8] = [
    "minecraft:bow",
    "minecraft:crossbow",
    "minecraft:trident",
    "minecraft:spyglass",
    "minecraft:goat_horn",
    "minecraft:brush",
    "minecraft:shield",
    "minecraft:fishing_rod",
];

/// The own first-person body is eating or drinking now (set while it is posed each frame).
static FP_EATING: AtomicBool = AtomicBool::new(false);

/// `anim::setup`'s HEAD: whether the model posed now is the own first-person body eating or drinking.
pub fn eat_begin(m: &crate::mobs::Mob, partial: f32) {
    let body = mode() == Mode::Body;
    let eating = (body && m.extra.using_item > 0 && !m.extra.blocking)
        .then(|| {
            let item = if m.extra.using_off_hand { m.extra.off_hand.as_deref() } else { m.extra.main_hand.as_deref() };
            let item = item?;
            if NOT_FOOD.contains(&item) || item.ends_with("_spear") {
                return None;
            }
            Some((m.main_arm_left() != m.extra.using_off_hand, m.extra.using_item as f32 + partial))
        })
        .flatten();
    if body {
        FP_EATING.store(eating.is_some(), Ordering::Relaxed);
    }
    EATING.with(|c| c.set(eating));
}

/// Eating or drinking with `eat_hand = 1`: the game's own hand animation shows instead of the model's 3D arm
/// (the arm before the face sits behind the camera, so the food was never seen).
pub fn fp_eating_hand() -> bool {
    tune().eat_hand && FP_EATING.load(Ordering::Relaxed)
}

/// `anim::bob`'s HEAD (right arm, so after both arms are posed): the eating arm comes up before the face,
/// across to the middle, and bobs as vanilla's first-person hand does (`applyEatTransform`).
pub fn eat_arm(p: &mut crate::mobs::anim::Poses) {
    let Some((left, ticks)) = EATING.with(std::cell::Cell::take) else {
        return;
    };
    let arm = if left { "left_arm" } else { "right_arm" };
    if !p.has(arm) || !p.has("head") {
        return;
    }
    let tune = tune();
    if !tune.eat || tune.eat_hand {
        return;
    }
    let (up, across) = (tune.eat_up, tune.eat_across);
    let x = (ticks / 4.0).clamp(0.0, 1.0);
    let k = x * x * (3.0 - 2.0 * x);
    // Vanilla: |cos(t / 4 * pi)| * 0.1 up and down once the hand is there.
    let chew = if ticks > 4.0 { (ticks / 4.0 * std::f32::consts::PI).cos().abs() * tune.eat_bob } else { 0.0 };
    let head_x = p.get("head")[3];
    let side = if left { 1.0 } else { -1.0 };
    let goal = [head_x - up - chew, side * across, -side * 0.12];
    let q = p.get(arm);
    for i in 0..3 {
        q[3 + i] += (goal[i] - q[3 + i]) * k;
    }
}
