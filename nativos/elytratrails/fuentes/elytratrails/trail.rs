use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, OnceLock, PoisonError};

use crate::game::Game;
use crate::game::official_elytratrails as m;
use crate::game::official_elytratrails_net as net;
use horizon_client::gizmos::Line;
use horizon_client::mobs::{Mob, Part};

const OWN: u64 = u64::MAX;
const MIN_SPEED: f64 = 0.35;
const BOOST: f64 = 0.25;
const MAX_JUMP: f64 = 30.0;
const MAX_POINTS: usize = 2048;
const FORMATION_RANGE: f64 = 8.0;
const MIN_FADE: f32 = 0.02;
const FADE_IN_TICKS: u64 = 10;
const FLASH_TICKS: u64 = 40;
const FLASH_WIDTH: f32 = 1.6;
const FLASH_MIX: f32 = 0.35;
const DAY_BRIGHTNESS: f32 = 0.75;
const NIGHT_BRIGHTNESS: f32 = 1.25;
const DAY_LENGTH: i64 = 24000;
const NIGHT_START: i64 = 13000;
const NIGHT_END: i64 = 23000;
const SPEED_SPAN: f64 = 1.2;
const COLD: [f32; 3] = [0.3, 0.6, 1.0];
const HOT: [f32; 3] = [1.0, 0.5, 0.15];
const FORMATION_WIDTH: f32 = 1.0;
const TAPER: f32 = 0.55;
const GLOW_WIDTH: f32 = 2.4;
const GLOW_ALPHA: f32 = 0.22;
const FORMATION_ALPHA: f32 = 0.5;

#[derive(Clone, Copy)]
struct Point {
    pos: [f64; 3],
    born: u64,
    speed: f64,
    start: bool,
}

#[derive(Default)]
struct Wing {
    last: Option<[f64; 3]>,
    run_start: u64,
    points: VecDeque<Point>,
}

#[derive(Clone, Copy)]
struct Anchor {
    tips: [[f64; 3]; 2],
    body: [f64; 3],
}

impl Anchor {
    fn shifted(&self, at: [f64; 3]) -> [[f64; 3]; 2] {
        let d = [at[0] - self.body[0], at[1] - self.body[1], at[2] - self.body[2]];
        self.tips.map(|t| [t[0] + d[0], t[1] + d[1], t[2] + d[2]])
    }
}

#[derive(Default)]
struct Trail {
    colour: usize,
    pattern: usize,
    last_at: Option<[f64; 3]>,
    last_tick: u64,
    last_speed: f64,
    flash_until: u64,
    anchor: Option<Anchor>,
    wings: [Wing; 2],
}

struct State {
    now: u64,
    trails: HashMap<u64, Trail>,
}

struct Look {
    colour: usize,
    wavy: bool,
    dashed: bool,
    width: f32,
    brightness: f32,
    flash: bool,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);
static LOCAL_TIPS: OnceLock<[[f32; 3]; 2]> = OnceLock::new();

fn dist(a: [f64; 3], b: [f64; 3]) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

fn centre(tips: [[f64; 3]; 2]) -> [f64; 3] {
    [
        (tips[0][0] + tips[1][0]) / 2.0,
        (tips[0][1] + tips[1][1]) / 2.0,
        (tips[0][2] + tips[1][2]) / 2.0,
    ]
}

fn norm2(p: &[f32; 3]) -> f32 {
    p[0] * p[0] + p[1] * p[1] + p[2] * p[2]
}

fn argb(rgb: [f32; 3], alpha: f32) -> u32 {
    let channel = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u32;
    channel(alpha) << 24 | channel(rgb[0]) << 16 | channel(rgb[1]) << 8 | channel(rgb[2])
}

fn hue(h: f32) -> [f32; 3] {
    let h = h.fract() * 6.0;
    let x = 1.0 - (h % 2.0 - 1.0).abs();
    match h as u32 {
        0 => [1.0, x, 0.0],
        1 => [x, 1.0, 0.0],
        2 => [0.0, 1.0, x],
        3 => [0.0, x, 1.0],
        4 => [x, 0.0, 1.0],
        _ => [1.0, 0.0, x],
    }
}

fn speed_t(speed: f64) -> f32 {
    ((speed - MIN_SPEED) / SPEED_SPAN).clamp(0.0, 1.0) as f32
}

fn brightness(day_time: i64) -> f32 {
    let time = day_time.rem_euclid(DAY_LENGTH);
    if (NIGHT_START..NIGHT_END).contains(&time) {
        NIGHT_BRIGHTNESS
    } else {
        DAY_BRIGHTNESS
    }
}

fn pulse_factor(enabled: bool, clock: f64) -> f32 {
    if enabled {
        (0.8 + 0.2 * (clock * 0.4).sin()) as f32
    } else {
        1.0
    }
}

fn wave(born: u64) -> f64 {
    (born as f64 * 0.3).sin() * 0.15
}

fn lift(pos: [f64; 3], born: u64, wavy: bool) -> [f64; 3] {
    if wavy {
        [pos[0], pos[1] + wave(born), pos[2]]
    } else {
        pos
    }
}

fn fade_in(age: u64) -> f32 {
    (age as f32 / FADE_IN_TICKS as f32).min(1.0)
}

fn fade_out(age: u64, life: f64) -> f32 {
    let k = (1.0 - age as f64 / life).clamp(0.0, 1.0) as f32;
    k * k * (3.0 - 2.0 * k)
}

fn taper(age: u64, life: f64) -> f32 {
    1.0 - TAPER * (age as f64 / life).clamp(0.0, 1.0) as f32
}

fn find_part<'a>(part: &'a Part, name: &str) -> Option<&'a Part> {
    if part.name == name {
        return Some(part);
    }
    part.children.iter().find_map(|child| find_part(child, name))
}

fn part_matrix(
    part: &Part,
    parent: glam::Mat4,
    pose: &dyn Fn(&Part) -> [f32; 9],
    target: &str,
) -> Option<glam::Mat4> {
    let p = pose(part);
    let mat = parent
        * glam::Mat4::from_translation(glam::Vec3::new(p[0], p[1], p[2]) / 16.0)
        * glam::Mat4::from_quat(glam::Quat::from_euler(glam::EulerRot::ZYX, p[5], p[4], p[3]))
        * glam::Mat4::from_scale(glam::Vec3::new(p[6], p[7], p[8]));
    if part.name == target {
        return Some(mat);
    }
    part.children
        .iter()
        .find_map(|child| part_matrix(child, mat, pose, target))
}

fn wing_tip(part: &Part) -> Option<[f32; 3]> {
    part.polygons
        .iter()
        .flat_map(|polygon| polygon.vertices.iter())
        .map(|v| [v[0] / 16.0, v[1] / 16.0, v[2] / 16.0])
        .max_by(|a, b| norm2(a).total_cmp(&norm2(b)))
}

fn local_tips(model: &Part) -> Option<[[f32; 3]; 2]> {
    if let Some(tips) = LOCAL_TIPS.get() {
        return Some(*tips);
    }
    let tips = [
        wing_tip(find_part(model, "left_wing")?)?,
        wing_tip(find_part(model, "right_wing")?)?,
    ];
    let _ = LOCAL_TIPS.set(tips);
    Some(tips)
}

impl Look {
    fn paint(&self, born: u64, speed: f64) -> ([f32; 3], f32) {
        let (rgb, scale) = match self.colour {
            m::RAINBOW => (hue(born as f32 * 0.015), 1.0),
            m::SPEED => {
                let t = speed_t(speed);
                let rgb = [0, 1, 2].map(|i| COLD[i] + (HOT[i] - COLD[i]) * t);
                (rgb, 1.0 + t)
            }
            index => {
                let c = m::rgba_of(index);
                ([c[0], c[1], c[2]], 1.0)
            }
        };
        let mut rgb = rgb.map(|v| (v * self.brightness).clamp(0.0, 1.0));
        let mut width = self.width * scale;
        if self.flash {
            rgb = rgb.map(|v| v + (1.0 - v) * FLASH_MIX);
            width *= FLASH_WIDTH;
        }
        (rgb, width)
    }

    fn emit(&self, lines: &mut Vec<Line>, from: [f64; 3], to: [f64; 3], born: u64, speed: f64, alpha: f32, taper: f32) {
        let (rgb, width) = self.paint(born, speed);
        lines.push(Line {
            start: from,
            end: to,
            color: argb(rgb, alpha),
            width: width * taper,
        });
    }

    fn glow(&self, lines: &mut Vec<Line>, from: [f64; 3], to: [f64; 3], born: u64, speed: f64, alpha: f32) {
        let (rgb, width) = self.paint(born, speed);
        lines.push(Line {
            start: from,
            end: to,
            color: argb(rgb, alpha * GLOW_ALPHA),
            width: width * GLOW_WIDTH,
        });
    }
}

impl Wing {
    fn push(&mut self, tip: [f64; 3], tick: u64, speed: f64) {
        let gap = self.last.is_none();
        let jump = self.last.is_some_and(|prev| dist(prev, tip) > MAX_JUMP);
        let start = gap || jump;
        if start {
            self.run_start = tick;
        }
        self.last = Some(tip);
        self.points.push_back(Point {
            pos: tip,
            born: tick,
            speed,
            start,
        });
        while self.points.len() > MAX_POINTS {
            self.points.pop_front();
        }
    }
}

impl Game<'_> {
    fn elytratrails_tips(&self, mob: &Mob, partial: f32) -> Option<[[f64; 3]; 2]> {
        let model = self.armor_stand_assets.player.get("elytra")?;
        let local = local_tips(model)?;
        let crouching = mob.extra.crouching;
        let angles = mob.extra.elytra.angles(partial);
        let pose = move |part: &Part| -> [f32; 9] {
            let mut p = part.pose;
            if part.name == "left_wing" || part.name == "right_wing" {
                let invert = if part.name == "left_wing" { 1.0 } else { -1.0 };
                p[1] = if crouching { 3.0 } else { 0.0 };
                p[3] = angles[0];
                p[4] = invert * angles[1];
                p[5] = invert * angles[2];
            }
            p
        };
        let base = horizon_client::mobs::render::base_transform(mob, partial, glam::DVec3::ZERO)
            * glam::Mat4::from_translation(glam::Vec3::new(0.0, 0.0, 0.125));
        let mut out = [[0.0f64; 3]; 2];
        for (slot, name) in ["left_wing", "right_wing"].into_iter().enumerate() {
            let mat = part_matrix(model, base, &pose, name)?;
            let world = mat.transform_point3(glam::Vec3::from(local[slot]));
            out[slot] = [f64::from(world.x), f64::from(world.y), f64::from(world.z)];
        }
        Some(out)
    }

    fn elytratrails_sample(
        &self,
        trails: &mut HashMap<u64, Trail>,
        id: u64,
        mob: &Mob,
        origin: Option<[f64; 3]>,
        tick: u64,
        style: (usize, usize),
        settings: &m::Settings,
    ) {
        let fixed = if id == OWN {
            settings.own_fixed
        } else {
            settings.others_fixed
        };
        let at = [mob.position.x, mob.position.y, mob.position.z];
        let near = origin.map_or(true, |o| dist(o, at) <= settings.range_blocks());
        let trail = trails.entry(id).or_default();
        let v = mob.velocity;
        let velocity = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
        let moved = match trail.last_at {
            Some(prev) if dist(prev, at) <= MAX_JUMP => {
                dist(prev, at) / tick.saturating_sub(trail.last_tick).max(1) as f64
            }
            _ => 0.0,
        };
        let speed = if id == OWN { velocity } else { velocity.max(moved) };
        trail.last_at = Some(at);
        trail.last_tick = tick;
        let fast_enough = id != OWN || speed >= MIN_SPEED;
        let gliding = mob.extra.fall_flying && fast_enough && near;
        let tips = if !gliding {
            None
        } else if fixed {
            if trail.anchor.is_none() {
                trail.anchor = self
                    .elytratrails_tips(mob, 0.0)
                    .map(|tips| Anchor { tips, body: at });
            }
            trail.anchor.map(|anchor| anchor.shifted(at))
        } else {
            self.elytratrails_tips(mob, 0.0)
        };
        if !(gliding && fixed) {
            trail.anchor = None;
        }
        trail.colour = style.0;
        trail.pattern = style.1;
        match tips {
            Some(tips) => {
                if speed - trail.last_speed > BOOST {
                    trail.flash_until = tick + FLASH_TICKS;
                }
                trail.last_speed = speed;
                for (wing, tip) in trail.wings.iter_mut().zip(tips) {
                    wing.push(tip, tick, speed);
                }
            }
            None => {
                trail.last_speed = 0.0;
                for wing in trail.wings.iter_mut() {
                    wing.last = None;
                }
            }
        }
    }

    pub(super) fn elytratrails_step(&mut self) {
        if !self.native_window() {
            return;
        }
        let settings = self.elytratrails_settings();
        let multiplayer = self.remote.is_some() || self.lan.is_some();
        let others_shown = settings.others && multiplayer;
        let own_shown = settings.own && (settings.first_person || !self.camera_type.first_person());
        let origin = self
            .player
            .as_ref()
            .map(|p| [p.position.x, p.position.y, p.position.z]);
        let mut guard = STATE.lock().unwrap_or_else(PoisonError::into_inner);
        let State { now, trails } = guard.get_or_insert_with(|| State {
            now: 0,
            trails: HashMap::new(),
        });
        *now += 1;
        let tick = *now;
        trails.retain(|id, _| {
            settings.trails
                && if *id == OWN {
                    own_shown
                } else {
                    others_shown
                }
        });
        if !settings.trails {
            return;
        }
        if own_shown {
            if let Some(mob) = self.player_model.as_ref() {
                let style = (settings.color, settings.pattern);
                self.elytratrails_sample(trails, OWN, mob, origin, tick, style, &settings);
            }
        }
        if others_shown {
            for mob in self
                .mobs
                .iter()
                .filter(|mob| &*mob.kind.id == "minecraft:player")
            {
                let style = net::style_of(mob.id).unwrap_or((0, 0));
                self.elytratrails_sample(trails, u64::from(mob.id), mob, origin, tick, style, &settings);
            }
        }
        let life = settings.life();
        trails.retain(|_, trail| {
            trail.wings.iter_mut().fold(false, |alive, wing| {
                while wing
                    .points
                    .front()
                    .is_some_and(|point| tick - point.born >= life)
                {
                    wing.points.pop_front();
                }
                alive || !wing.points.is_empty() || wing.last.is_some()
            })
        });
    }

    pub(super) fn elytratrails_draw(&self, partial: f32, lines: &mut Vec<Line>) {
        let settings = self.elytratrails_settings();
        if !settings.trails {
            return;
        }
        let guard = STATE.lock().unwrap_or_else(PoisonError::into_inner);
        let Some(state) = guard.as_ref() else {
            return;
        };
        let life = settings.life() as f64;
        let width = settings.px();
        let half = settings.half_detail;
        let brightness = if settings.day_night {
            brightness(self.world.day_time)
        } else {
            1.0
        };
        let pulse = pulse_factor(settings.pulse, state.now as f64 + f64::from(partial));
        let players: HashMap<u64, &Mob> = if settings.others && (self.remote.is_some() || self.lan.is_some()) {
            self.mobs
                .iter()
                .filter(|mob| &*mob.kind.id == "minecraft:player")
                .map(|mob| (u64::from(mob.id), mob))
                .collect()
        } else {
            HashMap::new()
        };
        let mut heads: Vec<[[f64; 3]; 2]> = Vec::new();
        for (id, trail) in &state.trails {
            let mob = if *id == OWN {
                self.player_model.as_ref()
            } else {
                players.get(id).copied()
            };
            let tips = mob.filter(|mob| mob.extra.fall_flying).and_then(|mob| match trail.anchor {
                Some(anchor) => Some(anchor.shifted([mob.position.x, mob.position.y, mob.position.z])),
                None => self.elytratrails_tips(mob, partial),
            });
            let look = Look {
                colour: trail.colour,
                wavy: trail.pattern == m::WAVY,
                dashed: trail.pattern == m::DASHED,
                width,
                brightness,
                flash: settings.flash && state.now < trail.flash_until,
            };
            for (slot, wing) in trail.wings.iter().enumerate() {
                let mut previous: Option<&Point> = None;
                for (index, point) in wing.points.iter().enumerate() {
                    if let (Some(prev), false) = (previous, point.start) {
                        let age = state.now - point.born;
                        let fade = fade_out(age, life) * fade_in(age) * pulse;
                        let gap = look.dashed && (index / 3) % 2 == 1;
                        let skip = half && index % 2 == 1;
                        if !gap && !skip && fade > MIN_FADE {
                            let from = lift(prev.pos, prev.born, look.wavy);
                            let to = lift(point.pos, point.born, look.wavy);
                            let tapered = taper(age, life);
                            look.emit(lines, from, to, point.born, point.speed, fade, tapered);
                        }
                    }
                    previous = Some(point);
                }
                if let (Some(last), Some(tips), true) = (wing.points.back(), tips, wing.last.is_some()) {
                    if dist(last.pos, tips[slot]) <= MAX_JUMP {
                        let from = lift(last.pos, last.born, look.wavy);
                        let to = lift(tips[slot], last.born, look.wavy);
                        let alpha = fade_in(state.now - wing.run_start) * pulse;
                        look.glow(lines, from, to, last.born, last.speed, alpha);
                        look.emit(lines, from, to, last.born, last.speed, alpha, 1.0);
                    }
                }
            }
            if let Some(tips) = tips {
                heads.push(tips);
            }
        }
        if !settings.formation {
            return;
        }
        let centres: Vec<[f64; 3]> = heads.iter().map(|tips| centre(*tips)).collect();
        let formation = argb([1.0, 1.0, 1.0], FORMATION_ALPHA * pulse);
        for (i, a) in heads.iter().enumerate() {
            for (j, b) in heads.iter().enumerate().skip(i + 1) {
                if dist(centres[i], centres[j]) <= FORMATION_RANGE {
                    for slot in 0..2 {
                        lines.push(Line {
                            start: a[slot],
                            end: b[slot],
                            color: formation,
                            width: FORMATION_WIDTH,
                        });
                    }
                }
            }
        }
    }
}
