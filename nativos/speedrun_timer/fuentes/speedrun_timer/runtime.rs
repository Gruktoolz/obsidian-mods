// Speedrun Timer, a native mod for Horizon: a real-time-attack timer in a
// corner of the screen. It arms when the player is in a world, starts on the
// first movement after that, and stops when the player leaves the End for the
// overworld, which is how the run ends once the dragon is dead. The time is
// HH:MM:SS.mmm with unlimited hours, and every finished run is appended to
// `runs.csv` in the mod's own folder. This file is both the mod's Cargo
// library (its tests run with `cargo test`, on no game at all) and the module
// the bundle adds to the game as `crate::game::speedrun_timer`.

use std::path::Path;
use std::time::Instant;

/// The mod's id (its settings folder), name and version.
pub const ID: &str = "speedrun_timer";
pub const NAME: &str = "Speedrun Timer";
pub const VERSION: &str = "1.0.1";
pub const DESCRIPTION: &str = "A real-time-attack timer in a corner: HH:MM:SS.mmm, starts when you first move, stops when you leave the End. Saves every run to its folder.";

/// The panel: translucent black, gold text while it runs, green when it
/// stops, grey before it starts.
pub const BACKGROUND: u32 = 0x7800_0000;
pub const GOLD: u32 = 0xFF_FF55;
pub const GREEN: u32 = 0x55_FF55;
pub const DIM: u32 = 0xAA_AAAA;
/// The height of the box, as the other HUD boxes use it.
pub const ROW_H: f32 = 12.0;

/// The corners, as the settings files name them and the page labels them.
pub const CORNERS: [(&str, &str); 4] = [
    ("top_left", "Top left"),
    ("top_right", "Top right"),
    ("bottom_left", "Bottom left"),
    ("bottom_right", "Bottom right"),
];

/// The End and the overworld, as `NativePlayer::dimension` names them.
pub const THE_END: &str = "minecraft:the_end";
pub const OVERWORLD: &str = "minecraft:overworld";

/// How far the player has to move (in blocks, on any axis) for the run to
/// start: enough that standing still never starts it.
pub const START_EPSILON: f64 = 0.05;

/// How many frames out of a world (about a second) before the next world
/// counts as a new session and the timer starts again from zero. A dimension
/// change or a death is far shorter than this, so neither resets it.
pub const NEW_WORLD_FRAMES: u32 = 60;

/// Where the player is, as the frame hook reports it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub dimension: &'static str,
}

/// The settings, as one file per key in the mod's folder.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// Index in `CORNERS`.
    pub corner: usize,
    /// Show the box.
    pub shown: bool,
    /// Show the personal best beside the running time.
    pub best: bool,
    /// The personal best, in milliseconds (0: none yet).
    pub best_ms: u64,
}

impl Settings {
    /// From the stored values; defaults (top left, shown, best on, no PB)
    /// for the ones missing.
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let corner = get("corner")
            .and_then(|c| CORNERS.iter().position(|k| k.0 == c.trim()))
            .unwrap_or(0);
        let flag = |key: &str, default: bool| match get(key) {
            Some(v) => v.trim() != "false",
            None => default,
        };
        let best_ms = get("best_ms")
            .and_then(|v| v.trim().parse().ok())
            .unwrap_or(0);
        Self {
            corner,
            shown: flag("shown", true),
            best: flag("best", true),
            best_ms,
        }
    }

    /// The settings page's buttons, in order.
    pub fn labels(&self) -> Vec<String> {
        let on = |b: bool| if b { "ON" } else { "OFF" };
        vec![
            format!("Corner: {}", CORNERS[self.corner].1),
            format!("Timer shown: {}", on(self.shown)),
            format!("Personal best: {}", on(self.best)),
            "Open timer folder".to_owned(),
            "Reset timer".to_owned(),
        ]
    }

    /// A click on buttons 0-2 (3 and 4 are the folder and the reset, which
    /// need the game: see `click.rs`).
    pub fn click(&mut self, i: usize) {
        match i {
            0 => self.corner = (self.corner + 1) % CORNERS.len(),
            1 => self.shown = !self.shown,
            2 => self.best = !self.best,
            _ => {}
        }
    }

    /// The settings as the files to write.
    pub fn stored(&self) -> [(&'static str, String); 4] {
        let on = |b: bool| (if b { "true" } else { "false" }).to_owned();
        [
            ("corner", CORNERS[self.corner].0.to_owned()),
            ("shown", on(self.shown)),
            ("best", on(self.best)),
            ("best_ms", self.best_ms.to_string()),
        ]
    }
}

thread_local! {
    /// The settings, read once from the files, then kept.
    static SETTINGS: std::cell::Cell<Option<Settings>> = const { std::cell::Cell::new(None) };
    /// The run: phase, where it was armed, when it started.
    static RUN: std::cell::RefCell<Option<Run>> = const { std::cell::RefCell::new(None) };
}

/// The settings, read once through `get` (one file per key).
pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    SETTINGS.with(|s| {
        let found = s.get().unwrap_or_else(|| Settings::read(get));
        s.set(Some(found));
        found
    })
}

/// Replaces the settings (the settings page changed them).
pub fn set_settings(settings: Settings) {
    SETTINGS.with(|s| s.set(Some(settings)));
}

/// Where the run is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    /// No world yet, or the run was reset.
    Idle,
    /// In a world, waiting for the first movement.
    Armed,
    /// Counting.
    Running,
    /// Stopped; the time is final.
    Done,
}

/// What a frame did, for the frame hook to act on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    /// Nothing to do.
    Nothing,
    /// The run just started.
    Started,
    /// The run just stopped: its final time in nanoseconds.
    Finished(u64),
}

#[derive(Clone, Copy, Debug)]
struct Run {
    phase: Phase,
    armed_at: Option<Pos>,
    started: Option<Instant>,
    in_end: bool,
    last: u64,
    /// A screen came over the world (the death screen, a menu) while in the
    /// End: what the next frame shows decides death from a real finish.
    suspended: bool,
    /// Frames spent out of a world.
    away_frames: u32,
}

impl Run {
    fn new() -> Self {
        Self {
            phase: Phase::Idle,
            armed_at: None,
            started: None,
            in_end: false,
            last: 0,
            suspended: false,
            away_frames: 0,
        }
    }
}

/// `1h 02m 03.456s`-style math, as `HH:MM:SS.mmm`: the hours are padded to
/// two digits and then grow as far as they need.
pub fn format(nanos: u64) -> String {
    let ms = nanos / 1_000_000;
    let (h, m, s, milli) = (
        ms / 3_600_000,
        ms / 60_000 % 60,
        ms / 1_000 % 60,
        ms % 1_000,
    );
    format!("{h:02}:{m:02}:{s:02}.{milli:03}")
}

/// Whether the player left the spot the run was armed at.
pub fn moved(from: Pos, to: Pos) -> bool {
    (to.x - from.x).abs() > START_EPSILON
        || (to.y - from.y).abs() > START_EPSILON
        || (to.z - from.z).abs() > START_EPSILON
}

/// The time a run shows at `now`: live while it runs, final once done.
fn live(run: &Run, now: Instant) -> u64 {
    match (run.phase, run.started) {
        (Phase::Running, Some(start)) => {
            let d = now.saturating_duration_since(start);
            d.as_nanos().min(u64::MAX as u128) as u64
        }
        (Phase::Done, _) => run.last,
        _ => 0,
    }
}

/// One frame: arm in a world, start on the first movement, finish when the
/// End is left for the overworld by the exit portal. Death is not a finish:
/// the death screen suspends the reading, and a respawn in the overworld
/// leaves the run counting.
pub fn step_playing(pos: Option<Pos>, playing: bool, now: Instant) -> Event {
    RUN.with(|cell| {
        let mut slot = cell.borrow_mut();
        let run = slot.get_or_insert_with(Run::new);
        let Some(p) = pos else {
            // Out of a world: a short gap (a dimension change, a death) keeps
            // the run; a long one (the title screen) is a new session.
            run.away_frames = run.away_frames.saturating_add(1);
            if run.away_frames >= NEW_WORLD_FRAMES {
                *run = Run::new();
            }
            return Event::Nothing;
        };
        run.away_frames = 0;
        match run.phase {
            Phase::Idle => {
                // A world: armed at zero, waiting for the first movement, even
                // when the last one had finished.
                run.phase = Phase::Armed;
                run.armed_at = Some(p);
                run.in_end = p.dimension == THE_END;
                run.suspended = false;
                run.started = None;
                run.last = 0;
                Event::Nothing
            }
            Phase::Armed => match run.armed_at {
                Some(from) if moved(from, p) => {
                    run.phase = Phase::Running;
                    run.started = Some(now);
                    Event::Started
                }
                _ => Event::Nothing,
            },
            Phase::Running => {
                if !playing {
                    // The death screen or a menu is up: do not read where the
                    // player is until it is gone.
                    run.suspended = true;
                    return Event::Nothing;
                }
                if run.suspended {
                    run.suspended = false;
                    if run.in_end && p.dimension != THE_END {
                        // Died in the End and came back: not a finish.
                        run.in_end = false;
                        return Event::Nothing;
                    }
                }
                if p.dimension == THE_END {
                    run.in_end = true;
                } else if run.in_end && p.dimension == OVERWORLD {
                    run.last = live(run, now);
                    run.phase = Phase::Done;
                    return Event::Finished(run.last);
                }
                Event::Nothing
            }
            Phase::Done => Event::Nothing,
        }
    })
}

/// The phase and the time to show, for the HUD and `/speedrun`.
pub fn reading(now: Instant) -> (Phase, u64) {
    RUN.with(|cell| match cell.borrow().as_ref() {
        Some(run) => (run.phase, live(run, now)),
        None => (Phase::Idle, 0),
    })
}

/// Starts the run now, whatever the player is doing.
pub fn force_start(now: Instant) {
    RUN.with(|cell| {
        let mut slot = cell.borrow_mut();
        let run = slot.get_or_insert_with(Run::new);
        run.phase = Phase::Running;
        run.started = Some(now);
    });
}

/// Stops a running run and gives its final time.
pub fn force_stop(now: Instant) -> Option<u64> {
    RUN.with(|cell| {
        let mut slot = cell.borrow_mut();
        let run = slot.as_mut()?;
        if run.phase != Phase::Running {
            return None;
        }
        run.last = live(run, now);
        run.phase = Phase::Done;
        Some(run.last)
    })
}

/// Forgets the run: back to waiting for a world.
pub fn reset() {
    RUN.with(|cell| *cell.borrow_mut() = None);
}

/// The run's folder: `horizon-mods/.horizon-mod-data/speedrun_timer/`.
pub fn runs_file(dir: &Path) -> std::path::PathBuf {
    dir.join("runs.csv")
}

/// Appends a finished run to `runs.csv` (with a header the first time) and
/// writes `last_run.txt`.
pub fn save_run(dir: &Path, nanos: u64) -> std::io::Result<std::path::PathBuf> {
    std::fs::create_dir_all(dir)?;
    let unix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let path = runs_file(dir);
    let mut log = std::fs::read_to_string(&path).unwrap_or_default();
    if log.is_empty() {
        log.push_str("finished_unix,time_ms,time\n");
    }
    log.push_str(&format!("{unix},{},{}\n", nanos / 1_000_000, format(nanos)));
    std::fs::write(&path, log)?;
    std::fs::write(dir.join("last_run.txt"), format!("{}\n", format(nanos)))?;
    Ok(path)
}

/// Opens the folder in the desktop's file manager, trying each platform's
/// opener until one starts.
pub fn open_folder(dir: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;
    let mut last = None;
    for opener in ["xdg-open", "open", "explorer"] {
        match std::process::Command::new(opener).arg(dir).spawn() {
            Ok(_) => return Ok(()),
            Err(e) => last = Some(e),
        }
    }
    Err(last.unwrap_or_else(|| std::io::Error::other("no folder opener")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    /// Every test frame is on the screen unless the test says otherwise.
    fn step(pos: Option<Pos>, now: Instant) -> Event {
        step_playing(pos, true, now)
    }

    fn at(x: f64, y: f64, z: f64) -> Pos {
        Pos {
            x,
            y,
            z,
            dimension: OVERWORLD,
        }
    }

    #[test]
    fn formats_hours_minutes_seconds_and_milliseconds() {
        assert_eq!(format(0), "00:00:00.000");
        // Whole milliseconds, truncated: 0.999999 ms is still 0 ms.
        assert_eq!(format(999_999), "00:00:00.000");
        assert_eq!(format(999_000_000), "00:00:00.999");
        assert_eq!(format(1_000_000_000), "00:00:01.000");
        // 1h 02m 03.456s
        assert_eq!(format(3_723_456_000_000), "01:02:03.456");
        // Hours go on for ever: 100h 00m 00.000s, 1000h, and 10 000h.
        assert_eq!(format(360_000_000_000_000), "100:00:00.000");
        assert_eq!(format(3_600_000_000_000_000), "1000:00:00.000");
        assert_eq!(format(36_000_000_000_000_000), "10000:00:00.000");
    }

    #[test]
    fn movement_starts_and_standing_still_does_not() {
        assert!(!moved(at(0.0, 64.0, 0.0), at(0.0, 64.0, 0.0)));
        assert!(!moved(at(0.0, 64.0, 0.0), at(0.01, 64.0, 0.0)));
        assert!(moved(at(0.0, 64.0, 0.0), at(0.2, 64.0, 0.0)));
        assert!(moved(at(0.0, 64.0, 0.0), at(0.0, 63.0, 0.0)));
    }

    #[test]
    fn arms_in_a_world_starts_on_movement_and_finishes_leaving_the_end() {
        reset();
        let now = Instant::now();
        // No world: nothing.
        assert_eq!(step(None, now), Event::Nothing);
        assert_eq!(reading(now).0, Phase::Idle);
        // A world: armed, not counting, and it shows zero.
        assert_eq!(step(Some(at(0.0, 64.0, 0.0)), now), Event::Nothing);
        assert_eq!(reading(now), (Phase::Armed, 0));
        // Still standing: no start.
        assert_eq!(step(Some(at(0.0, 64.0, 0.0)), now), Event::Nothing);
        assert_eq!(reading(now).0, Phase::Armed);
        // Movement: the run starts.
        assert_eq!(step(Some(at(0.5, 64.0, 0.0)), now), Event::Started);
        assert_eq!(reading(now).0, Phase::Running);
        // Walking around in the overworld does not finish it.
        assert_eq!(step(Some(at(40.0, 64.0, 9.0)), now + Duration::from_secs(5)), Event::Nothing);
        // The End arms the finish line...
        let mut end = at(0.0, 64.0, 0.0);
        end.dimension = THE_END;
        assert_eq!(step(Some(end), now + Duration::from_secs(10)), Event::Nothing);
        assert_eq!(reading(now + Duration::from_secs(10)).0, Phase::Running);
        // ...and the overworld crosses it.
        match step(Some(at(0.0, 64.0, 0.0)), now + Duration::from_secs(12)) {
            Event::Finished(n) => assert_eq!(n, 12_000_000_000),
            other => panic!("expected a finish, got {other:?}"),
        }
        assert_eq!(reading(now + Duration::from_secs(99)).0, Phase::Done);
        // A finished run does not move again.
        assert_eq!(step(Some(at(1.0, 64.0, 1.0)), now + Duration::from_secs(100)), Event::Nothing);
        reset();
    }

    #[test]
    fn leaving_a_world_before_moving_waits_for_the_next_one() {
        reset();
        let now = Instant::now();
        step(Some(at(0.0, 64.0, 0.0)), now);
        // A single frame out of the world (a dimension change, a death) keeps
        // the run armed where it was.
        assert_eq!(step(None, now), Event::Nothing);
        assert_eq!(reading(now).0, Phase::Armed);
        // A whole second out of it is a new session.
        for _ in 0..NEW_WORLD_FRAMES {
            step(None, now);
        }
        assert_eq!(reading(now).0, Phase::Idle);
        reset();
    }

    #[test]
    fn force_start_and_stop_work_from_the_command() {
        reset();
        let now = Instant::now();
        force_start(now);
        assert_eq!(reading(now).0, Phase::Running);
        assert_eq!(
            force_stop(now + Duration::from_millis(1500)),
            Some(1_500_000_000)
        );
        assert_eq!(force_stop(now), None);
        reset();
    }

    #[test]
    fn dying_in_the_end_is_not_a_finish() {
        reset();
        let now = Instant::now();
        step(Some(at(0.0, 64.0, 0.0)), now);
        assert_eq!(step(Some(at(1.0, 64.0, 0.0)), now), Event::Started);
        let mut end = at(0.0, 64.0, 0.0);
        end.dimension = THE_END;
        step(Some(end), now + Duration::from_secs(10));
        // The death screen comes up.
        assert_eq!(
            step_playing(Some(end), false, now + Duration::from_secs(11)),
            Event::Nothing
        );
        // The respawn puts the player in the overworld: the run goes on.
        assert_eq!(
            step_playing(Some(at(0.5, 64.0, 0.5)), true, now + Duration::from_secs(12)),
            Event::Nothing
        );
        assert_eq!(reading(now + Duration::from_secs(12)).0, Phase::Running);
        assert_eq!(reading(now + Duration::from_secs(20)).1, 20_000_000_000);
        reset();
    }

    #[test]
    fn a_menu_in_the_end_keeps_the_finish_working() {
        reset();
        let now = Instant::now();
        step(Some(at(0.0, 64.0, 0.0)), now);
        step(Some(at(1.0, 64.0, 0.0)), now);
        let mut end = at(0.0, 64.0, 0.0);
        end.dimension = THE_END;
        step(Some(end), now + Duration::from_secs(10));
        // Paused in the End, then back to it: still in the End, so the exit
        // portal still finishes the run.
        step_playing(Some(end), false, now + Duration::from_secs(11));
        step_playing(Some(end), true, now + Duration::from_secs(12));
        match step_playing(Some(at(0.0, 64.0, 0.0)), true, now + Duration::from_secs(14)) {
            Event::Finished(n) => assert_eq!(n, 14_000_000_000),
            other => panic!("expected a finish, got {other:?}"),
        }
        reset();
    }

    #[test]
    fn a_new_world_starts_again_from_zero() {
        reset();
        let now = Instant::now();
        step(Some(at(0.0, 64.0, 0.0)), now);
        step(Some(at(1.0, 64.0, 0.0)), now);
        assert_eq!(reading(now + Duration::from_secs(5)).0, Phase::Running);
        // Back to the title long enough to be a new session.
        for _ in 0..NEW_WORLD_FRAMES {
            step(None, now);
        }
        assert_eq!(reading(now).0, Phase::Idle);
        // The new world: armed at zero, waiting for the first movement.
        assert_eq!(step(Some(at(100.0, 70.0, 100.0)), now), Event::Nothing);
        assert_eq!(reading(now), (Phase::Armed, 0));
        // And a finished run makes way for the next one the same way.
        reset();
    }

    #[test]
    fn settings_default_cycle_and_store() {
        let stored: [(&str, &str); 0] = [];
        let get = |key: &str| {
            stored
                .iter()
                .find(|p| p.0 == key)
                .map(|p| p.1.to_owned())
        };
        let mut s = Settings::read(get);
        assert_eq!(
            s,
            Settings {
                corner: 0,
                shown: true,
                best: true,
                best_ms: 0
            }
        );
        assert_eq!(s.labels()[0], "Corner: Top left");
        assert_eq!(s.labels()[3], "Open timer folder");
        assert_eq!(s.labels()[4], "Reset timer");
        // Three clicks take the corner all the way round.
        s.click(0);
        s.click(0);
        s.click(0);
        assert_eq!(s.labels()[0], "Corner: Bottom right");
        s.click(0);
        assert_eq!(s.labels()[0], "Corner: Top left");
        s.click(1);
        s.click(2);
        assert_eq!(s.labels()[1], "Timer shown: OFF");
        assert_eq!(s.labels()[2], "Personal best: OFF");
        assert_eq!(
            s.stored(),
            [
                ("corner", "top_left".to_owned()),
                ("shown", "false".to_owned()),
                ("best", "false".to_owned()),
                ("best_ms", "0".to_owned())
            ]
        );
        // Stored values come back, trailing newline and all.
        let pairs = [("corner", "bottom_left\n"), ("best_ms", "83456")];
        let s = Settings::read(|key| {
            pairs.iter().find(|p| p.0 == key).map(|p| p.1.to_owned())
        });
        assert_eq!(s.corner, 2);
        assert_eq!(s.best_ms, 83_456);
        assert!(s.shown && s.best);
    }

    #[test]
    fn a_finished_run_is_appended_to_the_folder() {
        let dir = std::env::temp_dir().join("speedrun-timer-test-folder");
        let _ = std::fs::remove_dir_all(&dir);
        let path = save_run(&dir, 3_723_456_000_000).unwrap();
        save_run(&dir, 65_000_000_000).unwrap();
        let log = std::fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = log.lines().collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "finished_unix,time_ms,time");
        assert!(lines[1].ends_with(",3723456,01:02:03.456"));
        assert!(lines[2].ends_with(",65000,00:01:05.000"));
        assert_eq!(
            std::fs::read_to_string(dir.join("last_run.txt")).unwrap(),
            "00:01:05.000\n"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
