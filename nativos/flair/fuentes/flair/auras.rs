use super::umbx_flair_items::{Effect, Item, Moves, Paint, Slot, Stroke};
use super::umbx_flair_paint::{Canvas, mix, noise, palette};
use std::f64::consts::{PI, TAU};

const WHITE: u32 = 0xFF_FFFF;
const WIDTH: f32 = 2.5;
const RING_WIDTH: f32 = 3.5;
const PLAYER_HEIGHT: f64 = 1.8;
const HALO_LIFT: f64 = 0.35;
const HALO_RADIUS: f64 = 0.32;
const HALO_SEGMENTS: usize = 40;
const STREAKS: usize = 14;
const STREAK_HEIGHT: f64 = 2.2;
const ORBIT_RADIUS: f64 = 0.95;
const ORBIT_TILT: f64 = 1.05;
const ORBIT_SEGMENTS: usize = 48;
const ORBIT_TAIL: usize = 8;
const WING_SPAN: f64 = 1.1;
const WING_STEPS: usize = 8;
const HELIX_STEPS: usize = 40;
const HELIX_RADIUS: f64 = 0.55;
const HELIX_HEIGHT: f64 = 2.1;
const RING_RADIUS: f64 = 0.85;
const RING_LIFT: f64 = 0.05;
const RING_SEGMENTS: usize = 96;
const BOLT_STEPS: usize = 4;
const VORTEX_ARMS: usize = 6;
const VORTEX_STEPS: usize = 16;
const VORTEX_HEIGHT: f64 = 0.9;
const RUNE_RADIUS: f64 = 0.95;
const RUNE_POINTS: usize = 8;
const RUNE_TICKS: usize = 24;
const PETALS: usize = 18;
const FLAMES: usize = 10;
const FLAME_STEPS: usize = 6;
const STARS: usize = 6;
const STAR_RADIUS: f64 = 0.42;

pub fn items() -> Vec<Item> {
    let aura = |key: &str, name: &str, effect: Effect, colors: [u32; 2]| Item::new(key, name, Slot::Aura, Vec::new(), Paint::Still(Canvas::default()), Moves::Still).with_effect(effect, colors);
    vec![
        aura("halo_rings", "Halo Rings", halo, [0xFF_D86B, WHITE]),
        aura("streaks", "Streaks", streaks, [0x9F_E8FF, WHITE]),
        aura("orbits", "Orbits", orbits, [0x9B_6BFF, 0x4A_D8FF]),
        aura("light_wings", "Light Wings", wings, [WHITE, 0xFF_E9A8]),
        aura("helix", "Helix", helix, [0x3C_E8FF, 0xFF_4CD8]),
        aura("rune_circle", "Rune Circle", runes, [0x9B_6BFF, 0x4A_D8FF]),
        aura("sakura_petals", "Sakura Petals", petals, [0xFF_B7D5, WHITE]),
        aura("spirit_flames", "Spirit Flames", flames, [0x3C_8CFF, 0x9F_E8FF]),
        aura("star_crown", "Star Crown", star_crown, [0xFF_D86B, WHITE]),
        aura("fire_ring", "Fire Ring", |t, _, c, out| ring(1, t, c, out), [0xFF_6A00, 0xFF_D84A]),
        aura("lava_ring", "Lava Ring", |t, _, c, out| ring(2, t, c, out), [0xE0_3A00, 0xFF_B030]),
        aura("storm_ring", "Storm Ring", |t, _, c, out| ring(3, t, c, out), [0x4A_C8FF, WHITE]),
        aura("vortex", "Vortex", |t, _, c, out| ring(4, t, c, out), [0x9B_6BFF, WHITE]),
    ]
}

fn line(out: &mut Vec<Stroke>, a: [f64; 3], b: [f64; 3], rgb: u32, alpha: f64, width: f32) {
    out.push(Stroke { a, b, color: ((alpha.clamp(0.0, 1.0) * 255.0).round() as u32) << 24 | (rgb & 0xFF_FFFF), width });
}

fn flat(angle: f64, radius: f64, y: f64) -> [f64; 3] {
    [radius * angle.cos(), y, radius * angle.sin()]
}

fn halo(time: f64, _: f64, [main, light]: [u32; 2], out: &mut Vec<Stroke>) {
    let center = [0.0, PLAYER_HEIGHT + HALO_LIFT + (time * 2.0).sin() * 0.03, 0.0];
    let tilt = (time * 0.9).sin() * 0.06;
    for ring in 0..2 {
        let radius = HALO_RADIUS - ring as f64 * 0.06;
        let point = |i: usize| {
            let angle = TAU * i as f64 / HALO_SEGMENTS as f64 + time * 0.8;
            [center[0] + radius * angle.cos(), center[1] + tilt * angle.sin(), center[2] + radius * angle.sin()]
        };
        for i in 0..HALO_SEGMENTS {
            let angle = TAU * i as f64 / HALO_SEGMENTS as f64;
            let shimmer = 0.55 + 0.45 * (angle * 3.0 + time * 4.0).sin();
            line(out, point(i), point(i + 1), mix(main, light, 0.5 * shimmer), shimmer * if ring == 0 { 1.0 } else { 0.5 }, WIDTH);
        }
    }
}

fn streaks(time: f64, _: f64, [main, light]: [u32; 2], out: &mut Vec<Stroke>) {
    for j in 0..STREAKS {
        let seed = j as f64 * 0.618_034;
        let progress = (time * 0.35 + seed).fract();
        let fade = (PI * progress).sin();
        let angle = time * (0.6 + 0.4 * (seed * 3.0).fract()) + j as f64 * TAU / STREAKS as f64;
        let radius = 0.75 + 0.15 * (time * 1.3 + j as f64).sin();
        let y = progress * STREAK_HEIGHT;
        let length = 0.15 + 0.35 * fade;
        line(out, flat(angle, radius, y), flat(angle + 0.15, radius, y + length), mix(main, light, (seed * 7.0).fract()), 0.85 * fade, WIDTH);
    }
}

fn orbits(time: f64, _: f64, [main, light]: [u32; 2], out: &mut Vec<Stroke>) {
    let center = [0.0, 1.0, 0.0];
    for k in 0..3 {
        let phi = time * 0.5 + k as f64 * TAU / 3.0;
        let u = [phi.cos(), 0.0, phi.sin()];
        let v = [-phi.sin() * ORBIT_TILT.sin(), ORBIT_TILT.cos(), phi.cos() * ORBIT_TILT.sin()];
        let at = |a: f64| -> [f64; 3] { std::array::from_fn(|i| center[i] + ORBIT_RADIUS * (u[i] * a.cos() + v[i] * a.sin())) };
        let color = mix(main, light, k as f64 / 2.0);
        for i in 0..ORBIT_SEGMENTS {
            let (a0, a1) = (TAU * i as f64 / ORBIT_SEGMENTS as f64, TAU * (i + 1) as f64 / ORBIT_SEGMENTS as f64);
            line(out, at(a0), at(a1), color, 0.3, WIDTH * 0.7);
        }
        let head = time * 3.0 + k as f64 * 2.0;
        for j in 0..ORBIT_TAIL {
            let a0 = head - j as f64 * 0.09;
            line(out, at(a0), at(a0 - 0.09), mix(color, WHITE, 0.4), 1.0 - j as f64 / ORBIT_TAIL as f64, WIDTH * (2.0 - 0.18 * j as f32));
        }
    }
}

fn wings(time: f64, yaw: f64, [main, light]: [u32; 2], out: &mut Vec<Stroke>) {
    let yaw = yaw.to_radians();
    let forward = [-yaw.sin(), 0.0, yaw.cos()];
    let right = [-yaw.cos(), 0.0, -yaw.sin()];
    let flap = 0.45 + 0.3 * (time * 3.2).sin();
    let shoulder = [-forward[0] * 0.28, 1.35, -forward[2] * 0.28];
    for side in [-1.0, 1.0] {
        let bone = |t: f64| -> [f64; 3] {
            let reach = WING_SPAN * t;
            let back = reach * flap.sin() * 0.5 + 0.05;
            let lift = 0.45 * t - 0.25 * t * t;
            std::array::from_fn(|i| shoulder[i] + right[i] * side * reach * flap.cos() - forward[i] * back + if i == 1 { lift } else { 0.0 })
        };
        for k in 0..WING_STEPS {
            let (t0, t1) = (k as f64 / WING_STEPS as f64, (k + 1) as f64 / WING_STEPS as f64);
            line(out, bone(t0), bone(t1), mix(main, light, t0), 0.95, WIDTH * 1.2);
            let root = bone(t1);
            let length = 0.75 * (1.0 - 0.55 * t1);
            let sway = 0.12 + 0.08 * (time * 4.0 + k as f64).sin();
            let tip = [root[0] - forward[0] * sway, root[1] - length, root[2] - forward[2] * sway];
            line(out, root, tip, mix(main, light, t1), 0.75 * (1.0 - 0.4 * t1), WIDTH);
        }
    }
}

fn helix_point(time: f64, strand: usize, k: usize) -> [f64; 3] {
    let t = k as f64 / HELIX_STEPS as f64;
    flat(time * 2.2 + t * TAU * 2.0 + strand as f64 * PI, HELIX_RADIUS, t * HELIX_HEIGHT)
}

fn helix(time: f64, _: f64, colors: [u32; 2], out: &mut Vec<Stroke>) {
    for (strand, color) in colors.iter().enumerate() {
        for k in 0..HELIX_STEPS {
            line(out, helix_point(time, strand, k), helix_point(time, strand, k + 1), *color, 0.85, WIDTH);
        }
    }
    for k in (0..=HELIX_STEPS).step_by(4) {
        let pulse = 0.5 + 0.5 * (time * 5.0 - k as f64 * 0.4).sin();
        line(out, helix_point(time, 0, k), helix_point(time, 1, k), mix(colors[0], colors[1], 0.5), 0.2 + 0.4 * pulse, WIDTH * 0.6);
    }
}

fn runes(time: f64, _: f64, [main, light]: [u32; 2], out: &mut Vec<Stroke>) {
    let lift = RING_LIFT + 0.02;
    let pulse = 0.6 + 0.4 * (time * 2.0).sin();
    for (radius, speed, alpha) in [(RUNE_RADIUS, 0.4, 0.9), (RUNE_RADIUS * 0.78, -0.6, 0.7)] {
        for i in 0..RING_SEGMENTS {
            let (a0, a1) = (TAU * i as f64 / RING_SEGMENTS as f64 + time * speed, TAU * (i + 1) as f64 / RING_SEGMENTS as f64 + time * speed);
            line(out, flat(a0, radius, lift), flat(a1, radius, lift), main, alpha * pulse, RING_WIDTH * 0.8);
        }
    }
    let spin = time * 0.4;
    let inner = RUNE_RADIUS * 0.78;
    for k in 0..RUNE_POINTS {
        let (a, b) = (spin + TAU * k as f64 / RUNE_POINTS as f64, spin + TAU * ((k + 3) % RUNE_POINTS) as f64 / RUNE_POINTS as f64);
        line(out, flat(a, inner, lift), flat(b, inner, lift), mix(main, light, 0.4), 0.55 * pulse, WIDTH * 0.8);
    }
    for k in 0..RUNE_TICKS {
        let a = -time * 0.6 + TAU * k as f64 / RUNE_TICKS as f64;
        let long = if k % 3 == 0 { 0.1 } else { 0.05 };
        line(out, flat(a, inner + 0.04, lift), flat(a, inner + 0.04 + long, lift), light, 0.8 * pulse, WIDTH * 0.7);
    }
}

fn petals(time: f64, _: f64, [main, light]: [u32; 2], out: &mut Vec<Stroke>) {
    for k in 0..PETALS {
        let seed = noise(k as i64, 41);
        let phase = (time * 0.18 + seed).fract();
        let angle = seed * TAU + time * 0.5 + phase * 2.0;
        let radius = 0.55 + 0.45 * noise(k as i64, 42);
        let center = flat(angle, radius, 2.4 - phase * 2.5);
        let spin = time * 3.0 + seed * 10.0;
        let tip = |a: f64, r: f64| [center[0] + r * a.cos(), center[1] + r * 0.5 * (a * 2.0).sin(), center[2] + r * a.sin()];
        let fade = (PI * phase).sin();
        let color = mix(main, light, noise(k as i64, 43) * 0.6);
        let corners = [tip(spin, 0.07), tip(spin + PI * 0.5, 0.035), tip(spin + PI, 0.07), tip(spin + PI * 1.5, 0.035)];
        for i in 0..4 {
            line(out, corners[i], corners[(i + 1) % 4], color, fade, WIDTH * 1.4);
        }
    }
}

fn flames(time: f64, _: f64, [main, light]: [u32; 2], out: &mut Vec<Stroke>) {
    for k in 0..FLAMES {
        let base = TAU * k as f64 / FLAMES as f64 + time * 0.3;
        let height = 0.55 + 0.35 * (0.5 + 0.5 * (time * 3.0 + k as f64 * 1.7).sin());
        let point = |j: usize| {
            let f = j as f64 / FLAME_STEPS as f64;
            let sway = 0.24 * (time * 6.0 + f * 5.0 + k as f64).sin() * f;
            flat(base + sway, 0.62 - 0.25 * f, f * height)
        };
        for j in 0..FLAME_STEPS {
            let f = j as f64 / FLAME_STEPS as f64;
            line(out, point(j), point(j + 1), mix(main, light, f), 0.9 * (1.0 - f * 0.7), WIDTH * (1.6 - f as f32));
        }
    }
}

fn star_crown(time: f64, _: f64, [main, light]: [u32; 2], out: &mut Vec<Stroke>) {
    let height = PLAYER_HEIGHT + 0.3 + 0.04 * (time * 2.0).sin();
    for k in 0..STARS {
        let a = time * 0.9 + TAU * k as f64 / STARS as f64;
        let center = flat(a, STAR_RADIUS, height);
        let spin = time * 2.0 + k as f64;
        let size = 0.07 + 0.015 * (time * 4.0 + k as f64).sin();
        let tip = |i: usize| {
            let b = spin + TAU * i as f64 / 5.0;
            [center[0] + size * b.cos(), center[1] + size * b.sin(), center[2]]
        };
        for i in 0..5 {
            line(out, tip(i), tip((i + 2) % 5), mix(main, light, 0.3), 1.0, WIDTH * 0.9);
        }
        line(out, center, flat(a - 0.35, STAR_RADIUS, height), light, 0.35, WIDTH * 0.6);
    }
}

fn ring(style: usize, time: f64, [main, light]: [u32; 2], out: &mut Vec<Stroke>) {
    let dark = mix(main, 0, 0.7);
    let point = |i: usize, r: f64| flat(TAU * i as f64 / RING_SEGMENTS as f64, r, RING_LIFT);
    let radius = RING_RADIUS;
    let frame = (time * 18.0) as i64;
    for i in 0..RING_SEGMENTS {
        let angle = TAU * i as f64 / RING_SEGMENTS as f64;
        match style {
            1 => {
                let flame = (0.5 + 0.35 * (angle * 7.0 + time * 5.0).sin() + 0.15 * (angle * 17.0 - time * 11.0).sin()).clamp(0.0, 1.0);
                line(out, point(i, radius), point(i + 1, radius), palette(dark, main, light, flame), 0.75 + 0.25 * flame, RING_WIDTH);
                if i % 4 == 0 {
                    let height = 0.1 + 0.45 * flame * (0.5 + 0.5 * (time * 9.0 + i as f64).sin());
                    let base = point(i, radius);
                    let mid = [base[0], base[1] + height * 0.55, base[2]];
                    let tip = [base[0], base[1] + height, base[2]];
                    line(out, base, mid, main, 0.85, RING_WIDTH * 0.8);
                    line(out, mid, tip, light, 0.55, RING_WIDTH * 0.5);
                }
            }
            2 => {
                let wave = 0.5 + 0.5 * (angle * 4.0 - time * 1.6).sin();
                let crack = ((angle * 23.0 + time * 0.7).sin() * (time * 2.3 + angle * 5.0).cos()).max(0.0).powi(6);
                line(out, point(i, radius), point(i + 1, radius), mix(palette(dark, main, main, wave), light, crack), 1.0, RING_WIDTH * 1.3);
                line(out, point(i, radius - 0.07), point(i + 1, radius - 0.07), dark, 0.6, RING_WIDTH * 0.8);
            }
            3 => {
                let jag = |k: usize| radius + (noise((k % RING_SEGMENTS) as i64, frame) - 0.5) * 0.16;
                let flicker = 0.6 + 0.4 * noise(i as i64 / 8, frame);
                line(out, point(i, radius), point(i + 1, radius), main, 0.3, RING_WIDTH * 1.8);
                line(out, point(i, jag(i)), point(i + 1, jag(i + 1)), mix(main, light, noise(i as i64, frame + 1)), flicker, RING_WIDTH * 0.8);
                if i % 16 == 0 && noise(i as i64, frame / 3) > 0.55 {
                    let mut at = point(i, radius);
                    for k in 0..BOLT_STEPS {
                        let step = frame + k as i64;
                        let next = [at[0] + (noise(i as i64, step) - 0.5) * 0.25, at[1] + 0.18, at[2] + (noise(i as i64 + 1, step) - 0.5) * 0.25];
                        line(out, at, next, light, 1.0 - 0.2 * k as f64, RING_WIDTH * 0.6);
                        at = next;
                    }
                }
            }
            _ => {
                let pulse = 0.5 + 0.5 * (angle * 6.0 - time * 6.0).sin();
                line(out, point(i, radius), point(i + 1, radius), mix(main, light, pulse * 0.5), 0.6 + 0.4 * pulse, RING_WIDTH);
            }
        }
    }
    if style == 4 {
        for arm in 0..VORTEX_ARMS {
            let at = |t: f64| flat(arm as f64 * TAU / VORTEX_ARMS as f64 + time * 1.8 + t * 3.0, radius * (1.0 - 0.75 * t), RING_LIFT + VORTEX_HEIGHT * t * t);
            for k in 0..VORTEX_STEPS {
                let (t0, t1) = (k as f64 / VORTEX_STEPS as f64, (k + 1) as f64 / VORTEX_STEPS as f64);
                line(out, at(t0), at(t1), mix(main, light, t0), 1.0 - t0, RING_WIDTH * (1.0 - 0.6 * t0 as f32));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_aura_draws_near_the_player_and_moves() {
        for item in items() {
            let (mut a, mut b) = (Vec::new(), Vec::new());
            item.stroke(1.0, 30.0, 0, &mut a);
            item.stroke(1.7, 30.0, 0, &mut b);
            assert!(!a.is_empty(), "{}", item.key);
            assert_ne!(a, b, "{}", item.key);
            for s in &a {
                for p in [s.a, s.b] {
                    assert!(p[0].abs() < 2.0 && p[2].abs() < 2.0 && (-0.1..3.0).contains(&p[1]), "{} {p:?}", item.key);
                }
            }
        }
    }

    #[test]
    fn themes_recolor_an_aura() {
        let aura = &items()[0];
        let (mut own, mut themed) = (Vec::new(), Vec::new());
        aura.stroke(1.0, 0.0, 0, &mut own);
        aura.stroke(1.0, 0.0, 3, &mut themed);
        assert_ne!(own[0].color, themed[0].color);
        assert_eq!(own[0].a, themed[0].a);
    }
}
