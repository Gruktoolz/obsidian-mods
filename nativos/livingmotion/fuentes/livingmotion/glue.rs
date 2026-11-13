// Living Motion for Horizon: the game side, injected as `crate::game::livingmotion`.
// Once a frame it hands every player's movement to the rig
// (`horizon_client::mobs::livingmotion`), which the model hooks read; it
// poses your first-person hands from the same rig, and keeps the settings,
// the page in Options > Mods and the keys. Elytra trails are in
// `crate::game::livingmotion_trails` (MPL-2.0).

use super::Game;
use super::livingmotion_trails as trails;
use horizon_client::mobs::livingmotion as rig;
use horizon_client::mod_api::NativeMod;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::sync::Mutex;
use std::time::Instant;

pub(crate) const ID: &str = "livingmotion";
const NAME: &str = "Living Motion";
const VERSION: &str = "1.2.0";
const DESCRIPTION: &str = "Livelier players in third and first person: lean, bank, bounce, landing squash, jump poses, swings that follow through, mining, eating and drinking to the mouth, maps in both hands, compasses, lanterns, climbing hand over hand, item inspect, elytra rolls, twirls and contrails, swim strokes.";
const KEY_TOGGLE: (&str, &str, &str) = ("toggle", "Toggle Living Motion", "key.keyboard.y");
const KEY_INSPECT: (&str, &str, &str) = ("inspect", "Inspect held item", "key.keyboard.i");
const KEY_TWIRL_LEFT: (&str, &str, &str) = ("twirl_left", "Elytra twirl left", "key.keyboard.comma");
const KEY_TWIRL_RIGHT: (&str, &str, &str) = ("twirl_right", "Elytra twirl right", "key.keyboard.period");
const KEYS: [(&str, &str, &str); 4] = [KEY_TOGGLE, KEY_INSPECT, KEY_TWIRL_LEFT, KEY_TWIRL_RIGHT];

/// `config.json`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
struct Config {
    enabled: bool,
    others: bool,
    lean: f32,
    bank: f32,
    bounce: f32,
    landing: f32,
    arms: f32,
    air: f32,
    idle: f32,
    elytra: f32,
    swim: f32,
    items: f32,
    climb: f32,
    actions: f32,
    first_person: bool,
    first_person_amount: f32,
    steady_head: bool,
    twirls: bool,
    trails: trails::TrailConfig,
}
impl Default for Config {
    fn default() -> Self {
        Self::from((rig::Settings::default(), true, trails::TrailConfig::default()))
    }
}
impl From<(rig::Settings, bool, trails::TrailConfig)> for Config {
    fn from((s, twirls, trails): (rig::Settings, bool, trails::TrailConfig)) -> Self {
        Self {
            enabled: s.enabled,
            others: s.others,
            lean: s.lean,
            bank: s.bank,
            bounce: s.bounce,
            landing: s.landing,
            arms: s.arms,
            air: s.air,
            idle: s.idle,
            elytra: s.elytra,
            swim: s.swim,
            items: s.items,
            climb: s.climb,
            actions: s.actions,
            first_person: s.first_person,
            first_person_amount: s.first_person_amount,
            steady_head: s.steady_head,
            twirls,
            trails,
        }
    }
}
impl Config {
    fn settings(self) -> rig::Settings {
        let a = |v: f32| if v.is_finite() { v.clamp(0.0, 2.0) } else { 1.0 };
        rig::Settings {
            enabled: self.enabled,
            others: self.others,
            lean: a(self.lean),
            bank: a(self.bank),
            bounce: a(self.bounce),
            landing: a(self.landing),
            arms: a(self.arms),
            air: a(self.air),
            idle: a(self.idle),
            elytra: a(self.elytra),
            swim: a(self.swim),
            items: a(self.items),
            climb: a(self.climb),
            actions: a(self.actions),
            first_person: self.first_person,
            first_person_amount: a(self.first_person_amount),
            steady_head: self.steady_head,
        }
    }
    fn trails(self) -> trails::TrailConfig {
        let mut t = self.trails;
        let f = |v: f32, lo: f32, hi: f32, d: f32| if v.is_finite() { v.clamp(lo, hi) } else { d };
        t.width = f(t.width, 0.0, 2.0, 0.05);
        t.lifetime = f(t.lifetime, 0.05, 30.0, 2.5);
        t.min_speed = f(t.min_speed, 0.0, 10.0, 0.8);
        t.alpha = f(t.alpha, 0.0, 1.0, 1.0);
        t.color = t.color.min(trails::COLORS.len() - 1);
        t
    }
}

/// A damped spring for the first-person sway.
#[derive(Clone, Copy, Default)]
struct Spring {
    x: f32,
    v: f32,
}
impl Spring {
    fn step(&mut self, target: f32, k: f32, d: f32, dt: f32) {
        let n = (dt / (1.0 / 120.0)).ceil().clamp(1.0, 8.0);
        let h = dt / n;
        for _ in 0..n as u32 {
            self.v += ((target - self.x) * k - self.v * d) * h;
            self.x += self.v * h;
        }
    }
}

struct State {
    loaded: bool,
    dir: Option<std::path::PathBuf>,
    last: Option<Instant>,
    start: Option<Instant>,
    twirls: bool,
    /// Keys pressed since the last frame.
    inspect: bool,
    twirl: i8,
    /// First person: the view's last angles and the hands' lag behind it (degrees).
    look: Option<(f32, f32)>,
    sway: [Spring; 2],
}
static STATE: Mutex<State> = Mutex::new(State {
    loaded: false,
    dir: None,
    last: None,
    start: None,
    twirls: true,
    inspect: false,
    twirl: 0,
    look: None,
    sway: [Spring { x: 0.0, v: 0.0 }, Spring { x: 0.0, v: 0.0 }],
});

fn state() -> std::sync::MutexGuard<'static, State> {
    STATE.lock().unwrap_or_else(|e| e.into_inner())
}

thread_local! {
    /// This frame's players, kept to fill again without allocating.
    static INPUTS: RefCell<Vec<rig::Input>> = const { RefCell::new(Vec::new()) };
}

fn ensure_loaded(game: &Game<'_>) {
    let mut s = state();
    if s.loaded {
        return;
    }
    let dir = game.native_data_dir(ID);
    let cfg = std::fs::read_to_string(dir.join("config.json"))
        .ok()
        .and_then(|t| serde_json::from_str::<Config>(&t).ok())
        .unwrap_or_default();
    rig::set_settings(cfg.settings());
    trails::set_config(cfg.trails());
    s.twirls = cfg.twirls;
    s.dir = Some(dir);
    s.loaded = true;
}

fn save() {
    let s = state();
    let Some(dir) = &s.dir else { return };
    let _ = std::fs::create_dir_all(dir);
    let cfg = Config::from((rig::settings(), s.twirls, trails::config()));
    if let Ok(text) = serde_json::to_string_pretty(&cfg) {
        let tmp = dir.join("config.json.tmp");
        if std::fs::write(&tmp, text).is_ok() {
            let _ = std::fs::rename(&tmp, dir.join("config.json"));
        }
    }
}

fn wrap(a: f32) -> f32 {
    (a + 180.0).rem_euclid(360.0) - 180.0
}

// ---------------------------------------------------------------- once a frame

pub(crate) fn frame(game: &mut Game<'_>) {
    let now = Instant::now();
    let (dt, inspect, twirl) = {
        let mut s = state();
        let dt = s.last.map_or(0.0, |t| now.duration_since(t).as_secs_f32());
        s.last = Some(now);
        s.start.get_or_insert(now);
        let twirl = if s.twirls { s.twirl } else { 0 };
        let keys = (std::mem::take(&mut s.inspect), twirl);
        s.twirl = 0;
        (dt, keys.0, keys.1)
    };
    if !game.native_on(ID) {
        rig::update(&[], dt);
        return;
    }
    ensure_loaded(game);
    let g: &Game<'_> = game;
    INPUTS.with(|cell| {
        let mut inputs = cell.borrow_mut();
        inputs.clear();
        if let Some(m) = g.player_model.as_ref() {
            let (busy, gliding, swimming) =
                g.player.as_ref().map_or((false, false, false), |p| (p.sleeping, p.fall_flying, p.swimming));
            let mut input = rig::Input::with(m, busy, gliding, swimming);
            input.mining = g.mining.is_some();
            input.inspect = inspect;
            input.twirl = twirl;
            inputs.push(input);
        }
        for m in g.mobs.iter() {
            if &*m.kind.id == "minecraft:player" && g.player_model.as_ref().is_none_or(|p| p.id != m.id) {
                inputs.push(rig::Input::of(m, false));
            }
        }
        rig::update(&inputs, dt);
    });
    // First person: the hands lag behind the view and catch up.
    let (yaw, pitch) = (game.yaw, game.pitch);
    let mut s = state();
    let (dyaw, dpitch) = match s.look {
        Some((y, p)) => (wrap(yaw - y), pitch - p),
        None => (0.0, 0.0),
    };
    s.look = Some((yaw, pitch));
    let per_second = |d: f32| if dt > 0.0 { d / dt } else { 0.0 };
    let lag = [(-per_second(dyaw) * 0.025).clamp(-9.0, 9.0), (-per_second(dpitch) * 0.025).clamp(-7.0, 7.0)];
    for (spring, target) in s.sway.iter_mut().zip(lag) {
        spring.step(target, 90.0, 14.0, dt.clamp(0.0, 0.1));
    }
}

// ---------------------------------------------------------------- first person

/// Your hands in first person, from the same rig the third-person model
/// uses: sway behind the view, bob with the stride, dip on landing, lower at a
/// sprint and in flight, follow a swing through, press while mining, chew,
/// turn an item over to inspect it. Offsets in blocks, angles in degrees.
#[allow(clippy::too_many_arguments)]
pub(crate) fn hand_pose(
    game: &Game<'_>,
    hand: horizon_client::first_person::Hand,
    has_item: bool,
    partial: f32,
    swing: f32,
    x: &mut f32,
    y: &mut f32,
    z: &mut f32,
    pitch: &mut f32,
    yaw: &mut f32,
    roll: &mut f32,
) {
    let set = rig::settings();
    if !set.enabled || !set.first_person || !game.native_on(ID) {
        return;
    }
    let Some(m) = game.player_model.as_ref() else { return };
    let Some(o) = rig::current(m.id) else { return };
    let main = hand == horizon_client::first_person::Hand::Main;
    // Which side of the screen: the main hand on the main arm.
    let left = game.settings.main_hand_left != !main;
    let s = if left { -1.0 } else { 1.0 };
    let k = set.first_person_amount * o.weight.max(o.glide.max(o.swim));
    let (sway, age) = {
        let st = state();
        ([st.sway[0].x, st.sway[1].x], st.start.map_or(0.0, |t| t.elapsed().as_secs_f32()))
    };
    let (mut dx, mut dy, mut dz, mut dp, mut dyw, mut dr) = (0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32);
    // Sway: behind the view as it turns.
    dyw += sway[0];
    dp += sway[1];
    dx += sway[0] * 0.003;
    dy -= sway[1] * 0.002;
    // Stride: a small figure eight, and a roll with each step.
    let pos = horizon_client::mobs::anim::walk_position(m, partial) * 0.6662;
    let speed = (m.previous_limb_speed + (m.limb_speed - m.previous_limb_speed) * partial).clamp(0.0, 1.0) * (1.0 - o.glide);
    dx += s * pos.sin() * 0.014 * speed;
    dy -= pos.cos().abs() * 0.02 * speed;
    dr += s * pos.sin() * 1.6 * speed;
    // A sprint lowers and tilts the hands; landing dips them; a jump lifts them.
    dy -= 0.035 * o.sprinting * speed;
    dp += 7.0 * o.sprinting * speed;
    dr += s * 4.0 * o.sprinting * speed;
    dy -= o.squash * 0.3;
    dy += 0.035 * o.rise;
    // Gliding: lower, braced.
    dy -= 0.07 * o.glide;
    dp += 10.0 * o.glide;
    // A swing follows through: across and rolled over, a punch forward.
    if swing > 0.0 && main {
        let f = (swing * std::f32::consts::PI).sin() * o.actions;
        dyw -= s * 12.0 * f;
        dr -= s * 16.0 * f;
        dz -= 0.05 * f;
    }
    // Mining: pressed in, with the rhythm of the strokes.
    if main {
        dz -= 0.035 * o.mining;
        dp -= (5.0 + (age * 9.0).sin() * 3.0) * o.mining;
    }
    // Eating: a chew; drinking: a slow tilt (vanilla brings the item up).
    let eating_here = o.eat > 0.001 && (o.eat_left == left);
    if eating_here {
        if o.drink {
            dr += s * 6.0 * o.eat;
            dp -= 4.0 * o.eat;
        } else {
            dy += (age * 11.0).sin().abs() * 0.022 * o.eat;
            dr += s * (age * 11.0).sin() * 3.0 * o.eat;
        }
    }
    // Inspect: the held item to the middle, turned one way, then the other.
    let ib = rig::inspect_blend(o.inspect);
    if ib > 0.0 {
        if main && has_item {
            let t = o.inspect;
            dx -= s * 0.26 * ib;
            dy += 0.07 * ib;
            dz += 0.06 * ib;
            dyw += s * (-30.0 + 55.0 * (t * 1.3).sin()) * ib;
            dr += 22.0 * (t * 2.1).sin() * ib;
            dp -= 12.0 * ib;
        } else {
            // The other hand makes room.
            dy -= 0.12 * ib;
        }
    }
    *x += dx * k;
    *y += dy * k;
    *z += dz * k;
    *pitch += dp * k;
    *yaw += dyw * k;
    *roll += dr * k;
}

// ---------------------------------------------------------------- the world: trails

pub(crate) fn world(game: &Game<'_>, camera: glam::DVec3, partial: f64, out: &mut Vec<horizon_client::mesher::Vertex>) {
    if !game.native_window() {
        return;
    }
    ensure_loaded(game);
    trails::world(game, camera, partial, out);
}

// ---------------------------------------------------------------- Options > Mods, the keys

pub(crate) fn listed(mods: &mut Vec<NativeMod>) {
    mods.push(NativeMod {
        id: ID.into(),
        title: format!("{NAME} {VERSION}"),
        description: DESCRIPTION.into(),
        keys: KEYS.iter().map(|(n, l, d)| [(*n).to_owned(), (*l).to_owned(), (*d).to_owned()]).collect(),
    });
}

/// A setting's field in [`rig::Settings`].
type Field = fn(&mut rig::Settings) -> &mut f32;
const AMOUNTS: [(&str, Field); 13] = [
    ("Lean into runs", |s| &mut s.lean),
    ("Bank into turns", |s| &mut s.bank),
    ("Stride bounce", |s| &mut s.bounce),
    ("Landing squash", |s| &mut s.landing),
    ("Arm swing", |s| &mut s.arms),
    ("Jump pose", |s| &mut s.air),
    ("Idle breathing", |s| &mut s.idle),
    ("Elytra flight", |s| &mut s.elytra),
    ("Swimming", |s| &mut s.swim),
    ("Item poses (eat, map, lantern)", |s| &mut s.items),
    ("Climbing", |s| &mut s.climb),
    ("Swings, mining, inspect", |s| &mut s.actions),
    ("First-person strength", |s| &mut s.first_person_amount),
];

fn on_off(b: bool) -> &'static str {
    if b { "ON" } else { "OFF" }
}

/// The page's buttons, in order: (label, what a click does).
enum Button {
    Enabled,
    Others,
    FirstPerson,
    Amount(usize),
    SteadyHead,
    Twirls,
    Trails,
    TrailsOthers,
    TrailPreset,
    TrailColor,
    TrailWidth,
    TrailLength,
    TrailSpeed,
    TrailGlow,
    TrailFade,
    Reset,
}
fn buttons() -> Vec<Button> {
    let mut b = vec![Button::Enabled, Button::Others, Button::FirstPerson];
    b.extend((0..AMOUNTS.len()).map(Button::Amount));
    b.extend([
        Button::SteadyHead,
        Button::Twirls,
        Button::Trails,
        Button::TrailsOthers,
        Button::TrailPreset,
        Button::TrailColor,
        Button::TrailWidth,
        Button::TrailLength,
        Button::TrailSpeed,
        Button::TrailGlow,
        Button::TrailFade,
        Button::Reset,
    ]);
    b
}

const WIDTHS: [f32; 7] = [0.03, 0.05, 0.075, 0.1, 0.15, 0.25, 0.5];
const LENGTHS: [f32; 6] = [0.6, 1.0, 2.5, 4.0, 6.0, 10.0];
const SPEEDS: [f32; 5] = [0.0, 0.5, 0.75, 0.8, 1.2];

/// The next value after `v` in `steps`, round again.
fn next(v: f32, steps: &[f32]) -> f32 {
    steps.iter().copied().find(|s| *s > v + 1e-4).unwrap_or(steps[0])
}

pub(crate) fn settings_page(game: &mut Game<'_>, mod_id: &str, out: &mut Vec<String>) {
    if mod_id != ID {
        return;
    }
    ensure_loaded(game);
    let mut s = rig::settings();
    let t = trails::config();
    let twirls = state().twirls;
    let preset = trails::PRESETS.iter().find(|(_, p)| {
        let mut p = *p;
        p.enabled = t.enabled;
        p.others = t.others;
        p.min_speed = t.min_speed;
        p == t
    });
    for b in buttons() {
        out.push(match b {
            Button::Enabled => format!("Living Motion: {}", on_off(s.enabled)),
            Button::Others => format!("Other players: {}", on_off(s.others)),
            Button::FirstPerson => format!("First-person animations: {}", on_off(s.first_person)),
            Button::Amount(i) => format!("{}: {:.0}%", AMOUNTS[i].0, *AMOUNTS[i].1(&mut s) * 100.0),
            Button::SteadyHead => format!("Steady head: {}", on_off(s.steady_head)),
            Button::Twirls => format!("Elytra twirls (, and .): {}", on_off(twirls)),
            Button::Trails => format!("Elytra trails: {}", on_off(t.enabled)),
            Button::TrailsOthers => format!("Other players' trails: {}", on_off(t.others)),
            Button::TrailPreset => format!("Trail preset: {}", preset.map_or("Custom", |p| p.0)),
            Button::TrailColor => format!("Trail colour: {}", trails::COLORS[t.color.min(trails::COLORS.len() - 1)].0),
            Button::TrailWidth => format!("Trail width: {:.2} blocks", t.width),
            Button::TrailLength => format!("Trail duration: {:.1} s", t.lifetime),
            Button::TrailSpeed => format!("Trail minimum speed: {:.2}", t.min_speed),
            Button::TrailGlow => format!("Glowing trails: {}", on_off(t.glowing)),
            Button::TrailFade => format!("Trails fade out: {}", on_off(t.fade)),
            Button::Reset => "Reset to defaults".to_owned(),
        });
    }
}

pub(crate) fn settings_click(game: &mut Game<'_>, mod_id: &str, button: usize) {
    if mod_id != ID {
        return;
    }
    ensure_loaded(game);
    let Some(b) = buttons().into_iter().nth(button) else { return };
    let mut s = rig::settings();
    let mut t = trails::config();
    match b {
        Button::Enabled => s.enabled = !s.enabled,
        Button::Others => s.others = !s.others,
        Button::FirstPerson => s.first_person = !s.first_person,
        Button::Amount(i) => {
            // 0% to 200% in steps of 25%, round again.
            let v = AMOUNTS[i].1(&mut s);
            *v = if *v >= 1.999 { 0.0 } else { ((*v / 0.25).round() + 1.0) * 0.25 };
        }
        Button::SteadyHead => s.steady_head = !s.steady_head,
        Button::Twirls => {
            let mut st = state();
            st.twirls = !st.twirls;
        }
        Button::Trails => t.enabled = !t.enabled,
        Button::TrailsOthers => t.others = !t.others,
        Button::TrailPreset => {
            let i = trails::PRESETS.iter().position(|(_, p)| p.width == t.width && p.color == t.color && p.lifetime == t.lifetime);
            let (enabled, others, min_speed) = (t.enabled, t.others, t.min_speed);
            t = trails::PRESETS[i.map_or(0, |i| (i + 1) % trails::PRESETS.len())].1;
            (t.enabled, t.others, t.min_speed) = (enabled, others, min_speed);
        }
        Button::TrailColor => t.color = (t.color + 1) % trails::COLORS.len(),
        Button::TrailWidth => t.width = next(t.width, &WIDTHS),
        Button::TrailLength => t.lifetime = next(t.lifetime, &LENGTHS),
        Button::TrailSpeed => t.min_speed = next(t.min_speed, &SPEEDS),
        Button::TrailGlow => t.glowing = !t.glowing,
        Button::TrailFade => t.fade = !t.fade,
        Button::Reset => {
            s = rig::Settings::default();
            t = trails::TrailConfig::default();
            state().twirls = true;
        }
    }
    rig::set_settings(s);
    trails::set_config(t);
    save();
}

fn bound(game: &Game<'_>, k: (&str, &str, &str), key: &str) -> bool {
    game.native_mod_key(ID, k.0, k.2) == key
}

pub(crate) fn key(game: &mut Game<'_>, key: &str, pressed: bool) {
    if !pressed || !game.native_window() || !game.native_playing() || !game.native_on(ID) {
        return;
    }
    ensure_loaded(game);
    if bound(game, KEY_TOGGLE, key) {
        let mut s = rig::settings();
        s.enabled = !s.enabled;
        rig::set_settings(s);
        save();
        game.native_chat(if s.enabled { "[Living Motion] On." } else { "[Living Motion] Off: vanilla animations." });
    } else if bound(game, KEY_INSPECT, key) {
        state().inspect = true;
    } else if bound(game, KEY_TWIRL_LEFT, key) {
        state().twirl = -1;
    } else if bound(game, KEY_TWIRL_RIGHT, key) {
        state().twirl = 1;
    }
}
