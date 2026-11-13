// Pure client data logic and algorithms for Basic HUD 0.3.
// Independent of Game structs so it can be verified in standalone tests.
use std::collections::{HashMap, VecDeque};

pub const GOOD: u32 = 0x7C_E0_7C;
pub const BAD: u32 = 0xFF_70_70;
pub const WARNING: u32 = 0xFF_C8_40;
pub const INK: u32 = 0xFF_FFFF;
pub const AUTO_TEXT: u32 = 0xFFFF_FFFF;
/// Text override shared by preview and live HUD.
pub fn widget_text_color(configured: u32, status: u32) -> u32 {
    if configured == AUTO_TEXT { status } else { configured & 0xFF_FFFF }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ObservedHealth {
    pub name: String,
    pub hp: Option<f32>,
    pub absorption: Option<f32>,
}

#[derive(Clone, Debug, Default)]
pub struct HealthStore {
    entries: HashMap<u32, ObservedHealth>,
}

impl HealthStore {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn observe(
        &mut self,
        id: u32,
        confirmed_name: &str,
        hp: Option<f32>,
        absorption: Option<f32>,
    ) {
        let valid_hp = hp.filter(|v| v.is_finite() && *v >= 0.0);
        let valid_abs = absorption.filter(|v| v.is_finite() && *v >= 0.0);

        if valid_hp.is_none() && valid_abs.is_none() {
            return;
        }

        let entry = self.entries.entry(id).or_insert_with(|| ObservedHealth {
            name: confirmed_name.to_owned(),
            hp: None,
            absorption: None,
        });

        // If entity ID was recycled for another player name, reset transient state
        if entry.name != confirmed_name {
            entry.name = confirmed_name.to_owned();
            entry.hp = None;
            entry.absorption = None;
        }

        if let Some(h) = valid_hp {
            entry.hp = Some(h);
        }
        if let Some(a) = valid_abs {
            entry.absorption = Some(a);
        }
    }

    pub fn remove(&mut self, id: u32) {
        self.entries.remove(&id);
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn cleanup_stale(&mut self, confirmed: &[(u32, String)]) {
        self.entries.retain(|id, entry| {
            confirmed.iter().any(|(cid, cname)| cid == id && cname == &entry.name)
        });
    }

    pub fn get(&self, id: u32, confirmed_name: Option<&str>) -> Option<&ObservedHealth> {
        let entry = self.entries.get(&id)?;
        if let Some(expected_name) = confirmed_name {
            if entry.name != expected_name {
                return None;
            }
        }
        Some(entry)
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HealthSnapshot {
    pub name: String,
    pub hp: f32,
    pub max_hp: Option<f32>,
    pub absorption: f32,
}

impl HealthSnapshot {
    pub fn new(name: impl Into<String>, hp: f32, max_hp: Option<f32>, absorption: f32) -> Option<Self> {
        if !hp.is_finite() || hp < 0.0 {
            return None;
        }
        let valid_max = max_hp.filter(|m| m.is_finite() && *m > 0.0);
        let valid_abs = if absorption.is_finite() && absorption >= 0.0 {
            absorption
        } else {
            0.0
        };
        Some(Self {
            name: name.into(),
            hp,
            max_hp: valid_max,
            absorption: valid_abs,
        })
    }
}

pub fn format_health_line(snapshot: &HealthSnapshot) -> String {
    let hp = snapshot.hp;
    let abs = snapshot.absorption;
    match (abs > 0.0, snapshot.max_hp) {
        (true, Some(max)) => format!("{hp:.0}+{abs:.0}/{max:.0} HP"),
        (true, None) => format!("{hp:.0}+{abs:.0} HP"),
        (false, Some(max)) => format!("{hp:.0}/{max:.0} HP"),
        (false, None) => format!("{hp:.0} HP"),
    }
}

pub fn health_color(hp: f32, max_hp: Option<f32>) -> u32 {
    let max = max_hp.unwrap_or(20.0);
    if max <= 0.0 {
        return GOOD;
    }
    let ratio = hp / max;
    if ratio > 0.5 {
        GOOD
    } else if ratio > 0.2 {
        WARNING
    } else {
        BAD
    }
}

pub fn cooldown_remaining(start: i32, end: i32, current_tick: i32) -> Option<(i32, f32)> {
    let remaining = end.wrapping_sub(current_tick);
    if remaining <= 0 {
        return None;
    }
    let total = end.wrapping_sub(start);
    if total <= 0 {
        return None;
    }
    let secs = remaining as f32 / 20.0;
    Some((remaining, secs))
}

pub fn format_cooldown(name: &str, secs: f32) -> String {
    let clean = name.trim_matches(['[', ']']);
    format!("{clean}: {secs:.1}s")
}

pub fn cooldown_color(remaining_ticks: i32) -> u32 {
    if remaining_ticks < 40 {
        WARNING
    } else {
        INK
    }
}

pub fn clock_time_from_ticks(ticks: i64) -> (u32, u32) {
    let day_ticks = (ticks % 24000 + 24000) % 24000;
    let hours = ((day_ticks / 1000 + 6) % 24) as u32;
    let minutes = ((day_ticks % 1000) * 60 / 1000) as u32;
    (hours, minutes)
}

pub fn format_clock(hours: u32, minutes: u32, ru: bool) -> String {
    if ru {
        format!("Время: {hours:02}:{minutes:02}")
    } else {
        format!("Time: {hours:02}:{minutes:02}")
    }
}

pub fn format_utc_clock(secs_since_epoch: u64) -> String {
    let h = (secs_since_epoch / 3600) % 24;
    let min = (secs_since_epoch / 60) % 60;
    format!("{h:02}:{min:02} UTC")
}

pub fn facing_from_yaw(yaw: f32, ru: bool) -> &'static str {
    let norm = (yaw % 360.0 + 360.0) % 360.0;
    match ((norm + 45.0) / 90.0) as u32 % 4 {
        0 => if ru { "Юг (+Z)" } else { "South (+Z)" },
        1 => if ru { "Запад (-X)" } else { "West (-X)" },
        2 => if ru { "Север (-Z)" } else { "North (-Z)" },
        _ => if ru { "Восток (+X)" } else { "East (+X)" },
    }
}

pub fn format_dimension(dim: &str, ru: bool) -> &str {
    match dim {
        "minecraft:overworld" => if ru { "Верхний мир" } else { "Overworld" },
        "minecraft:the_nether" => if ru { "Нижний мир" } else { "The Nether" },
        "minecraft:the_end" => if ru { "Энд" } else { "The End" },
        other => other.strip_prefix("minecraft:").unwrap_or(other),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PingStatus {
    Local,
    Unknown,
    Latency(i32),
}

pub fn format_ping(status: PingStatus, ru: bool) -> (String, u32) {
    match status {
        PingStatus::Latency(lat) => {
            let color = if lat < 80 {
                GOOD
            } else if lat < 150 {
                WARNING
            } else {
                BAD
            };
            let text = if ru {
                format!("Пинг: {lat} мс")
            } else {
                format!("Ping: {lat} ms")
            };
            (text, color)
        }
        PingStatus::Unknown => {
            let text = if ru {
                "Пинг: неизвестно".to_owned()
            } else {
                "Ping: unknown".to_owned()
            };
            (text, WARNING)
        }
        PingStatus::Local => {
            let text = if ru {
                "Пинг: локально".to_owned()
            } else {
                "Ping: local".to_owned()
            };
            (text, INK)
        }
    }
}

pub fn classify_ping(local: bool, latency: Option<i32>) -> PingStatus {
    if local {
        PingStatus::Local
    } else {
        match latency {
            Some(lat) if lat >= 0 => PingStatus::Latency(lat),
            _ => PingStatus::Unknown,
        }
    }
}

pub fn format_fps(
    fps: u32,
    min: u32,
    low: u32,
    label: bool,
    lows: bool,
    auto: bool,
    scale: f32,
    compact: bool,
) -> String {
    let show_details = lows && !compact && (!auto || (scale.is_finite() && scale >= 0.85));
    let base = if label {
        format!("{fps} FPS")
    } else {
        format!("{fps}")
    };
    if show_details {
        format!("{base} (min: {min}, 1%: {low})")
    } else {
        base
    }
}

/// Time-windowed horizontal speed meter (blocks per second).
/// Frame-rate independent across 20-240+ FPS, quantized-tick smoothed,
/// with finite guards and discontinuity/teleport reset.
#[derive(Clone, Debug, PartialEq)]
pub struct SpeedMeter {
    samples: VecDeque<(f64, f64)>, // (distance, dt)
    total_dist: f64,
    total_dt: f64,
    last_pos: Option<(f64, f64)>,
    value: f64,
}

impl Default for SpeedMeter {
    fn default() -> Self {
        Self {
            samples: VecDeque::with_capacity(32),
            total_dist: 0.0,
            total_dt: 0.0,
            last_pos: None,
            value: 0.0,
        }
    }
}

impl SpeedMeter {
    pub const WINDOW: f64 = 0.5;
    pub const TELEPORT_BPS_THRESHOLD: f64 = 100.0;
    pub const TELEPORT_DIST_THRESHOLD: f64 = 64.0;

    pub fn reset(&mut self) {
        self.samples.clear();
        self.total_dist = 0.0;
        self.total_dt = 0.0;
        self.last_pos = None;
        self.value = 0.0;
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn sample(&mut self, x: f64, z: f64, dt: f64) -> f64 {
        if !x.is_finite() || !z.is_finite() || !dt.is_finite() || dt <= 0.0 {
            return self.value;
        }

        let Some((lx, lz)) = self.last_pos else {
            self.last_pos = Some((x, z));
            return self.value;
        };

        let dx = x - lx;
        let dz = z - lz;
        let dist = (dx * dx + dz * dz).sqrt();

        // Discontinuity / Teleport check
        // Tick-sized position bursts are legitimate even at very high render FPS.
        // Only a large displacement or a long frame is a discontinuity.
        if dt > 0.5 || dist > Self::TELEPORT_DIST_THRESHOLD || (dist > 8.0 && dist / dt > Self::TELEPORT_BPS_THRESHOLD) {
            self.reset();
            self.last_pos = Some((x, z));
            return 0.0;
        }

        self.last_pos = Some((x, z));
        self.samples.push_back((dist, dt));
        self.total_dist += dist;
        self.total_dt += dt;

        while self.samples.len() > 1 && self.total_dt > Self::WINDOW {
            let (old_dist, old_dt) = *self.samples.front().unwrap();
            if self.total_dt - old_dt < 0.25 {
                break;
            }
            self.samples.pop_front();
            self.total_dist = (self.total_dist - old_dist).max(0.0);
            self.total_dt = (self.total_dt - old_dt).max(0.0);
        }

        if self.total_dt > 0.0 {
            self.value = self.total_dist / self.total_dt;
        } else {
            self.value = 0.0;
        }

        self.value
    }
}

#[cfg(test)] // Historical filter regression; live sampling now uses SpeedMeter.
pub fn smooth_speed(previous_bps: f64, sample_bps: f64) -> f64 {
    if !sample_bps.is_finite() || sample_bps < 0.0 {
        return previous_bps;
    }
    previous_bps * 0.85 + sample_bps * 0.15
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_store_tracks_and_clears_by_identity() {
        let mut store = HealthStore::new();
        store.observe(101, "Alice", Some(18.0), Some(4.0));
        assert_eq!(store.len(), 1);

        let h = store.get(101, Some("Alice")).unwrap();
        assert_eq!(h.hp, Some(18.0));
        assert_eq!(h.absorption, Some(4.0));

        // Unknown player name matching ID fails
        assert!(store.get(101, Some("Bob")).is_none());

        // ID reuse resets prior data
        store.observe(101, "Bob", Some(20.0), None);
        let h2 = store.get(101, Some("Bob")).unwrap();
        assert_eq!(h2.hp, Some(20.0));
        assert_eq!(h2.absorption, None);

        // Stale cleanup removes non-confirmed IDs
        let confirmed = vec![(202, "Charlie".to_string())];
        store.cleanup_stale(&confirmed);
        assert!(store.is_empty());
    }

    #[test]
    fn health_snapshot_and_formatting_support_up_to_40_hp() {
        let s = HealthSnapshot::new("Steve", 20.0, Some(20.0), 16.0).unwrap();
        assert_eq!(format_health_line(&s), "20+16/20 HP");

        let s2 = HealthSnapshot::new("Alex", 14.0, Some(20.0), 0.0).unwrap();
        assert_eq!(format_health_line(&s2), "14/20 HP");

        let s_no_max = HealthSnapshot::new("Guest", 30.0, None, 10.0).unwrap();
        assert_eq!(format_health_line(&s_no_max), "30+10 HP");

        // Non-finite or negative values fail
        assert!(HealthSnapshot::new("Bad", f32::NAN, Some(20.0), 0.0).is_none());
        assert!(HealthSnapshot::new("Bad", -1.0, Some(20.0), 0.0).is_none());
    }

    #[test]
    fn cooldown_remaining_handles_wrapping_ticks() {
        let start: i32 = i32::MAX - 50;
        let end: i32 = start.wrapping_add(200);
        let tick: i32 = start.wrapping_add(60);

        let (rem_ticks, secs) = cooldown_remaining(start, end, tick).unwrap();
        assert_eq!(rem_ticks, 140);
        assert!((secs - 7.0).abs() < 1e-4);

        // Expired cooldown returns None
        let tick_after = end.wrapping_add(5);
        assert!(cooldown_remaining(start, end, tick_after).is_none());

        assert_eq!(format_cooldown("[Ender Pearl]", 7.0), "Ender Pearl: 7.0s");
    }

    #[test]
    fn clock_mapping_calculates_correct_minecraft_time() {
        assert_eq!(clock_time_from_ticks(0), (6, 0));
        assert_eq!(clock_time_from_ticks(6000), (12, 0));
        assert_eq!(clock_time_from_ticks(12000), (18, 0));
        assert_eq!(clock_time_from_ticks(18000), (0, 0));
        assert_eq!(clock_time_from_ticks(23000), (5, 0));

        assert_eq!(format_clock(14, 5, false), "Time: 14:05");
        assert_eq!(format_clock(14, 5, true), "Время: 14:05");
    }

    #[test]
    fn facing_direction_from_yaw() {
        assert_eq!(facing_from_yaw(0.0, false), "South (+Z)");
        assert_eq!(facing_from_yaw(90.0, false), "West (-X)");
        assert_eq!(facing_from_yaw(180.0, false), "North (-Z)");
        assert_eq!(facing_from_yaw(270.0, false), "East (+X)");
        assert_eq!(facing_from_yaw(-90.0, false), "East (+X)");

        assert_eq!(facing_from_yaw(0.0, true), "Юг (+Z)");
    }

    #[test]
    fn ping_formatting_and_colors() {
        let (t_good, c_good) = format_ping(PingStatus::Latency(40), false);
        assert_eq!(t_good, "Ping: 40 ms");
        assert_eq!(c_good, GOOD);

        let (t_warn, c_warn) = format_ping(PingStatus::Latency(120), false);
        assert_eq!(t_warn, "Ping: 120 ms");
        assert_eq!(c_warn, WARNING);

        let (t_bad, c_bad) = format_ping(PingStatus::Latency(250), false);
        assert_eq!(t_bad, "Ping: 250 ms");
        assert_eq!(c_bad, BAD);

        let (t_unk, c_unk) = format_ping(PingStatus::Unknown, true);
        assert_eq!(t_unk, "Пинг: неизвестно");
        assert_eq!(c_unk, WARNING);

        let (t_loc, c_loc) = format_ping(PingStatus::Local, false);
        assert_eq!(t_loc, "Ping: local");
        assert_eq!(c_loc, INK);
    }

    #[test]
    fn smooth_speed_applies_exponential_moving_average() {
        let initial = 0.0;
        let smoothed = smooth_speed(initial, 10.0);
        assert!((smoothed - 1.5).abs() < 1e-4);

        let smoothed2 = smooth_speed(smoothed, 10.0);
        assert!((smoothed2 - (1.5 * 0.85 + 1.5)).abs() < 1e-4);

        // Non-finite ignored
        assert_eq!(smooth_speed(5.0, f64::NAN), 5.0);
    }

    #[test]
    fn format_fps_label_lows_compact_and_auto_scale() {
        // default 400 FPS without lows
        assert_eq!(format_fps(400, 250, 200, true, false, true, 1.0, false), "400 FPS");
        // optional unit off
        assert_eq!(format_fps(400, 250, 200, false, false, true, 1.0, false), "400");
        // with lows and label
        assert_eq!(
            format_fps(400, 250, 200, true, true, true, 1.0, false),
            "400 FPS (min: 250, 1%: 200)"
        );
        // with lows, label off
        assert_eq!(
            format_fps(400, 250, 200, false, true, true, 1.0, false),
            "400 (min: 250, 1%: 200)"
        );
        // auto-hide when scale < 0.85
        assert_eq!(
            format_fps(400, 250, 200, true, true, true, 0.84, false),
            "400 FPS"
        );
        // scale >= 0.85 keeps details when auto is on
        assert_eq!(
            format_fps(400, 250, 200, true, true, true, 0.85, false),
            "400 FPS (min: 250, 1%: 200)"
        );
        // compact mode hides details even when scale is normal
        assert_eq!(
            format_fps(400, 250, 200, true, true, true, 1.0, true),
            "400 FPS"
        );
        // when auto is false, small scale does not hide details
        assert_eq!(
            format_fps(400, 250, 200, true, true, false, 0.5, false),
            "400 FPS (min: 250, 1%: 200)"
        );
        // non-finite scale with auto hides details
        assert_eq!(
            format_fps(400, 250, 200, true, true, true, f32::NAN, false),
            "400 FPS"
        );
    }

    #[test]
    fn classify_ping_local_and_latency() {
        assert_eq!(classify_ping(true, Some(0)), PingStatus::Local);
        assert_eq!(classify_ping(true, Some(50)), PingStatus::Local);
        assert_eq!(classify_ping(true, None), PingStatus::Local);

        assert_eq!(classify_ping(false, Some(45)), PingStatus::Latency(45));
        assert_eq!(classify_ping(false, Some(0)), PingStatus::Latency(0));
        assert_eq!(classify_ping(false, Some(-1)), PingStatus::Unknown);
        assert_eq!(classify_ping(false, None), PingStatus::Unknown);
    }

    #[test]
    fn speed_meter_time_window_sampling_and_quantized_ticks() {
        let mut meter = SpeedMeter::default();
        assert_eq!(meter.value(), 0.0);

        // Simulation at 20 FPS (dt = 0.05, 1 tick per frame, moving 0.5 blocks per tick = 10 bps)
        meter.reset();
        let mut x = 0.0;
        for _ in 0..20 {
            x += 0.5;
            meter.sample(x, 0.0, 0.05);
        }
        let speed_20 = meter.value();
        assert!((speed_20 - 10.0).abs() < 0.1, "speed_20 was {speed_20}");

        // Simulation at 60 FPS (dt ≈ 0.01667, quantized: jumps 0.5 blocks every 3 frames = 10 bps)
        let mut meter_60 = SpeedMeter::default();
        let mut x60 = 0.0;
        for i in 0..60 {
            if i % 3 == 0 {
                x60 += 0.5;
            }
            meter_60.sample(x60, 0.0, 1.0 / 60.0);
        }
        let speed_60 = meter_60.value();
        assert!((speed_60 - 10.0).abs() < 0.5, "speed_60 was {speed_60}");

        // Simulation at 144 FPS (dt ≈ 0.00694, quantized ticks at 20 Hz = ~7.2 frames/tick)
        let mut meter_144 = SpeedMeter::default();
        let mut x144 = 0.0;
        for i in 0..144 {
            if i % 7 == 0 {
                x144 += 0.486; // approximate 20 ticks per 144 frames
            }
            meter_144.sample(x144, 0.0, 1.0 / 144.0);
        }
        let speed_144 = meter_144.value();
        assert!((speed_144 - 10.0).abs() < 0.8, "speed_144 was {speed_144}");
    }

    #[test]
    fn speed_meter_teleport_discontinuity_and_finite_guards() {
        let mut meter = SpeedMeter::default();
        // Initial pos
        meter.sample(10.0, 10.0, 0.05);
        meter.sample(10.5, 10.0, 0.05);
        assert!(meter.value() > 0.0);

        // Teleport 500 blocks away in one frame
        let val_tp = meter.sample(510.5, 10.0, 0.05);
        assert_eq!(val_tp, 0.0);
        assert_eq!(meter.value(), 0.0);

        // Next normal movement operates from teleported position without spike
        meter.sample(511.0, 10.0, 0.05);
        meter.sample(511.5, 10.0, 0.05);
        assert!(meter.value() < 20.0);

        // Finite guards: NaN/Inf inputs do not alter cached value or crash
        let current = meter.value();
        assert_eq!(meter.sample(f64::NAN, 10.0, 0.05), current);
        assert_eq!(meter.sample(512.0, f64::INFINITY, 0.05), current);
        assert_eq!(meter.sample(512.0, 10.0, -0.05), current);
        assert_eq!(meter.sample(512.0, 10.0, 0.0), current);
        assert_eq!(meter.value(), current);
    }
}
