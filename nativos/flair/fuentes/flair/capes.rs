use super::umbx_flair_items::{Item, Moves, Paint, Slot};
use super::umbx_flair_model::{Bone, Cube, NONE, bone};
use super::umbx_flair_paint::{Canvas, hue, mix, noise, opaque, palette};

pub const STYLES: [&str; 12] = ["Royal", "Nebula", "Ember", "Prism", "Frost", "Tide", "Aurora", "Great Wave", "Sakura", "Storm", "Code Rain", "Synthwave"];
pub const FACE_W: usize = 10;
pub const FACE_H: usize = 16;
pub const SEGMENTS: usize = 8;
pub const SEGMENT_H: f32 = 2.0;
pub const CAPE_Z: f32 = 2.1;
const HALF_W: f32 = 5.0;
const REST_TILT: f32 = 0.08;
const LIFT_BASE: f32 = 0.1;
const LIFT_GROWTH: f32 = 0.025;
const RIPPLE: f32 = 0.035;
const RIPPLE_SPEED: f32 = 2.6;
const RIPPLE_STEP: f32 = 0.7;
const SIDE_SWAY: f32 = 0.5;
const FILL: f32 = 0.3;
const INSET: f32 = 0.02;
const TILE_TEXELS_PER_PX: f32 = 16.0;
const CAPE_ROWS: usize = 32;
const WHITE: u32 = 0xFF_FFFF;
const BLACK: u32 = 0;
const ROYAL: u32 = 0x4B_2496;
const GOLD: u32 = 0xF2_C14E;

pub fn items() -> Vec<Item> {
    STYLES
        .iter()
        .enumerate()
        .map(|(style, name)| Item::new(&name.to_lowercase().replace(' ', "_"), name, Slot::Cape, chain(classic), Paint::Live(Box::new(move |frame| cape(style, frame))), Moves::Still))
        .collect()
}

pub fn bend([lift, side]: [f32; 2], seconds: f32, i: usize) -> [f32; 3] {
    let along = i as f32 / SEGMENTS as f32;
    let rest = if i == 0 { REST_TILT } else { 0.0 };
    let ripple = RIPPLE * (1.0 + lift) * (seconds * RIPPLE_SPEED - i as f32 * RIPPLE_STEP).sin();
    [rest + lift * (LIFT_BASE + LIFT_GROWTH * i as f32) + ripple, 0.0, side * SIDE_SWAY * along]
}

pub fn chain(cubes: impl Fn(usize) -> Vec<Cube>) -> Vec<Bone> {
    let mut below: Option<Bone> = None;
    for i in (0..SEGMENTS).rev() {
        let mut segment = bone(&format!("seg{i}"), [0.0, SEGMENT_H * i as f32, CAPE_Z + 0.5], cubes(i));
        segment.children.extend(below.take());
        below = Some(segment);
    }
    below.into_iter().collect()
}

fn segment_box(i: usize, x: f32, width: f32) -> ([f32; 3], [f32; 3]) {
    ([x, SEGMENT_H * i as f32, CAPE_Z], [width, SEGMENT_H, 1.0])
}

fn filler_box(i: usize, x: f32, width: f32) -> ([f32; 3], [f32; 3]) {
    ([x + INSET, SEGMENT_H * i as f32 - FILL, CAPE_Z + INSET], [width - 2.0 * INSET, FILL, 1.0 - 2.0 * INSET])
}

pub fn classic(i: usize) -> Vec<Cube> {
    let row = 1.0 + SEGMENT_H * i as f32;
    let next = row + SEGMENT_H;
    let top = if i == 0 { [1.0, 0.0, 11.0, 1.0] } else { NONE };
    let bottom = if i + 1 == SEGMENTS { [11.0, 0.0, 21.0, 1.0] } else { NONE };
    let (from, size) = segment_box(i, -HALF_W, 2.0 * HALF_W);
    let mut cubes = vec![Cube::faced(from, size, [top, bottom, [0.0, row, 1.0, next], [12.0, row, 22.0, next], [11.0, row, 12.0, next], [1.0, row, 11.0, next]], 0)];
    if i > 0 {
        let band = row + FILL;
        let (from, size) = filler_box(i, -HALF_W, 2.0 * HALF_W);
        cubes.push(Cube::faced(from, size, [NONE, NONE, [0.0, row, 1.0, band], [12.0, row, 22.0, band], [11.0, row, 12.0, band], [1.0, row, 11.0, band]], 0));
    }
    cubes
}

pub fn tiled(i: usize) -> Vec<Cube> {
    let half = (i % 2) as f32 * 32.0;
    let outer = [0.0, half, 64.0, half + 32.0];
    let lining = [0.0, 0.0, 8.0, 8.0];
    let band = [0.0, half, 64.0, half + FILL * TILE_TEXELS_PER_PX];
    let mut cubes = Vec::new();
    for c in 0..2 {
        let x = if c == 0 { 0.0 } else { -HALF_W };
        let (from, size) = segment_box(i, x, HALF_W);
        cubes.push(Cube::faced(from, size, super::umbx_flair_model::only(5, outer), c + 2 * (i / 2)));
        if i > 0 {
            let (from, size) = filler_box(i, x, HALF_W);
            cubes.push(Cube::faced(from, size, super::umbx_flair_model::only(5, band), c + 2 * (i / 2)));
        }
    }
    let (from, size) = segment_box(i, -HALF_W, 2.0 * HALF_W);
    let top = if i == 0 { lining } else { NONE };
    let bottom = if i + 1 == SEGMENTS { lining } else { NONE };
    cubes.push(Cube::faced(from, size, [top, bottom, lining, lining, lining, NONE], 8));
    if i > 0 {
        let (from, size) = filler_box(i, -HALF_W, 2.0 * HALF_W);
        cubes.push(Cube::faced(from, size, [NONE, NONE, lining, lining, lining, NONE], 8));
    }
    cubes
}

pub fn cape(style: usize, frame: u32) -> Canvas {
    let mut canvas = Canvas::default();
    let t = f64::from(frame);
    for y in 0..CAPE_ROWS {
        let v = y.saturating_sub(1).min(FACE_H - 1);
        for x in 0..64 {
            let (u, inner) = match x {
                1..=10 => (x - 1, false),
                12..=21 => (21 - x, true),
                _ => (x % FACE_W, true),
            };
            let color = shade(style, u, v, t);
            canvas.put(x, y, opaque(if inner { mix(color, BLACK, 0.3) } else { color }));
        }
    }
    canvas
}

fn shade(style: usize, u: usize, v: usize, t: f64) -> u32 {
    let (fu, fv) = (u as f64 / (FACE_W - 1) as f64, v as f64 / (FACE_H - 1) as f64);
    let (ui, vi) = (u as i64, v as i64);
    match style {
        1 => {
            let nebula = 0.5 + 0.5 * (fu * 4.0 + t * 0.05).sin() * (fv * 3.0 - t * 0.03).cos();
            let base = palette(0x05_020F, 0x1B_0B3A, 0x4A_2380, nebula * (0.6 + 0.4 * fv));
            let star = noise(ui * 16 + vi, 7);
            if star > 0.86 { mix(base, WHITE, 0.45 + 0.55 * (t * 0.6 + star * 50.0).sin().max(0.0)) } else { base }
        }
        2 => {
            let flicker = 0.5 * (noise(ui, vi + t as i64) + noise(ui, vi + 1 + t as i64));
            palette(0x1A_0000, 0xD8_300A, 0xFF_D24A, (fv * 1.1 + 0.45 * flicker - 0.3).clamp(0.0, 1.0))
        }
        3 => mix(hue(fu * 0.25 + fv * 0.35 - t * 0.02), BLACK, 0.15 - 0.15 * (fv * 6.0 - t * 0.5).sin()),
        4 => {
            let base = palette(0xE4_F6FF, 0x7F_C4F0, 0x23_4F8C, fv);
            let flake = noise(ui * 31, (v as f64 - t * 0.3).floor() as i64) > 0.88;
            let shimmer = ((u + 2 * v) as f64 - t * 0.6).rem_euclid(24.0) < 1.5;
            if flake { WHITE } else if shimmer { mix(base, WHITE, 0.5) } else { base }
        }
        5 => {
            let wave = (fv * 5.0 + 0.35 * (fu * 6.0 + t * 0.25).sin() - t * 0.06).rem_euclid(1.0);
            let water = palette(0x04_1E3A, 0x0B_5F8A, 0x2A_B7D6, 0.3 + 0.7 * wave);
            if wave > 0.9 { mix(water, 0xD8_FBFF, 0.8) } else { water }
        }
        6 => {
            let sky = mix(0x02_0814, 0x0A_1F3A, fv);
            let crest = 0.32 + 0.12 * (fu * 3.5 + t * 0.12).sin() + 0.05 * (fu * 9.0 - t * 0.2).sin();
            let glow = (-(fv - crest).powi(2) * 30.0).exp() + 0.6 * (-(fv - crest - 0.22).powi(2) * 18.0).exp();
            let curtain = palette(0x2C_FF9A, 0x1F_D6E0, 0x9B_5CFF, (fu + 0.2 * (t * 0.05).sin()).rem_euclid(1.0));
            let star = noise(ui * 13 + vi, 3) > 0.93 && fv < 0.3 && (t * 0.4 + noise(ui, vi) * 9.0).sin() > 0.0;
            if star { mix(sky, WHITE, 0.8) } else { mix(sky, curtain, glow.min(1.0)) }
        }
        7 => {
            let crest = 5.0 + 1.6 * (fu * 5.0 + t * 0.25).sin() + 0.8 * (fu * 11.0 - t * 0.4).sin();
            let depth = v as f64 - crest;
            if depth < 0.0 {
                let (dx, dy) = (u as f64 - 7.0, v as f64 - 2.5);
                if dx * dx + dy * dy < 3.2 { 0xD8_342C } else { mix(0xF2_E6C8, 0xE8_D4A8, fv) }
            } else if depth < 1.0 || (depth < 2.0 && ((u as f64 + t * 0.3).floor() as i64) % 3 == 0) {
                0xF8_F4EA
            } else {
                let streak = ((v as f64 + u as f64 * 0.5 - t * 0.2).floor() as i64).rem_euclid(4) == 0;
                let water = palette(0x8F_B8E0, 0x3A_6FB5, 0x14_306A, (depth / 9.0).min(1.0));
                if streak { mix(water, 0xF8_F4EA, 0.35) } else { water }
            }
        }
        8 => {
            let base = palette(0x2A_0E1E, 0x4A_1838, 0x7A_2A50, fv);
            let petal = noise(ui + (t * 0.25) as i64, vi - (t * 0.5) as i64);
            let branch = v == 2 && u < 8 || v == 3 && (3..6).contains(&u) || v == 1 && u == 7;
            if branch {
                0x3B_2016
            } else if petal > 0.9 {
                mix(0xFF_B7D5, WHITE, (petal - 0.9) * 6.0)
            } else {
                base
            }
        }
        9 => {
            let cloud = 0.5 * (noise(ui / 2 + (t * 0.1) as i64, vi / 2) + noise(ui, vi + 7));
            let sky = palette(0x0B_0F1C, 0x24_2E45, 0x47_5670, (1.0 - fv) * 0.6 + cloud * 0.4);
            let flash = (t as u64 % 37) < 3;
            let bolt = 4.5 + ((v as f64 * 1.7 + (t as u64 / 37) as f64 * 5.0).sin() * 2.0).round();
            if flash && (u as f64 - bolt).abs() < 0.6 {
                0xE8_F4FF
            } else if flash {
                mix(sky, 0x9F_B8FF, 0.35)
            } else {
                sky
            }
        }
        10 => {
            let speed = 0.4 + noise(ui, 1) * 0.6;
            let head = (t * speed + noise(ui, 2) * 24.0).rem_euclid(24.0);
            let behind = head - v as f64;
            if behind.abs() < 0.5 {
                0xD8_FFD8
            } else if (0.0..9.0).contains(&behind) {
                let glyph = 0.65 + 0.35 * noise(ui * 17 + vi, (t / 4.0) as i64);
                mix(BLACK, 0x1C_FF5C, (1.0 - behind / 9.0) * glyph)
            } else {
                0x00_0A04
            }
        }
        11 => {
            if fv < 0.62 {
                let sky = palette(0x2B_0B4F, 0xC2_2E8C, 0xFF_9A3C, fv / 0.62);
                let (dx, dy) = (u as f64 - 4.5, v as f64 - 6.5);
                let sun = dx * dx + dy * dy < 12.5;
                let gap = sun && v > 6 && ((v as f64 + t * 0.15).floor() as i64) % 3 == 0;
                if sun && !gap { mix(0xFF_E066, 0xFF_5E9A, (v as f64 - 3.0) / 7.0) } else { sky }
            } else {
                let depth = v as f64 - 9.9;
                let row = ((depth * depth * 0.9 + t * 0.25).rem_euclid(2.0)) < 0.35;
                let col = ((u as f64 - 4.5) / (depth + 1.0) * 3.0).rem_euclid(2.0) < 0.3;
                if row || col { 0xFF_3CC8 } else { 0x14_0424 }
            }
        }
        _ => {
            let diamond = (u as f64 - 4.5).abs() + (v as f64 - 7.0).abs();
            let glint = ((u + v) as f64 - t * 0.8).rem_euclid(30.0) < 2.0;
            let color = if u == 0 || u == FACE_W - 1 || v == 0 || v == FACE_H - 1 {
                GOLD
            } else if diamond > 2.5 && diamond <= 3.5 {
                mix(GOLD, WHITE, 0.3 + 0.3 * (t * 0.4).sin())
            } else if diamond <= 2.5 {
                mix(ROYAL, WHITE, 0.15)
            } else {
                mix(ROYAL, BLACK, 0.2 + 0.4 * fv)
            };
            if glint { mix(color, WHITE, 0.4) } else { color }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_style_fills_the_top_half_only() {
        for style in 0..STYLES.len() {
            let canvas = cape(style, 3);
            assert!((0..CAPE_ROWS).all(|y| (0..64).all(|x| canvas.get(x, y) >> 24 == 0xFF)));
            assert!((CAPE_ROWS..64).all(|y| (0..64).all(|x| canvas.get(x, y) == 0)));
        }
    }

    #[test]
    fn every_style_animates() {
        for style in 0..STYLES.len() {
            let frames: Vec<Canvas> = (0..40).map(|f| cape(style, f)).collect();
            assert!(frames.windows(2).any(|w| w[0] != w[1]), "{}", STYLES[style]);
        }
    }

    #[test]
    fn moving_lifts_the_cape_more_than_standing() {
        let total = |swing: [f32; 2]| (0..SEGMENTS).map(|i| bend(swing, 0.0, i)[0]).sum::<f32>();
        assert!(total([1.0, 0.0]) > total([0.0, 0.0]) + 1.0);
        assert_ne!(bend([0.0, 0.0], 0.0, 3), bend([0.0, 0.0], 0.5, 3));
        assert!(bend([0.0, 0.3], 0.0, 7)[2] > 0.0);
    }

    #[test]
    fn joints_are_filled_but_hidden_while_flat() {
        for cubes in [classic(3), tiled(3)] {
            let fillers: Vec<&Cube> = cubes.iter().filter(|c| c.from[1] < SEGMENT_H * 3.0).collect();
            assert!(!fillers.is_empty());
            for f in fillers {
                assert!(f.from[2] > CAPE_Z && f.from[2] + f.size[2] < CAPE_Z + 1.0);
                assert!((f.from[1] - (SEGMENT_H * 3.0 - FILL)).abs() < 1e-6);
            }
        }
        assert!(classic(0).iter().all(|c| c.from[1] >= 0.0));
    }

    #[test]
    fn items_have_unique_keys_and_hang_as_a_chain() {
        let items = items();
        let mut keys: Vec<_> = items.iter().map(|i| i.key.clone()).collect();
        keys.sort();
        keys.dedup();
        assert_eq!(keys.len(), STYLES.len());
        let (lo, hi) = super::super::umbx_flair_model::bounds(&items[0].bones).unwrap();
        assert_eq!((lo, hi), ([-HALF_W, 0.0, CAPE_Z], [HALF_W, 16.0, CAPE_Z + 1.0]));
        let mut depth = 0;
        let mut at = &items[0].bones[0];
        while let Some(next) = at.children.first() {
            depth += 1;
            at = next;
        }
        assert_eq!(depth, SEGMENTS - 1);
        assert_eq!(super::super::umbx_flair_model::layers(&chain(tiled)), 9);
    }
}
