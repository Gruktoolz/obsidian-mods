// Fair Freecam for Horizon: press the key (F4 by default) to leave your body and
// fly the camera freely while your player stays where it is. Press it again
// to snap back. WASD moves, Space goes up, Shift down, Ctrl flies faster,
// and the mouse wheel changes the speed while flying. In fair play mode the
// camera can't pass through blocks, so it sees only what you could reach.
//
// This file is both the mod's Cargo library (its tests) and the module the
// bundle adds to the game (`crate::game::fair_freecam`). It knows nothing of the
// game's types: the hooks pass plain values in and take plain values out.
use std::cell::RefCell;
use std::time::Instant;

/// The mod's id: its settings folder and its key's line.
pub const ID: &str = "fair_freecam";
pub const VERSION: &str = "1.2.0";
pub const DESCRIPTION: &str =
    "Press a key to fly the camera freely while your player stays put, with an optional fair play mode.";
/// The key's name, and its default.
pub const KEY: &str = "toggle";
pub const DEFAULT_KEY: &str = "key.keyboard.f4";
/// Said in chat when the key is pressed on a server with the setting off.
pub const SERVER_OFF: &str =
    "Fair Freecam is off on servers. Turn it on in Options > Mods > Fair Freecam > Settings.";

/// The flying speeds the settings page cycles through, in blocks a second.
pub const SPEEDS: [u8; 6] = [2, 5, 10, 20, 40, 80];
const DEFAULT_SPEED: u8 = 10;
/// Ctrl multiplies the speed by this.
const FAST: f64 = 2.5;
/// Each wheel step multiplies the speed by this, within these limits.
const WHEEL_STEP: f32 = 1.2;
const BOOST_MIN: f32 = 0.1;
const BOOST_MAX: f32 = 10.0;
/// Half the free camera's size in fair play: it fits where the player fits
/// sideways and through one-block gaps, and stays far enough from a wall
/// that the view never clips into it.
pub const RADIUS: f64 = 0.3;
/// The longest step a fair play move takes before checking for blocks.
const FAIR_STEP: f64 = 0.25;
/// How quickly the camera reaches the speed it is asked for (1/s): a short
/// ease in and out instead of a jerk.
const RESPONSE: f64 = 10.0;
/// The longest frame a step accounts for, so a hitch doesn't fling the camera.
const MAX_DT: f64 = 0.1;

/// The settings, as kept in `horizon-mods/.horizon-mod-data/fair_freecam/`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// Blocks a second, one of `SPEEDS`.
    pub speed: u8,
    /// Fair play: the camera can't fly through blocks (no looking into
    /// caves or bases through walls).
    pub fair_play: bool,
    /// Whether the key works on servers too (it always works in your own
    /// worlds).
    pub servers: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            speed: DEFAULT_SPEED,
            fair_play: false,
            servers: true,
        }
    }
}

impl Settings {
    /// From the stored values (`get` reads one key's file); the defaults for
    /// the ones missing or unreadable.
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let speed = get("speed")
            .and_then(|v| v.trim().parse::<u8>().ok())
            .filter(|v| SPEEDS.contains(v))
            .unwrap_or(DEFAULT_SPEED);
        let fair_play = get("fair_play").is_some_and(|v| v.trim() == "true");
        let servers = get("servers").is_none_or(|v| v.trim() != "false");
        Self {
            speed,
            fair_play,
            servers,
        }
    }
    /// The settings page's buttons.
    pub fn labels(&self) -> Vec<String> {
        let on = |b: bool| if b { "ON" } else { "OFF" };
        vec![
            format!("Speed: {} blocks/s", self.speed),
            format!("Fair play (no flying through blocks): {}", on(self.fair_play)),
            format!("Allowed on servers: {}", on(self.servers)),
        ]
    }
    /// A click on button `i` of `labels`.
    pub fn click(&mut self, i: usize) {
        match i {
            0 => {
                let at = SPEEDS.iter().position(|&s| s == self.speed).unwrap_or(0);
                self.speed = SPEEDS[(at + 1) % SPEEDS.len()];
            }
            1 => self.fair_play = !self.fair_play,
            2 => self.servers = !self.servers,
            _ => {}
        }
    }
    /// What to store, key by key.
    pub fn stored(&self) -> [(&'static str, String); 3] {
        let flag = |b: bool| if b { "true" } else { "false" }.to_owned();
        [
            ("speed", self.speed.to_string()),
            ("fair_play", flag(self.fair_play)),
            ("servers", flag(self.servers)),
        ]
    }
}

/// The movement keys held this frame.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Input {
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub fast: bool,
}

/// The free camera while it flies.
#[derive(Clone, Copy, Debug)]
struct Cam {
    position: [f64; 3],
    velocity: [f64; 3],
    yaw: f32,
    pitch: f32,
    /// The wheel's speed factor.
    boost: f32,
    last: Option<Instant>,
}

#[derive(Default)]
struct State {
    /// Read on first use, then kept.
    settings: Option<Settings>,
    cam: Option<Cam>,
    /// The movement keys taken from the player for the current tick, one bit
    /// each, to give back after it.
    stash: u8,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

/// The settings, read once through `get`.
pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    STATE.with_borrow_mut(|s| *s.settings.get_or_insert_with(|| Settings::read(get)))
}

/// Replaces the settings (the settings page changed them).
pub fn set_settings(settings: Settings) {
    STATE.with_borrow_mut(|s| s.settings = Some(settings));
}

/// Whether the camera is flying free.
pub fn active() -> bool {
    STATE.with_borrow(|s| s.cam.is_some())
}

/// Leaves the body: the camera starts where the view is now.
pub fn start(position: [f64; 3], yaw: f32, pitch: f32) {
    STATE.with_borrow_mut(|s| {
        s.cam = Some(Cam {
            position,
            velocity: [0.0; 3],
            yaw,
            pitch: pitch.clamp(-90.0, 90.0),
            boost: 1.0,
            last: None,
        });
    });
}

/// Back to the player.
pub fn stop() {
    STATE.with_borrow_mut(|s| s.cam = None);
}

/// A mouse turn (`Entity.turn`'s deltas): true when the free camera took it,
/// so the player keeps looking where it was.
pub fn turn(xo: f64, yo: f64) -> bool {
    STATE.with_borrow_mut(|s| {
        let Some(c) = &mut s.cam else {
            return false;
        };
        c.yaw += (xo as f32) * 0.15;
        c.pitch = (c.pitch + (yo as f32) * 0.15).clamp(-90.0, 90.0);
        true
    })
}

/// Wheel steps: true when the free camera took them as a speed change.
pub fn wheel(steps: f32) -> bool {
    STATE.with_borrow_mut(|s| {
        let Some(c) = &mut s.cam else {
            return false;
        };
        c.boost = (c.boost * WHEEL_STEP.powf(steps)).clamp(BOOST_MIN, BOOST_MAX);
        true
    })
}

/// One frame of flight: moves the camera by the keys held since the last
/// frame and returns where to render from (position, yaw, pitch), or none
/// when the camera isn't free. In fair play, `free` says whether a camera
/// at a point (a box `RADIUS` each way) is clear of blocks.
pub fn step(
    input: Input,
    settings: Settings,
    mut free: impl FnMut([f64; 3]) -> bool,
) -> Option<([f64; 3], f32, f32)> {
    let now = Instant::now();
    STATE.with_borrow_mut(|s| {
        let c = s.cam.as_mut()?;
        let dt = c
            .last
            .map_or(0.0, |l| now.duration_since(l).as_secs_f64())
            .min(MAX_DT);
        c.last = Some(now);
        advance(c, input, settings, dt, &mut free);
        Some((c.position, c.yaw, c.pitch))
    })
}

/// The camera after `dt` seconds of `input`.
fn advance(
    c: &mut Cam,
    input: Input,
    settings: Settings,
    dt: f64,
    free: &mut impl FnMut([f64; 3]) -> bool,
) {
    let axis = |plus: bool, minus: bool| f64::from(i8::from(plus) - i8::from(minus));
    let (yaw, pitch) = (f64::from(c.yaw).to_radians(), f64::from(c.pitch).to_radians());
    // Minecraft's axes: yaw 0 looks south (+z), 90 west (-x); pitch 90 looks down.
    let forward = [-yaw.sin() * pitch.cos(), -pitch.sin(), yaw.cos() * pitch.cos()];
    let right = [-yaw.cos(), 0.0, -yaw.sin()];
    let (f, r, u) = (
        axis(input.forward, input.back),
        axis(input.right, input.left),
        axis(input.up, input.down),
    );
    let mut wish = [
        forward[0] * f + right[0] * r,
        forward[1] * f + u,
        forward[2] * f + right[2] * r,
    ];
    let length = wish.iter().map(|v| v * v).sum::<f64>().sqrt();
    if length > 1e-9 {
        wish = wish.map(|v| v / length);
    }
    let speed = f64::from(settings.speed)
        * f64::from(c.boost)
        * if input.fast { FAST } else { 1.0 };
    let ease = 1.0 - (-dt * RESPONSE).exp();
    for i in 0..3 {
        c.velocity[i] += (wish[i] * speed - c.velocity[i]) * ease;
    }
    let delta = c.velocity.map(|v| v * dt);
    if !settings.fair_play {
        for i in 0..3 {
            c.position[i] += delta[i];
        }
        return;
    }
    // Fair play: short steps, one axis at a time, so the camera stops at a
    // wall and slides along it. A camera already inside a block (a block
    // placed on it) may move, to get out.
    let longest = delta.iter().fold(0.0f64, |m, d| m.max(d.abs()));
    let steps = (longest / FAIR_STEP).ceil().max(1.0) as usize;
    let mut clear = free(c.position);
    for _ in 0..steps {
        for i in 0..3 {
            if delta[i] == 0.0 || c.velocity[i] == 0.0 {
                continue;
            }
            let mut next = c.position;
            next[i] += delta[i] / steps as f64;
            let next_clear = free(next);
            if next_clear || !clear {
                c.position = next;
                clear = next_clear;
            } else {
                c.velocity[i] = 0.0;
            }
        }
    }
}

/// Takes note of the movement keys taken from the player for this tick
/// (one bit each); kept until `take_stash` gives them back.
pub fn stash(mask: u8) {
    STATE.with_borrow_mut(|s| s.stash |= mask);
}

/// The movement keys to give back to the player, and forgets them.
pub fn take_stash() -> u8 {
    STATE.with_borrow_mut(|s| std::mem::take(&mut s.stash))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stored<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        |key| pairs.iter().find(|p| p.0 == key).map(|p| p.1.to_owned())
    }

    fn cam(yaw: f32, pitch: f32) -> Cam {
        Cam {
            position: [0.0; 3],
            velocity: [0.0; 3],
            yaw,
            pitch,
            boost: 1.0,
            last: None,
        }
    }

    /// Flies for `seconds` in small steps, through everything.
    fn fly(c: &mut Cam, input: Input, seconds: f64) {
        fly_in(c, input, seconds, Settings::default(), |_| true);
    }

    fn fly_in(
        c: &mut Cam,
        input: Input,
        seconds: f64,
        settings: Settings,
        mut free: impl FnMut([f64; 3]) -> bool,
    ) {
        for _ in 0..(seconds * 100.0) as usize {
            advance(c, input, settings, 0.01, &mut free);
        }
    }

    /// A stone wall filling x 5..6, and the camera's box against it.
    fn wall(at: [f64; 3]) -> bool {
        at[0] + RADIUS <= 5.0 || at[0] - RADIUS >= 6.0
    }

    #[test]
    fn settings_read_click_and_store() {
        assert_eq!(Settings::read(stored(&[])), Settings::default());
        let s = Settings::read(stored(&[
            ("speed", "40\n"),
            ("fair_play", "true"),
            ("servers", "false"),
        ]));
        assert_eq!(
            s,
            Settings {
                speed: 40,
                fair_play: true,
                servers: false
            }
        );
        assert_eq!(Settings::read(stored(&[("speed", "7")])).speed, DEFAULT_SPEED);
        let mut t = s;
        t.click(0);
        assert_eq!(t.speed, 80);
        t.click(0);
        assert_eq!(t.speed, 2);
        t.click(1);
        t.click(2);
        assert_eq!(
            t.labels(),
            [
                "Speed: 2 blocks/s",
                "Fair play (no flying through blocks): OFF",
                "Allowed on servers: ON"
            ]
        );
        let round: Vec<(&str, String)> = t.stored().into();
        let back: Vec<(&str, &str)> = round.iter().map(|(k, v)| (*k, v.as_str())).collect();
        assert_eq!(Settings::read(stored(&back)), t);
    }

    #[test]
    fn flies_where_it_looks() {
        // Yaw 0 looks south (+z).
        let mut c = cam(0.0, 0.0);
        fly(&mut c, Input { forward: true, ..Input::default() }, 2.0);
        assert!(c.position[2] > 15.0, "{:?}", c.position);
        assert!(c.position[0].abs() < 1e-6 && c.position[1].abs() < 1e-6);
        // Facing south, right is west (-x).
        let mut c = cam(0.0, 0.0);
        fly(&mut c, Input { right: true, ..Input::default() }, 1.0);
        assert!(c.position[0] < -5.0, "{:?}", c.position);
        // Looking straight down, forward goes down.
        let mut c = cam(0.0, 90.0);
        fly(&mut c, Input { forward: true, ..Input::default() }, 1.0);
        assert!(c.position[1] < -5.0, "{:?}", c.position);
        // Space up, shift down, regardless of the view.
        let mut c = cam(45.0, -30.0);
        fly(&mut c, Input { up: true, ..Input::default() }, 1.0);
        assert!(c.position[1] > 5.0 && c.position[0].abs() < 1e-6);
    }

    #[test]
    fn eases_in_and_stops() {
        let mut c = cam(0.0, 0.0);
        advance(
            &mut c,
            Input { forward: true, ..Input::default() },
            Settings::default(),
            0.01,
            &mut |_| true,
        );
        assert!(c.velocity[2] > 0.0 && c.velocity[2] < 2.0, "eases in");
        fly(&mut c, Input { forward: true, ..Input::default() }, 2.0);
        assert!((c.velocity[2] - 10.0).abs() < 0.1, "reaches the speed: {:?}", c.velocity);
        fly(&mut c, Input::default(), 2.0);
        assert!(c.velocity[2].abs() < 0.1, "glides to a stop");
        // Diagonals are no faster.
        let mut d = cam(0.0, 0.0);
        fly(&mut d, Input { forward: true, right: true, ..Input::default() }, 2.0);
        let v = d.velocity.iter().map(|v| v * v).sum::<f64>().sqrt();
        assert!((v - 10.0).abs() < 0.1, "{v}");
        // Ctrl is faster.
        let mut e = cam(0.0, 0.0);
        fly(&mut e, Input { forward: true, fast: true, ..Input::default() }, 2.0);
        assert!((e.velocity[2] - 25.0).abs() < 0.3);
    }

    #[test]
    fn fair_play_stops_at_walls_and_slides() {
        let fair = Settings {
            fair_play: true,
            ..Settings::default()
        };
        // Yaw -90 looks east (+x), at the wall.
        let east = Input { forward: true, ..Input::default() };
        let mut c = cam(-90.0, 0.0);
        fly_in(&mut c, east, 3.0, fair, wall);
        assert!(c.position[0] <= 5.0 - RADIUS + 1e-9, "stops before the wall: {:?}", c.position);
        assert!(c.position[0] > 5.0 - RADIUS - FAIR_STEP, "close to it: {:?}", c.position);
        // Fast as it may fly, it never tunnels through.
        let mut f = cam(-90.0, 0.0);
        f.boost = BOOST_MAX;
        fly_in(&mut f, Input { fast: true, ..east }, 3.0, fair, wall);
        assert!(f.position[0] < 5.0, "{:?}", f.position);
        // Along the wall it still slides.
        let mut s = cam(-45.0, 0.0);
        fly_in(&mut s, east, 3.0, fair, wall);
        assert!(s.position[0] < 5.0 && s.position[2] > 10.0, "{:?}", s.position);
        // Off, it flies through.
        let mut o = cam(-90.0, 0.0);
        fly_in(&mut o, east, 3.0, Settings::default(), wall);
        assert!(o.position[0] > 6.0, "{:?}", o.position);
        // A camera stuck in a block may leave it.
        let mut stuck = cam(90.0, 0.0);
        stuck.position = [5.5, 0.0, 0.0];
        fly_in(&mut stuck, Input { forward: true, ..Input::default() }, 1.0, fair, wall);
        assert!(stuck.position[0] < 5.0, "{:?}", stuck.position);
    }

    #[test]
    fn starts_turns_wheels_and_stops() {
        stop();
        assert!(!active());
        assert!(!turn(10.0, 10.0), "nothing to turn while off");
        assert!(!wheel(1.0));
        assert_eq!(step(Input::default(), Settings::default(), |_| true), None);
        start([1.0, 70.0, 2.0], 90.0, 120.0);
        assert!(active());
        let (p, yaw, pitch) = step(Input::default(), Settings::default(), |_| true).unwrap();
        assert_eq!((p, yaw, pitch), ([1.0, 70.0, 2.0], 90.0, 90.0), "pitch is clamped");
        assert!(turn(100.0, -100.0));
        let (_, yaw, pitch) = step(Input::default(), Settings::default(), |_| true).unwrap();
        assert_eq!((yaw, pitch), (105.0, 75.0));
        assert!(wheel(100.0));
        STATE.with_borrow(|s| assert_eq!(s.cam.unwrap().boost, BOOST_MAX));
        stop();
        assert!(!active());
    }

    #[test]
    fn stash_gathers_until_taken() {
        take_stash();
        stash(0b0001);
        stash(0b0100);
        assert_eq!(take_stash(), 0b0101);
        assert_eq!(take_stash(), 0);
    }
}
