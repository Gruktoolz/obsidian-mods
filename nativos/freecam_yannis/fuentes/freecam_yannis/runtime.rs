// FreeCam by Yannis_: native camera state and geometry. No game objects retained.
use std::cell::RefCell;
pub const ID: &str = "freecam_yannis";
pub const VERSION: &str = "1.0.1";
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pose {
    pub position: [f64; 3],
    pub yaw: f32,
    pub pitch: f32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    Player,
    Free,
    Follow,
}
#[derive(Clone, Copy)]
pub struct Settings {
    pub speed: f64,
    pub distance: f64,
    pub height: f64,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            speed: 10.0,
            distance: 28.0,
            height: 18.0,
        }
    }
}
impl Settings {
    pub fn load(read: impl Fn(&str) -> Option<String>) -> Self {
        fn number(v: Option<String>, default: f64, min: f64, max: f64) -> f64 {
            v.and_then(|s| s.parse::<f64>().ok())
                .filter(|n| n.is_finite())
                .unwrap_or(default)
                .clamp(min, max)
        }
        Self {
            speed: number(read("speed"), 10.0, 1.0, 80.0),
            distance: number(read("distance"), 28.0, 2.0, 80.0),
            height: number(read("height"), 18.0, 1.0, 80.0),
        }
    }
}
pub struct State {
    pub mode: Mode,
    pub pose: Option<Pose>,
    pub settings: Option<Settings>,
    pub slots: [Option<Pose>; 9],
    pub free_key: bool,
    pub chord: bool,
    previous_free: Option<Pose>,
    dimension: Option<&'static str>,
}
impl Default for State {
    fn default() -> Self {
        Self {
            mode: Mode::Player,
            pose: None,
            settings: None,
            slots: [None; 9],
            free_key: false,
            chord: false,
            previous_free: None,
            dimension: None,
        }
    }
}
impl State {
    pub fn reset(&mut self) {
        let settings = self.settings;
        *self = Self::default();
        self.settings = settings;
    }
    pub fn world(&mut self, dimension: Option<&'static str>) {
        if self.dimension != dimension {
            self.reset();
            self.dimension = dimension;
        }
    }
    pub fn toggle_free(&mut self, initial: Pose) {
        if self.mode == Mode::Free {
            self.mode = Mode::Player;
            self.pose = None;
        } else {
            self.pose = Some(self.pose.unwrap_or(initial));
            self.mode = Mode::Free;
        }
        self.previous_free = None;
    }
    pub fn toggle_follow(&mut self, initial: Pose) {
        if self.mode == Mode::Follow {
            self.pose = self.previous_free.take();
            self.mode = if self.pose.is_some() {
                Mode::Free
            } else {
                Mode::Player
            };
        } else {
            self.previous_free = if self.mode == Mode::Free {
                self.pose
            } else {
                None
            };
            self.pose = Some(follow(initial, self.settings.unwrap_or_default()));
            self.mode = Mode::Follow;
        }
    }
    pub fn tripod(&mut self, slot: usize, initial: Pose) {
        if let Some(saved) = self.slots.get_mut(slot) {
            let pose = *saved.get_or_insert(self.pose.unwrap_or(initial));
            self.pose = Some(pose);
            self.mode = Mode::Free;
            self.previous_free = None;
            self.chord = true;
        }
    }
    pub fn turn(&mut self, x: f64, y: f64) {
        if self.mode != Mode::Free || !x.is_finite() || !y.is_finite() {
            return;
        }
        if let Some(p) = &mut self.pose {
            p.yaw = (p.yaw as f64 + x * 0.15).rem_euclid(360.0) as f32;
            p.pitch = (p.pitch as f64 + y * 0.15).clamp(-90.0, 90.0) as f32;
        }
    }
    pub fn advance(&mut self, player: Pose, seconds: f64, input: [f64; 3], sprint: bool) {
        let dt = if seconds.is_finite() {
            seconds.clamp(0.0, 0.1)
        } else {
            0.0
        };
        let settings = self.settings.unwrap_or_default();
        match self.mode {
            Mode::Player => {}
            Mode::Follow => {
                let target = follow(player, settings);
                let p = self.pose.get_or_insert(target);
                let alpha = 1.0 - (-18.0 * dt).exp();
                for (a, b) in p.position.iter_mut().zip(target.position) {
                    *a += (b - *a) * alpha;
                }
                // Always look at the player, even while the offset is easing.
                let d = std::array::from_fn::<_, 3, _>(|i| player.position[i] - p.position[i]);
                p.yaw = (-d[0]).atan2(d[2]).to_degrees() as f32;
                p.pitch = (-d[1]).atan2(d[0].hypot(d[2])).to_degrees() as f32;
            }
            Mode::Free => {
                if let Some(p) = &mut self.pose {
                    let [forward, side, up] = input;
                    let len = forward.hypot(side).hypot(up).max(1.0);
                    let speed = settings.speed * if sprint { 3.0 } else { 1.0 } * dt / len;
                    let yaw = (p.yaw as f64).to_radians();
                    p.position[0] += (-yaw.sin() * forward + yaw.cos() * side) * speed;
                    p.position[1] += up * speed;
                    p.position[2] += (yaw.cos() * forward + yaw.sin() * side) * speed;
                }
            }
        }
    }
}
pub fn follow(player: Pose, settings: Settings) -> Pose {
    let y = (player.yaw as f64).to_radians();
    Pose {
        position: [
            player.position[0] + y.sin() * settings.distance,
            player.position[1] + settings.height,
            player.position[2] - y.cos() * settings.distance,
        ],
        yaw: player.yaw,
        pitch: settings.height.atan2(settings.distance).to_degrees() as f32,
    }
}
thread_local! { pub static STATE: RefCell<State> = RefCell::new(State::default()); }
pub fn active() -> bool {
    STATE.with(|s| s.borrow().mode != Mode::Player)
}
pub fn detached() -> bool {
    STATE.with(|s| s.borrow().mode == Mode::Free)
}
#[cfg(test)]
mod tests {
    use super::*;
    fn pose() -> Pose {
        Pose {
            position: [-20.0, 64.0, -30.0],
            yaw: 0.0,
            pitch: 0.0,
        }
    }
    #[test]
    fn repeated_f6_restores_free_camera() {
        let mut s = State::default();
        s.toggle_free(pose());
        for _ in 0..1000 {
            s.toggle_follow(pose());
            s.toggle_follow(pose());
            assert_eq!(s.pose, Some(pose()));
            assert_eq!(s.mode, Mode::Free);
        }
    }
    #[test]
    fn follow_returns_to_player() {
        let mut s = State::default();
        s.toggle_follow(pose());
        s.toggle_follow(pose());
        assert_eq!(s.mode, Mode::Player);
        assert!(s.pose.is_none());
    }
    #[test]
    fn worlds_clear_all_cameras() {
        let mut s = State::default();
        s.world(Some("overworld"));
        s.tripod(0, pose());
        s.toggle_follow(pose());
        s.world(Some("nether"));
        assert_eq!(s.mode, Mode::Player);
        assert!(s.slots.iter().all(Option::is_none));
    }
    #[test]
    fn bad_settings_are_bounded() {
        let s = Settings::load(|k| {
            Some(
                if k == "speed" {
                    "NaN"
                } else if k == "height" {
                    "-5"
                } else {
                    "99999"
                }
                .into(),
            )
        });
        assert_eq!((s.speed, s.distance, s.height), (10.0, 80.0, 1.0));
    }
    #[test]
    fn diagonal_speed_and_stalls_are_bounded() {
        let mut s = State::default();
        s.toggle_free(pose());
        s.advance(pose(), 100.0, [1.0, 1.0, 1.0], false);
        let p = s.pose.unwrap();
        let d = p
            .position
            .iter()
            .zip(pose().position)
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt();
        assert!((d - 1.0).abs() < 1e-10);
    }
    #[test]
    fn follow_geometry_looks_at_player() {
        let p = follow(pose(), Settings::default());
        assert_eq!(p.position, [-20.0, 82.0, -58.0]);
        assert!((p.pitch - 32.735226).abs() < 0.001);
    }
    #[test]
    fn tripod_restores_saved_pose() {
        let mut s = State::default();
        s.tripod(0, pose());
        s.turn(300.0, 200.0);
        s.tripod(0, pose());
        assert_eq!(s.pose, Some(pose()));
    }
    #[test]
    fn follow_does_not_consume_look() {
        let mut s = State::default();
        s.toggle_follow(pose());
        let before = s.pose;
        s.turn(100.0, 100.0);
        assert_eq!(s.pose, before);
    }
}
