// Living Camera for Horizon: the camera leans into strafes, tips with
// jumps and falls, sways when you turn fast, dips when you land, shakes when
// you're hit, bobs with your steps and breathes when you stand still, all on
// springs so it eases in and settles; the first-person hand lags behind the
// view and the FOV can widen with speed. Every effect has its own strength on
// the settings page, with presets, and it calms down while you aim or ride.
// Inspired by Mirsario's Camera Overhaul, written from scratch on Horizon's
// hook points (none of its code). This file is both the mod's Cargo library
// (its tests) and the module the bundle adds to the game.
//
// Movement comes from the player's position each tick, turning from the view
// each frame. Fast travel (elytra, boats, minecarts) fades the effects out,
// so it leaves gyro's elytra camera alone.
use std::cell::RefCell;
use std::f32::consts::TAU;

/// The mod's id (its settings folder), name and version.
pub const ID: &str = "living_camera";
pub const NAME: &str = "Living Camera";
pub const VERSION: &str = "1.1.0";
pub const DESCRIPTION: &str = "The camera leans into strafes, tips with jumps and falls, sways on turns, dips on landing, bobs with your steps and breathes. Every effect is adjustable.";

/// The key that turns the whole mod on and off (unbound until the player
/// picks one in Options > Mods).
pub const TOGGLE_KEY: &str = "toggle";
pub const TOGGLE_KEY_LABEL: &str = "Toggle Living Camera";
pub const TOGGLE_KEY_DEFAULT: &str = "key.keyboard.unknown";

/// Degrees of roll per block a second of sideways speed, at 100%.
const STRAFE_ROLL: f32 = 0.7;
/// Degrees of pitch per block a second of forward speed (a slight lean).
const FORWARD_PITCH: f32 = 0.12;
/// Degrees of pitch per block a second of vertical speed (up looks up).
const VERTICAL_PITCH: f32 = 0.25;
/// Degrees of roll per degree a second of turning.
const TURN_ROLL: f32 = 0.008;
/// Largest target on each effect at 100%, in degrees.
const MAX_ROLL: f32 = 8.0;
const MAX_PITCH: f32 = 6.0;
/// A fall at least this fast (blocks a second) dips the camera on landing.
const LANDING_SPEED: f32 = 6.0;
/// Push (blocks a second) per block a second of landing speed, and its limit.
const LANDING_KICK: f32 = 0.25;
const MAX_KICK: f32 = 6.0;
/// The dip's limit, in blocks.
const MAX_DIP: f32 = 0.35;
/// Step bob: blocks walked per bob, and its height (blocks) and roll
/// (degrees) at walking speed.
const STEP_LENGTH: f32 = 1.3;
const BOB_HEIGHT: f32 = 0.035;
const BOB_ROLL: f32 = 0.6;
const WALK_SPEED: f32 = 4.3;
/// Idle breathing: one breath every few seconds, its pitch (degrees) and rise
/// (blocks).
const BREATH_SECONDS: f32 = 4.0;
const BREATH_PITCH: f32 = 0.35;
const BREATH_RISE: f32 = 0.006;
/// Speed FOV: the FOV scale per block a second past walking, and its most.
const FOV_PER_SPEED: f32 = 0.008;
const MAX_FOV: f32 = 0.15;
/// The fast-travel fade goes from all to nothing over this many blocks a second.
const FADE_SPAN: f32 = 8.0;
/// Damage shake: degrees a second of roll and pitch kick per health point
/// lost, the most health counted for one hit, and its springs.
const HIT_ROLL: f32 = 12.0;
const HIT_PITCH: f32 = 8.0;
const MAX_HIT: f32 = 10.0;
const HIT_STIFFNESS: f32 = 350.0;
const HIT_DAMPING: f32 = 0.35;
/// Seconds without word from the health HUD before its health and mount are forgotten.
const HEALTH_STALE: f32 = 0.5;
/// The hand's lag: blocks per degree a second of turning, and its limit.
const HAND_LAG: f32 = 0.0004;
const MAX_HAND_LAG: f32 = 0.08;

/// One option of the settings page: its file, its label, the values a click
/// cycles through (with their names) and the default.
pub struct Opt {
    pub key: &'static str,
    pub label: &'static str,
    pub choices: &'static [(u16, &'static str)],
    pub default: u16,
}

const ON_OFF: &[(u16, &str)] = &[(0, "OFF"), (1, "ON")];
const STRENGTH: &[(u16, &str)] = &[
    (0, "OFF"),
    (25, "25%"),
    (50, "50%"),
    (75, "75%"),
    (100, "100%"),
    (150, "150%"),
    (200, "200%"),
];
const INTENSITY: &[(u16, &str)] = &[
    (25, "25%"),
    (50, "50%"),
    (75, "75%"),
    (100, "100%"),
    (125, "125%"),
    (150, "150%"),
    (200, "200%"),
    (300, "300%"),
];
const CONTEXT: &[(u16, &str)] = &[(0, "OFF"), (25, "25%"), (50, "50%"), (75, "75%"), (100, "100%")];
/// Spring stiffness.
const SMOOTHNESS: &[(u16, &str)] = &[(300, "Snappy"), (120, "Normal"), (60, "Smooth"), (30, "Floaty")];
/// Damping, in percent of critical: lower bounces more.
const BOUNCE: &[(u16, &str)] = &[(100, "None"), (90, "Low"), (80, "Normal"), (65, "High"), (50, "Very high")];
/// Horizontal speed (blocks a second) where effects start to fade; 0 never.
const FAST_TRAVEL: &[(u16, &str)] = &[
    (0, "Never"),
    (10, "Above 10 b/s"),
    (12, "Above 12 b/s"),
    (16, "Above 16 b/s"),
    (24, "Above 24 b/s"),
];
const DIRECTION: &[(u16, &str)] = &[(0, "Normal"), (1, "Inverted")];

/// The options, in page order. `O::*` index it.
/// The presets; Custom is what the settings become once one is changed by hand.
const PRESET: &[(u16, &str)] = &[(0, "Custom"), (1, "Subtle"), (2, "Default"), (3, "Cinematic"), (4, "PvP")];

pub const OPTS: [Opt; 21] = [
    Opt { key: "enabled", label: "Living Camera", choices: ON_OFF, default: 1 },
    Opt { key: "preset", label: "Preset", choices: PRESET, default: 2 },
    Opt { key: "intensity", label: "Overall intensity", choices: INTENSITY, default: 100 },
    Opt { key: "strafe", label: "Strafe lean", choices: STRENGTH, default: 100 },
    Opt { key: "forward", label: "Forward lean", choices: STRENGTH, default: 100 },
    Opt { key: "turn", label: "Turn sway", choices: STRENGTH, default: 100 },
    Opt { key: "vertical", label: "Jump and fall tilt", choices: STRENGTH, default: 100 },
    Opt { key: "landing", label: "Landing dip", choices: STRENGTH, default: 100 },
    Opt { key: "damage", label: "Damage shake", choices: STRENGTH, default: 100 },
    Opt { key: "step_bob", label: "Step bob", choices: STRENGTH, default: 50 },
    Opt { key: "breathing", label: "Idle breathing", choices: STRENGTH, default: 0 },
    Opt { key: "speed_fov", label: "Speed FOV", choices: STRENGTH, default: 0 },
    Opt { key: "hand", label: "Hand sway", choices: STRENGTH, default: 100 },
    Opt { key: "smoothness", label: "Smoothness", choices: SMOOTHNESS, default: 120 },
    Opt { key: "bounce", label: "Bounce", choices: BOUNCE, default: 80 },
    Opt { key: "swimming", label: "While swimming", choices: CONTEXT, default: 50 },
    Opt { key: "flying", label: "While flying (creative)", choices: CONTEXT, default: 75 },
    Opt { key: "riding", label: "While riding mounts", choices: CONTEXT, default: 75 },
    Opt { key: "aiming", label: "While aiming (bow, spyglass)", choices: CONTEXT, default: 25 },
    Opt { key: "fast_travel", label: "Fade out when fast", choices: FAST_TRAVEL, default: 12 },
    Opt { key: "direction", label: "Lean direction", choices: DIRECTION, default: 0 },
];

/// Indexes into `OPTS`.
pub mod o {
    pub const ENABLED: usize = 0;
    pub const PRESET: usize = 1;
    pub const INTENSITY: usize = 2;
    pub const STRAFE: usize = 3;
    pub const FORWARD: usize = 4;
    pub const TURN: usize = 5;
    pub const VERTICAL: usize = 6;
    pub const LANDING: usize = 7;
    pub const DAMAGE: usize = 8;
    pub const STEP_BOB: usize = 9;
    pub const BREATHING: usize = 10;
    pub const SPEED_FOV: usize = 11;
    pub const HAND: usize = 12;
    pub const SMOOTHNESS: usize = 13;
    pub const BOUNCE: usize = 14;
    pub const SWIMMING: usize = 15;
    pub const FLYING: usize = 16;
    pub const RIDING: usize = 17;
    pub const AIMING: usize = 18;
    pub const FAST_TRAVEL: usize = 19;
    pub const DIRECTION: usize = 20;
}

/// What a preset sets, in this order (the rest keep their values).
pub const PRESET_KEYS: [usize; 15] = [
    o::INTENSITY,
    o::STRAFE,
    o::FORWARD,
    o::TURN,
    o::VERTICAL,
    o::LANDING,
    o::DAMAGE,
    o::STEP_BOB,
    o::BREATHING,
    o::SPEED_FOV,
    o::HAND,
    o::SMOOTHNESS,
    o::BOUNCE,
    o::RIDING,
    o::AIMING,
];

/// Each preset's values for `PRESET_KEYS`. Default is the options' own defaults.
pub fn preset_values(preset: u16) -> Option<[u16; 15]> {
    Some(match preset {
        // A light touch.
        1 => [75, 50, 50, 50, 50, 50, 50, 25, 0, 0, 50, 120, 90, 50, 50],
        2 => PRESET_KEYS.map(|i| OPTS[i].default),
        // Stronger, floatier, bouncier: for videos.
        3 => [125, 150, 100, 150, 150, 150, 150, 75, 50, 50, 150, 60, 65, 100, 50],
        // Few distractions: small lean, no bob, quick springs, steady aim.
        4 => [75, 50, 25, 25, 0, 50, 50, 0, 0, 0, 25, 300, 100, 25, 0],
        _ => return None,
    })
}

/// The label of the button after the options.
pub const RESET_LABEL: &str = "Reset to defaults";

/// The settings, one file each in the mod's data folder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    values: [u16; OPTS.len()],
}

impl Default for Settings {
    fn default() -> Self {
        Self { values: OPTS.map(|opt| opt.default) }
    }
}

impl Settings {
    /// From the stored values (`get` reads one key's file), the defaults for
    /// missing or unknown ones. Version 0.1 kept its effects as `true` and
    /// `false`: those read as the default and OFF.
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let mut s = Self::default();
        for (i, opt) in OPTS.iter().enumerate() {
            let Some(stored) = get(opt.key) else {
                continue;
            };
            let value = match stored.trim() {
                "false" => 0,
                "true" => opt.default,
                v => v.parse().unwrap_or(opt.default),
            };
            if opt.choices.iter().any(|c| c.0 == value) {
                s.values[i] = value;
            }
        }
        s
    }

    pub fn get(&self, i: usize) -> u16 {
        self.values[i]
    }

    /// A percent option as a factor (100% is 1).
    pub fn factor(&self, i: usize) -> f32 {
        f32::from(self.values[i]) / 100.0
    }

    pub fn enabled(&self) -> bool {
        self.values[o::ENABLED] != 0
    }

    /// The settings page's buttons: the options, then reset.
    pub fn labels(&self) -> Vec<String> {
        let mut labels: Vec<String> = OPTS
            .iter()
            .zip(self.values)
            .map(|(opt, v)| {
                let name = opt.choices.iter().find(|c| c.0 == v).map_or("?", |c| c.1);
                format!("{}: {name}", opt.label)
            })
            .collect();
        labels.push(RESET_LABEL.into());
        labels
    }

    /// A click on button `i` of `labels`: what to store, key by key.
    pub fn click(&mut self, i: usize) -> Vec<(&'static str, String)> {
        if i == OPTS.len() {
            *self = Self::default();
            return self.stored();
        }
        if i == o::PRESET {
            // Subtle, Default, Cinematic, PvP and round again (Custom isn't chosen, it happens).
            let next = self.values[o::PRESET] % 4 + 1;
            self.apply_preset(next);
            return self.stored();
        }
        let Some(opt) = OPTS.get(i) else {
            return Vec::new();
        };
        let at = opt.choices.iter().position(|c| c.0 == self.values[i]).unwrap_or(0);
        self.values[i] = opt.choices[(at + 1) % opt.choices.len()].0;
        let mut stored = vec![(opt.key, self.values[i].to_string())];
        if PRESET_KEYS.contains(&i) && self.values[o::PRESET] != 0 {
            self.values[o::PRESET] = 0;
            stored.push((OPTS[o::PRESET].key, "0".to_owned()));
        }
        stored
    }

    /// Sets the preset and the values it stands for.
    pub fn apply_preset(&mut self, preset: u16) {
        if let Some(values) = preset_values(preset) {
            for (i, v) in PRESET_KEYS.iter().zip(values) {
                self.values[*i] = v;
            }
            self.values[o::PRESET] = preset;
        }
    }

    /// Turns the whole mod on or off (the toggle key): what to store.
    pub fn toggle(&mut self) -> (&'static str, String) {
        self.values[o::ENABLED] = u16::from(!self.enabled());
        (OPTS[o::ENABLED].key, self.values[o::ENABLED].to_string())
    }

    /// Every setting, key by key.
    pub fn stored(&self) -> Vec<(&'static str, String)> {
        OPTS.iter().zip(self.values).map(|(opt, v)| (opt.key, v.to_string())).collect()
    }
}

/// The line in chat when the toggle key flips the mod.
pub fn toggled_line(on: bool) -> String {
    format!("{NAME} {}", if on { "ON" } else { "OFF" })
}

/// A damped spring easing `value` toward a target.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Spring {
    pub value: f32,
    velocity: f32,
}

impl Spring {
    /// One frame of `dt` seconds toward `target`, with `stiffness` and
    /// `damping` as a fraction of critical (below 1 settles with a bounce).
    pub fn step(&mut self, target: f32, dt: f32, stiffness: f32, damping: f32) {
        let c = 2.0 * stiffness.sqrt() * damping;
        // Small steps keep it stable through a long frame.
        let steps = (dt / 0.004).ceil().max(1.0);
        let h = dt / steps;
        for _ in 0..steps as u32 {
            let accel = stiffness * (target - self.value) - c * self.velocity;
            self.velocity += accel * h;
            self.value += self.velocity * h;
        }
    }

    /// A push, as a landing gives.
    pub fn kick(&mut self, velocity: f32) {
        self.velocity += velocity;
    }
}

/// Forward and sideways (to the right) speed for a world velocity `[x, z]`
/// and a yaw in degrees (0 faces south, +Z; 90 west, -X).
pub fn local(velocity: [f64; 2], yaw: f32) -> (f32, f32) {
    let (s, c) = yaw.to_radians().sin_cos();
    let [x, z] = velocity;
    let forward = -x as f32 * s + z as f32 * c;
    let right = -x as f32 * c - z as f32 * s;
    (forward, right)
}

/// The player's eyes above their feet, standing.
pub const EYE_HEIGHT: f64 = 1.62;

/// Whether a block the eyes are in is water (swimming).
pub fn is_water(block: &str) -> bool {
    matches!(
        block,
        "minecraft:water"
            | "minecraft:bubble_column"
            | "minecraft:kelp"
            | "minecraft:kelp_plant"
            | "minecraft:seagrass"
            | "minecraft:tall_seagrass"
    )
}

/// How much of the effects horizontal speed `bps` leaves, fading from
/// `from` blocks a second (never when it is 0).
pub fn speed_fade(bps: f32, from: u16) -> f32 {
    if from == 0 {
        return 1.0;
    }
    (1.0 - (bps - f32::from(from)) / FADE_SPAN).clamp(0.0, 1.0)
}

/// What the camera and hand should do this frame.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Pose {
    pub pitch: f32,
    pub roll: f32,
    /// Blocks up (negative down).
    pub rise: f32,
    pub hand: [f32; 2],
    pub hand_roll: f32,
    /// Multiplies the FOV.
    pub fov: f32,
}

#[derive(Default)]
struct Motion {
    last_pos: Option<[f64; 3]>,
    /// Blocks a second: forward, right, up; and horizontal speed.
    forward: f32,
    right: f32,
    up: f32,
    horizontal: f32,
    falling: f32,
    landed: f32,
    on_ground: bool,
    swimming: bool,
    flying: bool,
    /// Drawing a bow or looking through a spyglass (the FOV hook tells it).
    aiming: bool,
    /// On a living mount (the health HUD tells it).
    riding: bool,
    /// The health the HUD last showed, the damage since, and seconds since
    /// the HUD last told (it doesn't under F1 or in creative).
    last_health: Option<f32>,
    hurt: f32,
    health_age: f32,
    /// The side the next damage shake rolls to.
    hit_left: bool,
    last_view: Option<[f32; 2]>,
    /// Degrees a second, smoothed.
    yaw_rate: f32,
    pitch_rate: f32,
    /// Step bob's phase (radians) and how much of it shows.
    step_phase: f32,
    step_amount: f32,
    /// Seconds, for breathing.
    time: f32,
}

#[derive(Default)]
struct State {
    settings: Option<Settings>,
    motion: Motion,
    pitch: Spring,
    roll: Spring,
    dip: Spring,
    hand_x: Spring,
    hand_y: Spring,
    fov: Spring,
    /// The damage shake: stiff, lightly damped springs, apart from the lean.
    hit_roll: Spring,
    hit_pitch: Spring,
    /// Step bob and breathing, added on top of the springs.
    bob: [f32; 3],
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

/// The settings, read once through `get`, then kept.
pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    STATE.with_borrow_mut(|s| *s.settings.get_or_insert_with(|| Settings::read(get)))
}

/// Replaces the settings (the settings page or the key changed them).
pub fn set_settings(settings: Settings) {
    STATE.with_borrow_mut(|s| s.settings = Some(settings));
}

/// Whether the player flies in creative and whether they aim (the FOV hook tells it).
pub fn set_player(flying: bool, aiming: bool) {
    STATE.with_borrow_mut(|s| {
        s.motion.flying = flying;
        s.motion.aiming = aiming;
    });
}

/// The health HUD's view, each frame it draws: the player's health, and
/// whether they ride a living mount. A drop in health shakes the camera.
pub fn health(health: f32, riding: bool) {
    STATE.with_borrow_mut(|s| {
        let m = &mut s.motion;
        if let Some(last) = m.last_health
            && health < last - 0.01
        {
            m.hurt += last - health;
        }
        m.last_health = Some(health);
        m.riding = riding;
        m.health_age = 0.0;
    });
}

/// A tick on the window: the player at `pos` facing `yaw`, eyes in water or
/// not; none when not in a world.
pub fn tick(pos: Option<[f64; 3]>, yaw: f32, swimming: bool) {
    STATE.with_borrow_mut(|s| {
        let m = &mut s.motion;
        let Some(pos) = pos else {
            *m = Motion::default();
            return;
        };
        let Some(last) = m.last_pos.replace(pos) else {
            return;
        };
        let v = [(pos[0] - last[0]) * 20.0, (pos[2] - last[2]) * 20.0];
        let up = ((pos[1] - last[1]) * 20.0) as f32;
        let horizontal = v[0].hypot(v[1]) as f32;
        // A teleport isn't movement.
        if horizontal > 100.0 || up.abs() > 100.0 {
            return;
        }
        (m.forward, m.right) = local(v, yaw);
        m.horizontal = horizontal;
        // Landing: a fast fall that stopped.
        if m.falling <= -LANDING_SPEED && up > -0.5 {
            m.landed = -m.falling;
        }
        // On the ground: not moving up or down, two ticks running.
        m.on_ground = up.abs() < 0.01 && m.falling.abs() < 0.01;
        m.falling = up;
        m.up = up;
        m.swimming = swimming;
    });
}

/// A frame of `dt` seconds, the view at `[yaw, pitch]` (none when not
/// playing): the springs move toward what the motion asks.
pub fn frame(dt: f32, view: Option<[f32; 2]>) {
    let dt = dt.clamp(0.0, 0.1);
    if dt <= 0.0 {
        return;
    }
    STATE.with_borrow_mut(|s| {
        let set = s.settings.unwrap_or_default();
        let m = &mut s.motion;
        let (yaw_rate, pitch_rate) = match (view, m.last_view) {
            (Some([y, p]), Some([ly, lp])) => {
                // Yaw wraps: take the short way round.
                let dy = (y - ly + 540.0).rem_euclid(360.0) - 180.0;
                (dy / dt, (p - lp) / dt)
            }
            _ => (0.0, 0.0),
        };
        m.last_view = view;
        let ease = (dt * 20.0).min(1.0);
        m.yaw_rate += (yaw_rate - m.yaw_rate) * ease;
        m.pitch_rate += (pitch_rate - m.pitch_rate) * ease;
        m.time = (m.time + dt) % (BREATH_SECONDS * 1000.0);

        let fade = speed_fade(m.horizontal, set.get(o::FAST_TRAVEL));
        let mut scale = set.factor(o::INTENSITY) * fade;
        if m.swimming {
            scale *= set.factor(o::SWIMMING);
        }
        if m.flying {
            scale *= set.factor(o::FLYING);
        }
        // The HUD stopped telling (F1, creative, a screen over the HUD): forget its health and mount.
        m.health_age += dt;
        if m.health_age > HEALTH_STALE {
            m.last_health = None;
            m.riding = false;
        }
        if m.riding {
            scale *= set.factor(o::RIDING);
        }
        if m.aiming {
            scale *= set.factor(o::AIMING);
        }
        if view.is_none() || !set.enabled() {
            scale = 0.0;
        }
        let sign = if set.get(o::DIRECTION) == 1 { -1.0 } else { 1.0 };

        let roll = (m.right * STRAFE_ROLL).clamp(-MAX_ROLL, MAX_ROLL) * set.factor(o::STRAFE)
            + (m.yaw_rate * TURN_ROLL).clamp(-MAX_ROLL / 2.0, MAX_ROLL / 2.0) * set.factor(o::TURN);
        let mut pitch = (m.forward * FORWARD_PITCH).clamp(-MAX_PITCH, MAX_PITCH) * set.factor(o::FORWARD);
        if !m.flying {
            pitch -= (m.up * VERTICAL_PITCH).clamp(-MAX_PITCH, MAX_PITCH) * set.factor(o::VERTICAL);
        }
        let stiffness = f32::from(set.get(o::SMOOTHNESS));
        let damping = set.factor(o::BOUNCE);
        s.roll.step(sign * roll * scale, dt, stiffness, damping);
        s.pitch.step(pitch * scale, dt, stiffness, damping);

        if m.landed > 0.0 {
            let kick = (m.landed * LANDING_KICK).min(MAX_KICK) * set.factor(o::LANDING) * scale;
            s.dip.kick(-kick);
        }
        m.landed = 0.0;
        // The dip comes back on a stiffer spring, so it reads as a bump.
        s.dip.step(0.0, dt, stiffness * 1.5, damping);

        // Damage shake: a jolt as big as the hit (up to `MAX_HIT` health), to
        // alternating sides, that rings out on its own stiff springs.
        if m.hurt > 0.0 {
            let hit = m.hurt.min(MAX_HIT) * set.factor(o::DAMAGE) * scale;
            let side = if m.hit_left { -1.0 } else { 1.0 };
            m.hit_left = !m.hit_left;
            s.hit_roll.kick(side * hit * HIT_ROLL);
            s.hit_pitch.kick(-hit * HIT_PITCH);
        }
        m.hurt = 0.0;
        s.hit_roll.step(0.0, dt, HIT_STIFFNESS, HIT_DAMPING);
        s.hit_pitch.step(0.0, dt, HIT_STIFFNESS, HIT_DAMPING);

        // Step bob: on the ground, in step with the distance walked.
        let walking = m.on_ground && !m.swimming && !m.flying;
        let stride = (m.horizontal / WALK_SPEED).min(1.4);
        let target = if walking { stride } else { 0.0 };
        m.step_amount += (target - m.step_amount) * (dt * 8.0).min(1.0);
        m.step_phase = (m.step_phase + m.horizontal * dt / STEP_LENGTH * TAU) % TAU;
        let step = set.factor(o::STEP_BOB) * scale * m.step_amount;
        // Breathing: standing still, slowly in and out.
        let still = (1.0 - m.horizontal).clamp(0.0, 1.0);
        let breath = (m.time / BREATH_SECONDS * TAU).sin() * set.factor(o::BREATHING) * scale * still;
        s.bob = [
            breath * BREATH_PITCH,
            m.step_phase.sin() * BOB_ROLL * step * sign,
            -(m.step_phase.sin().abs()) * BOB_HEIGHT * step + breath * BREATH_RISE,
        ];

        let lag = |rate: f32| {
            (-rate * HAND_LAG).clamp(-MAX_HAND_LAG, MAX_HAND_LAG) * set.factor(o::HAND) * scale
        };
        s.hand_x.step(lag(m.yaw_rate), dt, stiffness, damping);
        s.hand_y.step(lag(-m.pitch_rate), dt, stiffness, damping);

        // Speed FOV: wider past walking speed. Not faded by fast travel, where it
        // matters most.
        let mut fov = 0.0;
        if view.is_some() && set.enabled() {
            let past = (m.horizontal - WALK_SPEED).max(0.0);
            fov = (past * FOV_PER_SPEED).min(MAX_FOV) * set.factor(o::SPEED_FOV) * set.factor(o::INTENSITY);
        }
        s.fov.step(fov, dt, 40.0, 1.0);
    });
}

/// The camera and hand pose for this frame.
pub fn pose() -> Pose {
    STATE.with_borrow(|s| {
        let hand = s.settings.unwrap_or_default().factor(o::HAND);
        Pose {
            pitch: s.pitch.value + s.bob[0] + s.hit_pitch.value,
            roll: s.roll.value + s.bob[1] + s.hit_roll.value,
            rise: (s.dip.value + s.bob[2]).clamp(-MAX_DIP, MAX_DIP),
            hand: [s.hand_x.value, s.hand_y.value],
            hand_roll: s.roll.value * 0.5 * hand.min(1.0),
            fov: 1.0 + s.fov.value.max(0.0),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stored<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        |key| pairs.iter().find(|p| p.0 == key).map(|p| p.1.to_owned())
    }

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-4
    }

    #[test]
    fn options_have_their_defaults_among_their_choices() {
        for opt in &OPTS {
            assert!(opt.choices.iter().any(|c| c.0 == opt.default), "{}", opt.key);
        }
        let keys: std::collections::HashSet<_> = OPTS.iter().map(|o| o.key).collect();
        assert_eq!(keys.len(), OPTS.len());
    }

    #[test]
    fn reads_new_and_old_settings() {
        assert_eq!(Settings::read(stored(&[])), Settings::default());
        let s = Settings::read(stored(&[
            ("intensity", "150"),
            ("hand", "false"),
            ("turn", "true"),
            ("strafe", "75"),
            ("step_bob", "33"),
            ("smoothness", "60\n"),
        ]));
        assert_eq!(s.get(o::INTENSITY), 150);
        assert_eq!(s.get(o::HAND), 0);
        assert_eq!(s.get(o::TURN), 100);
        assert_eq!(s.get(o::STRAFE), 75);
        assert_eq!(s.get(o::STEP_BOB), 50, "not a choice: the default");
        assert_eq!(s.get(o::SMOOTHNESS), 60);
    }

    #[test]
    fn buttons_cycle_store_and_reset() {
        let mut s = Settings::default();
        let labels = s.labels();
        assert_eq!(labels.len(), OPTS.len() + 1);
        assert_eq!(labels[o::ENABLED], "Living Camera: ON");
        assert_eq!(labels[o::SMOOTHNESS], "Smoothness: Normal");
        assert_eq!(labels[OPTS.len()], RESET_LABEL);
        assert_eq!(s.labels()[o::PRESET], "Preset: Default");
        // Changing an effect by hand makes the preset Custom.
        assert_eq!(
            s.click(o::STRAFE),
            [("strafe", "150".to_owned()), ("preset", "0".to_owned())]
        );
        assert_eq!(s.click(o::STRAFE), [("strafe", "200".to_owned())]);
        assert_eq!(s.click(o::STRAFE), [("strafe", "0".to_owned())]);
        assert_eq!(s.labels()[o::STRAFE], "Strafe lean: OFF");
        assert_eq!(s.labels()[o::PRESET], "Preset: Custom");
        assert_eq!(s.toggle(), ("enabled", "0".to_owned()));
        assert!(!s.enabled());
        let stored = s.click(OPTS.len());
        assert_eq!(s, Settings::default());
        assert_eq!(stored.len(), OPTS.len());
        assert!(s.click(99).is_empty());
        assert_eq!(toggled_line(false), "Living Camera OFF");
    }

    #[test]
    fn local_speed_follows_minecraft_yaw() {
        // Facing south (+Z): moving +Z is forward, moving -X is to the right.
        let (f, r) = local([0.0, 4.0], 0.0);
        assert!(close(f, 4.0) && close(r, 0.0));
        let (f, r) = local([-4.0, 0.0], 0.0);
        assert!(close(f, 0.0) && close(r, 4.0));
        // Facing west (-X): moving -X is forward, moving -Z is to the right.
        let (f, r) = local([-4.0, 0.0], 90.0);
        assert!(close(f, 4.0) && close(r, 0.0));
        let (_, r) = local([0.0, -4.0], 90.0);
        assert!(close(r, 4.0));
    }

    #[test]
    fn springs_settle_at_every_preset() {
        for &(stiffness, _) in SMOOTHNESS {
            for &(damping, _) in BOUNCE {
                let mut s = Spring::default();
                for _ in 0..600 {
                    s.step(5.0, 1.0 / 60.0, f32::from(stiffness), f32::from(damping) / 100.0);
                }
                assert!((s.value - 5.0).abs() < 0.01, "{stiffness} {damping}: {}", s.value);
                s.step(-5.0, 0.1, f32::from(stiffness), f32::from(damping) / 100.0);
                assert!(s.value.is_finite() && s.value.abs() < 10.0);
            }
        }
    }

    #[test]
    fn fast_travel_fades_out() {
        assert_eq!(speed_fade(5.6, 12), 1.0);
        assert!(close(speed_fade(16.0, 12), 0.5));
        assert_eq!(speed_fade(30.0, 12), 0.0);
        assert_eq!(speed_fade(60.0, 0), 1.0);
    }

    fn quiet() -> Settings {
        // Only the effect a test looks at.
        let mut s = Settings::default();
        for i in [o::STEP_BOB, o::BREATHING, o::SPEED_FOV] {
            s.values[i] = 0;
        }
        s
    }

    fn run(ticks: usize, mut pos: impl FnMut(usize) -> [f64; 3], yaw: f32) -> Pose {
        for t in 0..ticks {
            tick(Some(pos(t)), yaw, false);
            for _ in 0..3 {
                frame(1.0 / 60.0, Some([yaw, 0.0]));
            }
        }
        pose()
    }

    #[test]
    fn strafing_leans_scales_and_inverts() {
        set_settings(quiet());
        tick(None, 0.0, false);
        // Strafing right while facing south: moving -X at sprint speed.
        let p = run(60, |t| [-(t as f64) * 0.28, 64.0, 0.0], 0.0);
        assert!((p.roll - 5.6 * STRAFE_ROLL).abs() < 0.01, "{}", p.roll);
        assert!(p.pitch.abs() < 0.01);
        let mut s = quiet();
        s.values[o::STRAFE] = 50;
        s.values[o::DIRECTION] = 1;
        set_settings(s);
        let p = run(60, |t| [-16.8 - (t as f64) * 0.28, 64.0, 0.0], 0.0);
        assert!((p.roll + 2.8 * STRAFE_ROLL).abs() < 0.01, "{}", p.roll);
        // Turned off: back to level.
        s.values[o::ENABLED] = 0;
        set_settings(s);
        let p = run(60, |t| [-33.6 - (t as f64) * 0.28, 64.0, 0.0], 0.0);
        assert!(p.roll.abs() < 0.01, "{}", p.roll);
    }

    #[test]
    fn landing_dips_and_comes_back() {
        set_settings(quiet());
        tick(None, 0.0, false);
        tick(Some([0.0, 80.0, 0.0]), 0.0, false);
        tick(Some([0.0, 79.5, 0.0]), 0.0, false);
        tick(Some([0.0, 79.5, 0.0]), 0.0, false);
        let mut lowest = 0.0f32;
        for _ in 0..20 {
            frame(1.0 / 60.0, Some([0.0, 0.0]));
            lowest = lowest.min(pose().rise);
        }
        assert!(lowest < -0.05, "{lowest}");
        for _ in 0..120 {
            frame(1.0 / 60.0, Some([0.0, 0.0]));
        }
        assert!(pose().rise.abs() < 0.01, "{}", pose().rise);
    }

    #[test]
    fn walking_bobs_and_standing_breathes() {
        let mut s = quiet();
        s.values[o::STEP_BOB] = 100;
        s.values[o::BREATHING] = 100;
        s.values[o::FORWARD] = 0;
        set_settings(s);
        tick(None, 0.0, false);
        // Walking south on flat ground: the camera goes down and up.
        let mut lowest = 0.0f32;
        for t in 0..60 {
            tick(Some([0.0, 64.0, t as f64 * 0.215]), 0.0, false);
            for _ in 0..3 {
                frame(1.0 / 60.0, Some([0.0, 0.0]));
                lowest = lowest.min(pose().rise);
            }
        }
        assert!(lowest < -0.02, "{lowest}");
        // Standing still: the bob stops, a slow breath remains.
        let mut pitches = Vec::new();
        for _ in 0..100 {
            tick(Some([0.0, 64.0, 12.9]), 0.0, false);
            for _ in 0..3 {
                frame(1.0 / 60.0, Some([0.0, 0.0]));
            }
            pitches.push(pose().pitch);
        }
        let (lo, hi) = pitches.iter().fold((f32::MAX, f32::MIN), |(l, h), &p| (l.min(p), h.max(p)));
        assert!(hi - lo > BREATH_PITCH, "{lo} {hi}");
    }

    #[test]
    fn speed_widens_the_fov() {
        let mut s = quiet();
        s.values[o::SPEED_FOV] = 100;
        set_settings(s);
        tick(None, 0.0, false);
        let p = run(60, |t| [0.0, 64.0, t as f64 * 0.6], 0.0);
        let expected = 1.0 + ((12.0 - WALK_SPEED) * FOV_PER_SPEED).min(MAX_FOV);
        assert!((p.fov - expected).abs() < 0.005, "{} {expected}", p.fov);
        set_settings(quiet());
        let p = run(60, |t| [0.0, 64.0, 36.0 + t as f64 * 0.6], 0.0);
        assert!((p.fov - 1.0).abs() < 0.005, "{}", p.fov);
    }

    #[test]
    fn turning_sways_and_wraps_at_180() {
        set_settings(quiet());
        tick(None, 0.0, false);
        tick(Some([0.0, 64.0, 0.0]), 0.0, false);
        // Turning right through the wrap at 180: +120 degrees a second.
        let mut yaw = 170.0f32;
        for _ in 0..60 {
            yaw += 2.0;
            let shown = if yaw > 180.0 { yaw - 360.0 } else { yaw };
            frame(1.0 / 60.0, Some([shown, 0.0]));
        }
        let p = pose();
        assert!((p.roll - 120.0 * TURN_ROLL).abs() < 0.01, "{}", p.roll);
        assert!(p.hand[0] < 0.0);
    }

    #[test]
    fn presets_are_valid_and_cycle() {
        for preset in 1..=4 {
            let values = preset_values(preset).unwrap();
            for (i, v) in PRESET_KEYS.iter().zip(values) {
                assert!(OPTS[*i].choices.iter().any(|c| c.0 == v), "preset {preset}: {}={v}", OPTS[*i].key);
            }
        }
        assert!(preset_values(0).is_none());
        let mut s = Settings::default();
        // Default, then Cinematic, PvP, Subtle and back to Default.
        let stored = s.click(o::PRESET);
        assert_eq!(s.get(o::PRESET), 3);
        assert_eq!(stored.len(), OPTS.len());
        assert_eq!(s.get(o::SMOOTHNESS), 60);
        s.click(o::PRESET);
        assert_eq!(s.labels()[o::PRESET], "Preset: PvP");
        assert_eq!((s.get(o::STEP_BOB), s.get(o::AIMING)), (0, 0));
        s.click(o::PRESET);
        assert_eq!(s.get(o::PRESET), 1);
        s.click(o::PRESET);
        assert_eq!(s, Settings::default(), "Default is the defaults");
        // From Custom, the next preset is Subtle.
        s.click(o::TURN);
        s.click(o::PRESET);
        assert_eq!(s.get(o::PRESET), 1);
        // Presets leave the rest alone.
        let mut s = Settings::default();
        s.values[o::DIRECTION] = 1;
        s.values[o::SWIMMING] = 0;
        s.apply_preset(4);
        assert_eq!((s.get(o::DIRECTION), s.get(o::SWIMMING)), (1, 0));
    }

    #[test]
    fn damage_shakes_and_settles() {
        set_settings(quiet());
        tick(None, 0.0, false);
        tick(Some([0.0, 64.0, 0.0]), 0.0, false);
        health(20.0, false);
        frame(1.0 / 60.0, Some([0.0, 0.0]));
        assert!(pose().roll.abs() < 1e-4, "no hit yet");
        // Lose 4 health: the camera jolts, both ways, then rings out.
        health(16.0, false);
        let mut peak = 0.0f32;
        let mut crossed = false;
        let mut last = 0.0f32;
        for _ in 0..30 {
            frame(1.0 / 60.0, Some([0.0, 0.0]));
            health(16.0, false);
            let r = pose().roll;
            peak = peak.max(r.abs());
            crossed |= last != 0.0 && r.signum() != last.signum();
            last = r;
        }
        assert!(peak > 1.0 && peak < 6.0, "{peak}");
        assert!(crossed, "it shakes, not just tilts");
        for _ in 0..60 {
            frame(1.0 / 60.0, Some([0.0, 0.0]));
            health(16.0, false);
        }
        assert!(pose().roll.abs() < 0.05, "{}", pose().roll);
        // Healing doesn't shake; damage shake OFF doesn't either.
        health(20.0, false);
        frame(1.0 / 60.0, Some([0.0, 0.0]));
        assert!(pose().roll.abs() < 0.05);
        let mut off = quiet();
        off.values[o::DAMAGE] = 0;
        set_settings(off);
        health(10.0, false);
        frame(1.0 / 60.0, Some([0.0, 0.0]));
        assert!(pose().roll.abs() < 0.05);
    }

    #[test]
    fn health_from_a_hidden_hud_is_forgotten() {
        set_settings(quiet());
        tick(None, 0.0, false);
        health(20.0, true);
        // F1 for a second: no word from the HUD.
        for _ in 0..60 {
            frame(1.0 / 60.0, Some([0.0, 0.0]));
        }
        // Back with less health (healing items, a respawn...): no shake from the gap.
        health(12.0, false);
        frame(1.0 / 60.0, Some([0.0, 0.0]));
        assert!(pose().roll.abs() < 1e-4);
    }

    #[test]
    fn aiming_and_riding_calm_the_lean() {
        let mut s = quiet();
        s.values[o::AIMING] = 25;
        s.values[o::RIDING] = 50;
        set_settings(s);
        tick(None, 0.0, false);
        let strafe = |start: f64| run(60, move |t| [start - (t as f64) * 0.28, 64.0, 0.0], 0.0).roll;
        let full = strafe(0.0);
        set_player(false, true);
        let aimed = strafe(-16.8);
        assert!((aimed - full * 0.25).abs() < 0.02, "{full} {aimed}");
        set_player(false, false);
        let mut riding = 0.0;
        for t in 0..60 {
            tick(Some([-33.6 - (t as f64) * 0.28, 64.0, 0.0]), 0.0, false);
            for _ in 0..3 {
                health(20.0, true);
                frame(1.0 / 60.0, Some([0.0, 0.0]));
            }
            riding = pose().roll;
        }
        assert!((riding - full * 0.5).abs() < 0.02, "{full} {riding}");
    }
}
